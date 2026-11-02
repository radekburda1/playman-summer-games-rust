#!/usr/bin/env python3
"""Audit JAR sound bytes, shipped music provenance and offline-render timing.

This is an asset audit, not a recording of browser/Java speaker output.
"""
import argparse
import array
import csv
import hashlib
import json
import math
import struct
import sys
import wave
import zipfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def audit_resources(jar):
    resources = []
    with zipfile.ZipFile(jar) as archive:
        info, sounds = archive.read("i"), archive.read("s")
        integers = struct.unpack_from(">324i", info, 938 + 1778 * 2)
        for resource in range(27):
            extension = "mid" if info[907 + resource] == 1 else "wav"
            offset, length = integers[3 + resource], integers[65 + resource]
            original = sounds[offset:offset + length]
            extracted = ROOT / f"assets/sounds/{resource}.{extension}"
            resources.append({
                "resource": resource,
                "kind": extension,
                "sha256": sha256(original),
                "matches_jar": original == extracted.read_bytes(),
            })
    return resources


def render_timing(path):
    with wave.open(str(path)) as wav:
        frames, rate, channels = wav.getnframes(), wav.getframerate(), wav.getnchannels()
        if wav.getsampwidth() != 2:
            raise ValueError("Expected a 16-bit source render")

        # Scan the final 3 seconds in 20 ms RMS windows at -60 dBFS.
        start = max(0, frames - 3 * rate)
        wav.setpos(start)
        samples = array.array("h", wav.readframes(frames - start))
        if sys.byteorder != "little":
            samples.byteswap()
        block = max(1, rate // 50) * channels
        last_audible = start / rate
        for offset in range(0, len(samples), block):
            window = samples[offset:offset + block]
            rms = math.sqrt(sum(value * value for value in window) / len(window)) / 32768
            if rms >= 0.001:
                last_audible = (start + (offset + len(window)) / channels) / rate
        duration = frames / rate
    return {
        "render_seconds": duration,
        "trailing_below_minus_60_db_seconds": max(0, duration - last_audible),
        "trailing_is_lower_bound": last_audible == start / rate,
    }


def audit_music(rendered_wavs, midi_csv, resources):
    provenance = json.loads((ROOT / "assets/music/provenance.json").read_text())
    with midi_csv.open() as stream:
        midi = {int(row["resource"]): row for row in csv.DictReader(stream)}
    clips = []
    for resource in range(12):
        timing = render_timing(rendered_wavs / f"{resource}.wav")
        encoded = (ROOT / f"assets/music/{resource}.ogg").read_bytes()
        expected = provenance["clips"][str(resource)]
        sequence = float(midi[resource]["sequence_seconds"])
        clips.append({
            "resource": resource,
            "midi_sequence_seconds": sequence,
            **timing,
            "render_minus_sequence_seconds": timing["render_seconds"] - sequence,
            "ogg_matches_provenance": sha256(encoded) == expected["ogg_sha256"],
            "midi_matches_provenance": resources[resource]["sha256"] == expected["midi_sha256"],
            "programs": midi[resource]["program_changes"],
            "note_on_events": int(midi[resource]["note_on_events"]),
        })
    return clips


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("jar", type=Path)
    parser.add_argument("rendered_wavs", type=Path)
    parser.add_argument("midi_csv", type=Path, help="MidiAudit output")
    parser.add_argument("output", type=Path)
    args = parser.parse_args()

    resources = audit_resources(args.jar)
    music = audit_music(args.rendered_wavs, args.midi_csv, resources)
    result = {
        "jar_sha256": sha256(args.jar.read_bytes()),
        "evidence": (
            "Original JAR bytes, MIDI metadata, existing Gervill source renders "
            "and shipped OGG hashes; no live audio capture"
        ),
        "resources": resources,
        "music": music,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print("JAR resource matches:", sum(r["matches_jar"] for r in resources), "/ 27")
    print("OGG provenance matches:", sum(r["ogg_matches_provenance"] for r in music), "/ 12")
    for clip in music:
        bound = ">=" if clip["trailing_is_lower_bound"] else ""
        print(
            f"MIDI {clip['resource']:2}: source {clip['midi_sequence_seconds']:.3f}s, "
            f"render {clip['render_seconds']:.3f}s, tail below -60dBFS "
            f"{bound}{clip['trailing_below_minus_60_db_seconds']:.3f}s"
        )
    if not all(r["matches_jar"] for r in resources) or not all(
        r["ogg_matches_provenance"] and r["midi_matches_provenance"] for r in music
    ):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
