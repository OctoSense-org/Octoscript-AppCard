#!/usr/bin/env python3
"""OpenHarmony instrument: makepad-shaped inspection artifacts from ArkUI.

Makepad's Studio inspection (WidgetTreeDump, WidgetQuery, WidgetSnapshot,
Screenshot) only sees widgets makepad draws itself. Octoscript-OH renders a
card as native ArkUI components, so the same three artifacts come from the OS
instead: ``uitest dumpLayout`` for the component tree (type, id, bounds,
text) and ``uitest screenCap`` for pixels, both over hdc. This module maps
them into the exact shapes the lab already consumes, so a round captured from
a HarmonyOS phone has the same ``tree.json``, ``snapshot.json``,
``queries.json``, ``screenshot.json`` and ``native.png`` as a Studio round.

The component id that joins a source element to a mounted node is the ArkUI
``id`` attribute; Octoscript-OH sets it from the card node's ``id`` (see
``octoscript-oh-arkui/src/dsl.rs``). Bounds are converted from physical pixels
to the logical pixels every other artifact uses, dividing by the display
density (3.25 on the Mate 70 Air, 406 logical px across).

Usage:
    python3 lab/core/ohos_instrument.py --device 5ZGYD25B13020968 \\
        --bundle com.example.myapplication --out /tmp/round [--ids a,b] [--density 3.25]
"""
import argparse
import json
import os
import pathlib
import subprocess
import time

DEFAULT_DENSITY = 3.25


def hdc_path():
    override = os.environ.get("HDC")
    if override and os.path.isfile(override):
        return override
    default = "/Applications/DevEco-Studio.app/Contents/sdk/default/openharmony/toolchains/hdc"
    if os.path.isfile(default):
        return default
    raise RuntimeError("hdc not found: set HDC or source ~/ohos-sdk/env-deveco.sh")


def hdc(device, *args, check=True):
    result = subprocess.run([hdc_path(), "-t", device, *args], capture_output=True, text=True)
    output = (result.stdout + result.stderr).replace("\r", "")
    if check and (result.returncode != 0 or "[Fail]" in output):
        raise RuntimeError("hdc %s failed: %s" % (" ".join(args), output.strip()))
    return output


def dump_layout(device, bundle=None, remote="/data/local/tmp/octoscript-layout.json", local=None):
    """The ArkUI component tree of the target window, parsed."""
    args = ["shell", "uitest", "dumpLayout", "-p", remote]
    if bundle:
        args += ["-b", bundle]
    hdc(device, *args)
    local = pathlib.Path(local or pathlib.Path(os.environ.get("TMPDIR", "/tmp")) / "octoscript-layout.json")
    hdc(device, "file", "recv", remote, str(local))
    return json.loads(local.read_text())


def screen_cap(device, out, remote="/data/local/tmp/octoscript-screen.png"):
    """The screen as a PNG, through the same ArkUI test tool."""
    hdc(device, "shell", "uitest", "screenCap", "-p", remote)
    hdc(device, "file", "recv", remote, str(out))
    return out


def parse_bounds(text):
    """'[x0,y0][x1,y1]' -> (x0, y0, x1, y1) in physical pixels."""
    parts = text.replace("[", " ").replace("]", " ").replace(",", " ").split()
    if len(parts) != 4:
        return None
    return tuple(int(float(p)) for p in parts)


def flatten(tree, density):
    """Depth-first nodes as (index, parent, id, type, x, y, w, h, text, extra)."""
    nodes = []

    def walk(node, parent):
        attrs = node.get("attributes", {})
        index = len(nodes)
        bounds = parse_bounds(attrs.get("bounds", "")) or (0, 0, 0, 0)
        x0, y0, x1, y1 = (v / density for v in bounds)
        nodes.append({
            "index": index,
            "parent": parent,
            "id": attrs.get("id") or "",
            "type": attrs.get("type") or "?",
            "x": x0, "y": y0, "width": x1 - x0, "height": y1 - y0,
            "text": attrs.get("text") or "",
            "enabled": attrs.get("enabled", "true") != "false",
            "visible": attrs.get("visible", "true") != "false",
            "checked": attrs.get("checked"),
            "selected": attrs.get("selected"),
        })
        for child in node.get("children", []):
            walk(child, index)

    walk(tree, -1)
    return nodes


def fmt(value):
    """Integers stay integers; makepad's dump prints whole logical pixels."""
    return str(int(round(value)))


def tree_dump(nodes, build_id):
    """WidgetTreeDump: the 'W3 <count>' header, then 'index parent id Type x y w h' per node."""
    lines = ["W3 %d" % len(nodes)]
    for n in nodes:
        lines.append(" ".join([str(n["index"]), str(n["parent"]), n["id"] or "-", n["type"],
                               fmt(n["x"]), fmt(n["y"]), fmt(n["width"]), fmt(n["height"])]))
    return {"query_id": [0], "build_id": [build_id], "dump": "\n".join(lines)}


def snapshot(nodes, build_id):
    """WidgetSnapshot: one record per node with the visibility/enabled state and geometry."""
    widgets = []
    for n in nodes:
        record = {"id": n["id"] or "-", "widget_type": n["type"], "window_id": "", "window_index": 0,
                  "visible": n["visible"], "enabled": n["enabled"],
                  "x": n["x"], "y": n["y"], "width": n["width"], "height": n["height"]}
        if n["text"]:
            record["text"] = n["text"]
        if n["checked"] is not None:
            record["checked"] = n["checked"] == "true"
        if n["selected"] is not None:
            record["selected"] = n["selected"]
        widgets.append(record)
    return {"query_id": [0], "build_id": [build_id], "widgets": widgets}


def query(nodes, wanted, build_id):
    """WidgetQuery for 'id:<name>': every node carrying that id, as 'index id Type x y w h'."""
    rects = [" ".join([str(n["index"]), n["id"], n["type"], fmt(n["x"]), fmt(n["y"]),
                       fmt(n["width"]), fmt(n["height"])]) for n in nodes if n["id"] == wanted]
    return {"query_id": [0], "build_id": [build_id], "query": "id:" + wanted, "rects": rects}


def capture(device, bundle, out, ids=None, density=DEFAULT_DENSITY, build_id=0):
    out = pathlib.Path(out)
    out.mkdir(parents=True, exist_ok=True)
    tree = dump_layout(device, bundle, local=out / "arkui-layout.json")
    nodes = flatten(tree, density)
    (out / "tree.json").write_text(json.dumps(tree_dump(nodes, build_id), indent=1))
    (out / "snapshot.json").write_text(json.dumps(snapshot(nodes, build_id), indent=1))
    wanted = list(ids) if ids else sorted({n["id"] for n in nodes if n["id"]})
    (out / "queries.json").write_text(json.dumps({i: query(nodes, i, build_id) for i in wanted}, indent=1))
    png = screen_cap(device, out / "native.png")
    (out / "screenshot.json").write_text(json.dumps(
        {"query_id": [0], "build_id": [build_id], "kind_id": 0, "path": str(png),
         "instrument": "arkui", "device": device, "bundle": bundle, "density": density,
         "captured_at": time.time()}, indent=1))
    return nodes


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--device", required=True)
    p.add_argument("--bundle", help="target window's bundle; omit for the whole screen")
    p.add_argument("--out", required=True)
    p.add_argument("--ids", help="comma-separated ids to query; default: every id present")
    p.add_argument("--density", type=float, default=DEFAULT_DENSITY)
    p.add_argument("--build-id", type=int, default=0)
    a = p.parse_args()
    nodes = capture(a.device, a.bundle, a.out, a.ids.split(",") if a.ids else None, a.density, a.build_id)
    with_ids = sum(1 for n in nodes if n["id"])
    print(json.dumps({"nodes": len(nodes), "with_id": with_ids, "out": str(a.out),
                      "types": sorted({n["type"] for n in nodes})}))


if __name__ == "__main__":
    main()
