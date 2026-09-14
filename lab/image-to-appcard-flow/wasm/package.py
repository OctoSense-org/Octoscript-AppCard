"""Package the pinned Makepad browser output and retain exact input evidence."""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import struct

PACKAGE = "octosense-wizard"
FALLBACK_FONTS = {"IBMPlexSans-Text.ttf", "NotoSans-Regular.ttf", "jetbrains_mono_variable.ttf", "NotoColorEmoji.ttf"}
FOCUS_PATCHES = [
    ("embedded-blur-does-not-recapture-focus",
     "ta.addEventListener('blur',e=>{\nthis.focus_keyboard_input();\n})",
     "ta.addEventListener('blur',e=>{\nif(window.self===window.top)this.focus_keyboard_input();\n})"),
    ("initial-focus-prevents-parent-scroll", "ta.focus();", "ta.focus({preventScroll:true});"),
    ("keyboard-focus-prevents-parent-scroll", "this.text_area.focus();", "this.text_area.focus({preventScroll:true});"),
]


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def font_notices(path: Path) -> str:
    """Read sfnt name IDs 0/13/14 without requiring a Python font package."""
    data = path.read_bytes()
    count = struct.unpack_from(">H", data, 4)[0]
    for offset in range(12, 12 + count * 16, 16):
        tag, _, start, length = struct.unpack_from(">4sIII", data, offset)
        if tag != b"name":
            continue
        if start + length > len(data):
            raise RuntimeError(f"Invalid font name table: {path.name}")
        _, records, strings = struct.unpack_from(">HHH", data, start)
        values = set()
        for pos in range(start + 6, start + 6 + records * 12, 12):
            platform, encoding, language, name_id, size, string_offset = struct.unpack_from(">HHHHHH", data, pos)
            if name_id not in (0, 13, 14):
                continue
            raw = data[start + strings + string_offset:start + strings + string_offset + size]
            codec = "utf-16-be" if platform in (0, 3) else "mac_roman"
            values.add(raw.decode(codec))
        if not values:
            raise RuntimeError(f"Font has no retained license/copyright metadata: {path.name}")
        return path.name + "\n" + "\n".join(sorted(values))
    raise RuntimeError(f"Font name table not found: {path.name}")


def project_source(receipt: dict, project: Path, path: Path) -> None:
    if not path.resolve().is_relative_to(project):
        raise RuntimeError(f"Project input resolves outside project: {path}")
    receipt["sources"][str(path.relative_to(project))] = sha(path)


def copy_artwork(project: Path, dist: Path, receipt: dict) -> None:
    cards = {}
    for directory in (project / "cards", project / "service-cards"):
        if not directory.exists():
            continue
        contracts = []
        for current, directories, files in os.walk(directory):
            if "contract.json" in files:
                contracts.append(Path(current) / "contract.json")
                # A card's rounds/evidence are immutable history, not current exports.
                directories.clear()
            else:
                directories[:] = sorted(name for name in directories if not name.startswith("."))
        for contract_path in sorted(contracts):
            contract = json.loads(contract_path.read_text())
            card_id = contract.get("id")
            if not isinstance(card_id, str) or not re.fullmatch(r"[A-Za-z0-9_.-]+", card_id) or card_id in (".", ".."):
                raise RuntimeError(f"Invalid card ID in {contract_path}")
            card = contract_path.parent
            project_source(receipt, project, contract_path)
            for name in ("page.card", "page.data.json", "mapping.json", "kit/native/light/kit.json", "kit/native/dark/kit.json"):
                source = card / name
                if source.is_file():
                    project_source(receipt, project, source)
            entries = {}
            if (card / "assets").is_dir():
                for source in sorted((card / "assets").rglob("*")):
                    if not source.is_file():
                        continue
                    # The reviewed browser bridge permits exactly <card>/assets/<file>.
                    relative = source.relative_to(card / "assets")
                    if len(relative.parts) != 1 or not re.fullmatch(r"[A-Za-z0-9_.-]+", relative.name):
                        raise RuntimeError(f"Artwork path is outside the browser asset contract: {source}")
                    project_source(receipt, project, source)
                    entries[str(relative)] = sha(source)
                    destination = dist / "card-assets" / card_id / "assets" / relative
                    if destination.exists() and sha(destination) != sha(source):
                        raise RuntimeError(f"Conflicting artwork for shared card ID: {card_id}/{relative}")
                    destination.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copy2(source, destination)
            if card_id in cards and cards[card_id]["assets"] != entries:
                raise RuntimeError(f"Duplicate card ID has different artwork: {card_id}")
            cards[card_id] = {"contract": str(contract_path.relative_to(project)), "assets": entries}
    if not cards:
        raise RuntimeError("No contract.json found under project cards/ or service-cards/")
    receipt["cards"] = cards


def package_runtime(host: Path, dist: Path, project: Path, dependencies: dict, receipt: dict) -> None:
    built = host / f"target/makepad-wasm-app/release/{PACKAGE}"
    source_html = (built / "index.html").read_text()
    matches = re.findall(r"'\./(octosense-wizard[^']*\.wasm)'", source_html)
    if len(matches) != 1 or Path(matches[0]).name != matches[0]:
        raise RuntimeError("Generated Makepad WASM filename requires review")
    shutil.copytree(built, dist, dirs_exist_ok=True)
    shutil.copy2(built / matches[0], dist / f"{PACKAGE}.wasm")
    for path in dist.glob(f"{PACKAGE}.*.wasm"):
        path.unlink()
    for path in list(dist.rglob("*")):
        if path.is_file() and path.suffix.lower() in {".ttf", ".otf", ".woff", ".woff2"}:
            if path.parent != dist / "makepad_widgets/resources" or path.name not in FALLBACK_FONTS:
                path.unlink()
    for path in sorted(dist.rglob("*"), reverse=True):
        if path.is_dir() and not any(path.iterdir()):
            path.rmdir()
    web = dist / "makepad_platform/web.js"
    before = sha(web)
    source = web.read_text()
    for name, old, new in FOCUS_PATCHES:
        if source.count(old) != 1:
            raise RuntimeError(f"Generated Makepad focus patch requires review: {name}")
        source = source.replace(old, new)
    web.write_text(source)
    receipt["generated_runtime_patches"] = {"makepad_platform/web.js": {
        "source_sha256": before, "packaged_sha256": sha(web), "operations": [row[0] for row in FOCUS_PATCHES]}}
    for name in ("index.html", "bridge.mjs"):
        shutil.copy2(host / name, dist / name)
    receipt["sources"] = dict(receipt["project_inputs"])
    copy_artwork(project, dist, receipt)
    licenses = dist / "licenses"
    licenses.mkdir(exist_ok=True)
    shutil.copy2(dependencies["makepad"] / "LICENSE", licenses / "Makepad-MIT.txt")
    shutil.copy2(dependencies["splash"] / "LICENSE", licenses / "Splash-MIT.txt")
    shutil.copy2(project / "fonts/OFL.txt", licenses / "NotoSansSC-OFL.txt")
    notices = [font_notices(dependencies["makepad"] / "widgets/resources" / name) for name in sorted(FALLBACK_FONTS)]
    (licenses / "Bundled-fonts.txt").write_text("\n\n".join(notices) + "\n\n" + (project / "fonts/OFL.txt").read_text())
    (dist / "THIRD_PARTY_NOTICES.md").write_text(
        "# Third-party notices\n\n"
        "Makepad widgets, renderer and browser loader use the pinned Makepad source. "
        "Its MIT license and copyright are in `licenses/Makepad-MIT.txt`.\n\n"
        "Splash / splash-ui-l0 use the pinned Splash source; its distributed MIT text is in `licenses/Splash-MIT.txt`. "
        "Splash-Makepad declares MIT OR Apache-2.0 in its README License section; this package selects MIT.\n\n"
        "The complete Noto Sans SC Regular font is embedded in WASM. Its SIL OFL is in `licenses/NotoSansSC-OFL.txt`. "
        "Fallback font metadata and OFL text are retained in `licenses/Bundled-fonts.txt`.\n\n"
        "Card artwork comes from the supplied project; its file hashes and source paths are recorded in `build.json`. "
        "No card text or controls have been replaced by bitmap screenshots.\n")
    wasm_sha = sha(dist / f"{PACKAGE}.wasm")
    for name, old, new in (
        ("index.html", 'src="./bridge.mjs"', f'src="./bridge.mjs?v={wasm_sha}"'),
        ("bridge.mjs", f"'./{PACKAGE}.wasm'", f"'./{PACKAGE}.wasm?v={wasm_sha}'"),
    ):
        path = dist / name
        text = path.read_text()
        if text.count(old) != 1:
            raise RuntimeError(f"Cache version stamping requires review: {name}")
        path.write_text(text.replace(old, new))
    receipt["wasm_sha256"] = receipt["cache_version"] = wasm_sha
    receipt["embedded_fonts"] = {"NotoSansSC-Regular.ttf": sha(project / "fonts/NotoSansSC-Regular.ttf")}
    # Re-check project sources after copying to reject changes during packaging.
    for name, expected in receipt["sources"].items():
        if sha(project / name) != expected:
            raise RuntimeError(f"Project source changed while packaging: {name}")
    receipt["files"] = {str(path.relative_to(dist)): sha(path) for path in sorted(dist.rglob("*"))
                        if path.is_file() and path.name != "build.json"}
