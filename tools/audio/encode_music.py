#!/usr/bin/env python3
"""Encode RenderMidi WAV output. Requires ffmpeg on PATH or imageio-ffmpeg."""
import argparse, hashlib, json, shutil, subprocess, wave
from pathlib import Path
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('wav_directory', type=Path)
a = p.parse_args()
root = Path(__file__).resolve().parents[2]
ffmpeg = shutil.which('ffmpeg')
if not ffmpeg:
    import imageio_ffmpeg
    ffmpeg = imageio_ffmpeg.get_ffmpeg_exe()
out = root / 'assets/music'
out.mkdir(exist_ok=True)
clips = {}
durations = []
for i in range(12):
    wav = a.wav_directory / f'{i}.wav'
    ogg = out / f'{i}.ogg'
    with wave.open(str(wav)) as w:
        duration = w.getnframes() / w.getframerate()
    subprocess.run([ffmpeg, '-v', 'error', '-y', '-i', str(wav), '-c:a', 'libvorbis', '-q:a', '4', str(ogg)], check=True)
    clips[str(i)] = dict(duration_seconds=duration,
        midi_sha256=hashlib.sha256((root / f'assets/sounds/{i}.mid').read_bytes()).hexdigest(),
        ogg_sha256=hashlib.sha256(ogg.read_bytes()).hexdigest())
    durations.append(duration)
(out / 'provenance.json').write_text(json.dumps(dict(renderer='Temurin 21 Gervill software General MIDI, 22050 Hz stereo, sinc interpolation, reverb/chorus; 2-second release tail; Vorbis quality 4', clips=clips), indent=2) + '\n')
(out / 'catalog.rs').write_text('const MUSIC: [&[u8]; 12] = [\n' + ''.join(f'    include_bytes!("{i}.ogg"),\n' for i in range(12)) + '];\nconst MUSIC_DURATION: [f64; 12] = ' + repr(durations) + ';\n')
