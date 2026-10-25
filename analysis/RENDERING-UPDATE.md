# Camera and animation correction pass — 25 October 2026

This pass uses the original Java executable as a component reference and the real Rust WebGL renderer for interactive checks. It is a rendering improvement, not a declaration of complete game parity.

## Changes

- Replaced the fixed player-at-(90,237) camera and linear ground scrolling with the original integer 3D-to-2D projection, including its integer sine-table recurrence, screen center and event coordinate scales.
- Restored the race camera's changing elevation, depth and angle along the track, initial lead, finish framing and dive lead. Hurdles use their distinct finish-zone thresholds.
- Replaced flat grid-like lanes with projected track bands, starting/finish lines and world-space field geometry. Loaded the original stadium/prop scene banks, with tiled and nested scene commands.
- Running animation now advances by world distance, not elapsed time. Negative field run-up coordinates wrap correctly. Hurdles select their own startup/jump/trip sequences.
- Corrected the long-jump takeoff/stretch arrays and timing, including resetting the stretch clock at the input transition. Corrected javelin startup/charge/throw/flight sequences and pole-vault's introduction-then-loop run sequence.
- Added original countdown poses and integer sprite centering. Result-screen animations now keep advancing.
- Carried javelins follow the attachment hooks embedded in the current body animation. Flight orientation is derived from projected world-space endpoints instead of an unrelated fixed screen angle.
- Added a WebAssembly build of the same Rust game (`make web`), so actual rendering and keyboard input can be checked without the native macOS window-init failure.

## Executed-reference checks

`reference-runner/src/OriginalOracle.java` runs the original unmodified JAR, navigates into the game, pauses its own game loop, and invokes the original projection/scene methods on an isolated image. This helper is separate from the live viewer.

- **384 projection samples:** exact equality between Rust and the executed original Java method under eight camera configurations.
- **512 sine-table values:** exact equality with the original constructor.
- **19 rendered-pose samples:** 15 keyframes and four interpolated samples, including the half-frame image-selection boundary. The shared Rust scene decoder is checked against original-rendered PNGs. Comparison allows four RGB levels for alpha quantization in the emulator; this is not a claim of bit-identical GPU alpha blending.
- Repeated the original export in a fresh save-data directory: the projection CSV and all 19 selected PNGs reproduced byte for byte.
- Animation regressions cover distance-driven running, negative run-up wrapping and the pole run-up introduction/loop boundary.
- All six pre-existing mechanics tests still pass; total Rust tests: **12**.
- Native optimized build, WebAssembly optimized build and warning-free Clippy completed.
- Browser inspection covered all five event setups; the renderer uses actual decoded original assets and the updated projection. Native macOS window startup remains an environment limitation, so native graphics/audio timing is not verified.

## Remaining work

The pole simulation remains approximate. Full NPC/competition behavior, original HUD/menu presentation, some scenery clipping and animation-specific drawing hooks, camera transitions driven by states absent from this simplified game, and synchronized whole-event input replays remain incomplete. Browser record saving still uses the unsupported native filesystem path and reports a save failure without preventing play.

The reference runner stays neutral-colored: its green LCD mask is disabled. No tint has been applied to Rust assets.
