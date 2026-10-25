#!/usr/bin/env python3
"""Consume cargo metadata and stage the WASM file with its matching JS bundle."""
import json,shutil,sys
from pathlib import Path
metadata=json.load(sys.stdin)
root=Path(__file__).resolve().parent.parent
macroquad=next(p for p in metadata['packages'] if p['name']=='macroquad')
shutil.copy(Path(macroquad['manifest_path']).parent/'js/mq_js_bundle.js',root/'web/mq_js_bundle.js')
shutil.copy(Path(metadata['target_directory'])/'wasm32-unknown-unknown/release/playman-rust.wasm',root/'web/game.wasm')
print('Browser build staged in',root/'web')
