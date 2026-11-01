#!/usr/bin/env python3
"""Stage the current Cargo release as a launchable macOS app; consume cargo metadata."""
import json
import plistlib
import shutil
import sys
from pathlib import Path


def package(metadata):
    root = Path(__file__).resolve().parent.parent
    project = next(p for p in metadata['packages'] if Path(p['manifest_path']).resolve().parent == root)
    executable = Path(metadata['target_directory']) / 'release' / 'playman-rust'
    if not executable.is_file():
        raise SystemExit('Release executable missing; run make mac-app to build it first.')
    app = root / 'dist' / 'Playman Rust.app'
    contents = app / 'Contents'
    binaries = contents / 'MacOS'
    binaries.mkdir(parents=True, exist_ok=True)
    shutil.copy2(executable, binaries / 'playman-rust')
    info = {
        'CFBundleExecutable': 'playman-rust',
        'CFBundleIdentifier': 'local.playman.rust',
        'CFBundleName': 'Playman Rust',
        'CFBundleDisplayName': 'Playman Rust',
        'CFBundlePackageType': 'APPL',
        'CFBundleVersion': project['version'],
        'CFBundleShortVersionString': project['version'],
        'NSHighResolutionCapable': True,
    }
    (contents / 'Info.plist').write_bytes(plistlib.dumps(info))
    # Preserve direct-launch records on the first app launch, without overwriting
    # records subsequently earned in the bundle. Repackaging retains that file.
    records = executable.parent / 'playman-records.txt'
    bundled_records = binaries / records.name
    if records.is_file() and not bundled_records.exists():
        shutil.copy2(records, bundled_records)
    print(f'Native app staged in {app}')


if __name__ == '__main__':
    package(json.load(sys.stdin))
