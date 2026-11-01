# Pole Vault

The scripted arc has been replaced by the original integer end-point/spring model.
`src/simulation/events/vault.rs` translates `d.h(c)`, `d.h/i(c,int,boolean)`, `d.e(c,boolean)`,
`d.i/j(c)` and `d.a(c)`; `d.o(int,int)` supplies the original length approximation.
The twelve existing pole values retain Java's r,s,t,u,v,w,x,y,C,B,E,D order.
`Vault` stores the held latch, measured clearance, fall height and camera look-ahead.

The implementation includes the 800 ms preparation, run-up, plant/hold latch,
bend/straighten phases, release, recoil, mat/ground landing, failed attempts and
post-landing pole movement. Clearance is the minimum sampled body height while
x is within [118000,138000], divided by 855; it is not peak height. Camera motion
uses the original bP look-ahead, height-driven angle and post-landing return.
Plant, release, landing and failed-input sound IDs follow the source.

Rendering now uses the original runway bands/marks, benches, mat trim and handle
sprites, official/equipment sprites, shaded uprights, pose-hook attachments for the carried pole, segmented bending pole,
moving spark, mat clipping and landing shadows. Preparation, flight/fall and
result reactions use the recovered animation tables. Pole Vault shares the
confirmation/finish flow with field attempts rather than the race countdown.

## Verification

Seven isolated original-JAR traces cover 2,075 ticks: clean, early plant/release,
late plant, no plant, no release, release before planting, and wrong run-up inputs.
The numerical regression compares phase/timer, runner position/speed/height,
vertical velocity, next input, all twelve pole values, held state, measured
clearance, fall height, look-ahead and four camera values at every tick, including
41 post-landing ticks. It also checks the measured Rust result. The case names
identify input strategies, not quality: the `late` strategy yields 6.47 m, whereas
`clean` yields 4.39 m. Both match Java.

Fixtures and source/JAR hashes are in `tests/fixtures/pole-vault/`. The Java generator
is `tools/oracle/PoleVaultOracle.java`; the companion reference runner's
`pole-vault-oracle.sh` executes it. It uses seed 12345 and 32 ms ticks, fixes athlete
appearance, and initializes transient scratch fields for independent scenarios.
On landing it writes the result through Java's original score-store method so
rendered reactions use the same measured result.

`make compare` also exports Pole Vault world frames under the ignored
`analysis/hurdles/rust/pole-vault/` folder. Compare these with matching ticks in the
companion runner's `captures/pole-vault/`. Tests, strict Clippy, native release and
WASM release builds pass; Dash, Long Jump and Hurdles regression checks remain.

## Remaining differences

This is not full visual/audio parity. Mat-side rasterization and trim clipping,
pole/rod rasterization,
release-burst particles and exact result-panel timing still need refinement against the
reference frames. The world-only harness does not compare the full HUD/transition
flow. Sound IDs are source-mapped, but audible mixing/timing is unverified.
Computer use still returns no available surfaces (`Sky Computer Use native pipe
startup failed`); live browser/native visual and audio verification remains blocked.
CPU opponents and original challenge/profile progression remain outside this port.

The carried-pole regression compares the original solid pole palette in three
Java preparation/run-up frames. Pose hooks render in sprite order, using Java
`d.i` direction quantization and the original 184-pixel pole length.

Timing highlight: original preset 1 emits six particles per 32 ms tick, with
350–449 ms lifetimes, original sprites 463–465 and full emitter-following motion.
The moving spark remains sprite 469. Stored Java bQ progress and integer marker
interpolation are preserved. The `--effects` oracle mode exports active particles
with visual RNG seed 54321; all 5,276 records matched before selecting 11 ticks
for the compact regression fixture. Visual randomness uses a separate generator
and cannot alter athlete inputs or physics. The timing spark is composited above the particles for legibility; this is an
intentional drawing-order adjustment from Java, which composites particles last.
