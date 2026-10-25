# Executed-original reference fixtures

Generated from the user's unmodified Playman Summer Games 3 1.0.21 JAR with `reference-runner/src/OriginalOracle.java` under FreeJ2ME-Plus (the emulator hash and game hash are recorded in `reference-runner/provenance.json`). Java 21 runs the original obfuscated class; the helper finds overloaded methods by parameter **and return** types.

- `projection.csv`: 384 points evaluated by the original `d.l(int,int,int)` under eight camera configurations. Camera columns contain actual original camera fields, not the offsets accepted by `d.k(...)`.
- `sine.txt`: all 512 values returned by the original integer sine-table constructor.
- `poses/`: representative original character frames rendered into a white 240×320 image at anchor (120,220). The original sprite-loading and appearance remapping run first. The helper also exports interpolation samples across the half-frame sprite-selection boundary.

The Rust tests render the shared scene commands in software and compare these reference images with a maximum four-level RGB tolerance for the emulator's alpha quantization. This tests the scene decoder, transforms, centering and interpolation, without requiring a native graphics window. Native/WebGL drawing uses the same command stream with standard GPU alpha blending.

To regenerate the complete original exports without disturbing the live reference runner:

```
../reference-runner/oracle.sh /absolute/path/to/playmansum_2jvrll9n.jar /absolute/path/to/output
```

The helper creates a fresh session directory for original-game save data. Fixture generation and the game retain their original copyrights. These component fixtures are not a claim of full synchronized game parity.
