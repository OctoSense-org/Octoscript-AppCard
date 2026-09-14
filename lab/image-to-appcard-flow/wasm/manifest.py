"""Generate the isolated browser host's Cargo manifest from dependency paths."""
from pathlib import Path
import json


PACKAGE = "octosense-wizard"


def cargo_manifest(dependencies: dict[str, Path]) -> str:
    # JSON quoted strings are valid TOML basic strings, including Windows paths.
    crates = {
        "makepad-widgets": dependencies["makepad"] / "widgets",
        "splash-widgets": dependencies["splash-makepad"] / "crates/splash-widgets",
        "splash-render": dependencies["splash-makepad"] / "crates/splash-render",
        "splash-makepad": dependencies["splash-makepad"] / "crates/splash-makepad",
        "splash-ui-l0": dependencies["splash"] / "crates/splash-ui-l0",
    }
    paths = "\n".join(f"{name} = {{ path = {json.dumps(str(path.resolve()))} }}" for name, path in crates.items())
    return f'''[package]
name = "{PACKAGE}"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
serde_json = {{ version = "1", features = ["float_roundtrip"] }}
{paths}

[profile.release]
opt-level = 3
lto = false
codegen-units = 1

[[bin]]
name = "{PACKAGE}"
path = "src/main.rs"
'''
