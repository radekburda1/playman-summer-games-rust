# Original MIDI conversion

The shipped OGG files are offline renders of the original twelve MIDI resources.
The game needs neither Java nor a MIDI synthesizer at runtime. Gervill's General
MIDI instruments differ from handset synthesizers; this preserves notes and tempo,
not a device-specific instrument bank.

To regenerate, compile `RenderMidi.java` with Java 21 (or the reference runner's
ECJ), then run:

```sh
java --add-exports java.desktop/com.sun.media.sound=ALL-UNNAMED \
  -cp /path/to/compiled/classes RenderMidi assets/sounds /tmp/playman-midi
python3 tools/audio/encode_music.py /tmp/playman-midi
```

The encoder needs ffmpeg on PATH or the Python `imageio-ffmpeg` package. It records
source/output hashes and durations in `assets/music/provenance.json`. WAV effects
remain original resources. Two seconds of release tail are retained; looping can
therefore have a short pause, and cue mixing/timing still needs audible checking.

## Cue and asset parity audit

`AudioOracle.java` runs the unmodified JAR in an isolated FreeJ2ME process, then
replaces its Player objects with observers of `start()` calls. It does not record
speaker output or use the live reference viewer. Selected original Dash and Long
Jump states produce `original-cues.csv`; base-mode good/poor/record result queues
produce `result-cues.csv`. Both fixtures live in `tests/fixtures/audio/`, with
setup and hashes in `provenance.json`. Do not regenerate expected values just
to make a changed implementation pass.

With Java 21, ECJ and FreeJ2ME available through the sibling reference runner:

```sh
PLAYMAN_JAVA=/path/to/java tools/audio/run_oracle.sh /path/to/game.jar /tmp/audio-oracle
cargo test --locked --test audio_parity
# After make web-build:
node tests/web_bundle.cjs
python3 tools/audio/audit_resources.py /path/to/game.jar /path/to/rendered-wavs /tmp/audio-oracle/midi.csv /tmp/audio-audit.json
```

`REFERENCE_DIR` can point to another reference-runner checkout. The runner writes
only to its explicit output directory and uses a fresh emulator working directory.
`MidiAudit` reads MIDI duration, instrument-program changes and note counts without
opening a synthesizer. `audit_resources.py` verifies the JAR bytes and shipped OGG
provenance, and measures the final three seconds of the existing source renders in
20 ms RMS windows at -60 dBFS. A wholly quiet scan is reported as a lower bound.
These measurements do not establish browser latency, mixing, loudness or matching
synthesizer instruments. See [the audio audit](../../analysis/AUDIO-PARITY.md) for results and limits.
