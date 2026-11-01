# Javelin

`src/simulation/events/javelin.rs` translates the original `d.c(c,boolean)` state machine and
`d.f/g(c,int,boolean)` input handlers. The launch prediction uses 15 simulation
ticks (Java truncates the 500 ms prediction to 15 ticks). The throwing animation
lasts 800 ms. Charge phase
is `(phase_ms * 60) & 0x1ffff`; release preserves its original angle, initial
velocities and foul test. The two-ended aerodynamic rod uses the original torque,
damping and integer truncation. Distance is its ground intersection, not its peak
or athlete position. A foul throw continues its trajectory; its result can appear
after two seconds. Fire during flight fast-forwards to landing.

Camera logic comes from `bx()` and `n(int,int)`, including flight height offsets,
foul camera behavior and the post-impact return. Near-horizon projection uses
Java's wrapping integer arithmetic to avoid a debug-only Rust overflow. Ground
markings additionally clip in camera space before projection and to the viewport
afterwards; this prevents behind-camera segments wrapping into the sky.

The renderer now draws the original curved sector, numbered 50–100 m boards,
approach stripes, stands, advertising, benches, official and equipment. Carried
javelins use the pose hook in sprite order. The shaded procedural rod follows the
original in-flight/embedded shapes and clips below the ground on impact. Charge
trails, release bursts and impact debris use original particle presets 0, 2 and 4.
Sample IDs 18/19/20/22/26 follow charge/release/throw/foul/impact triggers.

The event uses field start confirmation and a round-result panel, with independent
performance feedback and personal-record detection. Three attempts preserve the
best result. Field events no longer inherit a generic race countdown.

## Reference checks

`tests/fixtures/javelin/provenance.json` records counts, results and hashes. The
fixture generator is preserved in `tools/oracle/JavelinOracle.java`; the companion
runner's `javelin-oracle.sh` runs it against the unmodified JAR. Inputs cover valid,
early/late release, foul, missed throw, wrong running inputs, idle and flight skip.
The eight scenarios cover 3,021 simulation ticks. Each numerical row compares the
full phase/timer, athlete state, rod state,
release values, foul flag and camera. Selected original frames guard carried and
flying rendering. The in-flight world frame differs at 16 of 76,800 pixels with a
four-level RGB tolerance; that is a selected frame check, not whole-game parity.

The `--effects` oracle mode uses Java Random seed 54321 and the original particle
update/draw methods. All 8,131 active particle records matched before retaining
selected emission/expiry/midpoint ticks in the compact fixture. Pole Vault's
particle regressions still pass with the shared preset implementation.

`make compare` exports world replays under the ignored
`analysis/hurdles/rust/javelin/` directory. Tests, strict Clippy, native release and
WASM release builds are checked. Run the game with `make rust` or `make web`.

## Remaining verification

Live input, GPU rendering and audible mixing/timing remain unverified: computer
use still reports no surfaces and `Sky Computer Use native pipe startup failed`.
Software comparisons show remaining stadium/band-edge and ground-line raster
mismatches in some views. Exact record-arc/result-panel transition timing and the
original profile, opponent and challenge progression are not fully reproduced.
The timing cursor is drawn above its particles for legibility, consistent with
the requested Pole Vault adjustment. Full visual/audio parity is not claimed.
