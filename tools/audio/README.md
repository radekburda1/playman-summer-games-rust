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
