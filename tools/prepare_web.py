#!/usr/bin/env python3
"""Stage WASM and matching JS with content-versioned asset URLs."""
import hashlib
import json
import re
import shutil
import sys
from pathlib import Path


def asset_version(data):
    return hashlib.sha256(data).hexdigest()[:16]


def stage(metadata):
    root = Path(__file__).resolve().parent.parent
    web = root / "web"
    macroquad = next(p for p in metadata["packages"] if p["name"] == "macroquad")
    js = (Path(macroquad["manifest_path"]).parent / "js/mq_js_bundle.js").read_text()

    # Macroquad 0.4.16's quad_net plugin assigns an undeclared name in strict mode.
    # Fail visibly if a dependency update changes the code this workaround targets.
    original = "function i(){}register_plugin=function"
    if js.count(original) != 1:
        raise SystemExit("Expected Macroquad 0.4.16 quad_net registration; review bundle patch")
    js = js.replace(original, "function i(){}var register_plugin;register_plugin=function", 1)
    wasm = (
        Path(metadata["target_directory"])
        / "wasm32-unknown-unknown/release/playman-rust.wasm"
    )
    versions = {
        "game.wasm": asset_version(wasm.read_bytes()),
        "mq_js_bundle.js": asset_version(js.encode()),
        "records.js": asset_version((web / "records.js").read_bytes()),
    }

    index = web / "index.html"
    original_html = index.read_text()
    html, count = re.subn(
        r"load\('game\.wasm(?:\?[^']*)?'\)",
        f"load('game.wasm?v={versions['game.wasm']}')",
        original_html,
    )
    if count != 1:
        raise SystemExit("Expected one game.wasm load call in web/index.html")
    for name in ("mq_js_bundle.js", "records.js"):
        html, count = re.subn(
            r'src="' + re.escape(name) + r'(?:\?[^" ]*)?"',
            f'src="{name}?v={versions[name]}"',
            html,
        )
        if count != 1:
            raise SystemExit(f"Expected one {name} script in web/index.html")

    # Validate all expected inputs before replacing any staged outputs.
    (web / "mq_js_bundle.js").write_text(js)
    shutil.copy(wasm, web / "game.wasm")
    if html != original_html:
        index.write_text(html)
    print("Browser build staged in", web)


if __name__ == "__main__":
    stage(json.load(sys.stdin))
