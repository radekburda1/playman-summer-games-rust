# Extending the game

## Ownership and flow

`main.rs` is the Macroquad adapter: keyboard edges, frame time, record file I/O,
audio playback and window scaling. It samples aliases once into `FrameInput`.
`application.rs` owns menu/attempt/tournament flow, pause, transitions and the
32 ms accumulator. `App::update` accepts plain data and returns audio requests;
it never opens files, polls a device or draws. `records.rs` owns record comparison
and the existing five-line persistence format. Storage failure still appears in
the result screen; browser persistence behavior is unchanged.

`Game` is the simulation facade. `Discipline` owns exactly one event's data:

- Dash and Hurdles share running and dive equations in `race.rs`; Hurdles owns
  sequence progress and the fallen-hurdle mask alongside that behavior.
- `long_jump.rs` owns takeoff, stretch, landing and recovery.
- `vault.rs` owns the pole endpoints, spring constraints and release/landing.
- `javelin.rs` owns charging, the aerodynamic projectile and ground measurement.

`runner.rs` shares the same input history, acceleration and Java RNG wherever the
original mechanics share them. Each discipline has a restricted phase enum.
`Game::phase()` exposes a common read-only vocabulary for presentation. Reference
scenario setup uses the checked `set_phase`; invalid pairs leave state unchanged.
Internal transitions assert the same invariant. Event identity is derived from
its state, never separately writable.

The application sequence is Menu → Help → AwaitStart → Ready → Play → Finish.
Races continue to Results; field events advance through three attempts before
returning to the menu or continuing the tournament. Finish movement has its own
fixed-step update and does not advance measured race time. Javelin fouls can show
a result while the projectile is still flying; do not conflate `finished` with a
stationary scene. Performance feedback and personal-record detection remain
independent decisions.

## Presentation and resources

`presentation.rs` computes fixed-point camera projection, animation selection and
an immutable `SceneFrame`. A frame borrows simulation state, carries explicit
presentation clocks, camera and animation, and cannot advance the game.
`prompts.rs` computes input feedback. `effects.rs` updates deterministic particles
with a separate RNG; `vault_marker.rs` computes pole timing geometry. Effects
remain synchronized with simulation ticks, including post-finish updates.

`world.rs` and `ui.rs` consume immutable state. `artwork.rs` decodes original data
into image/rectangle/hook commands; `art.rs` draws them through `render.rs`.
Equipment attachments must remain inside pose hooks to preserve sprite ordering.
The same renderer is used by the window and software comparison tool; there are
no copied renderer modules or broad dead-code allowances for that tool.

`resources.rs` maps animation-table offsets, shared scenes and equipment sprites
to names. `audio.rs` maps logical `SoundCue`s to original bank IDs. Binary-format
indexing stays in resource readers, not physics equations. `Pole` and `Projectile`
use named scalar fields. Their `reference_values()` methods document legacy CSV
column order solely for oracle tests. Existing event analysis and fixture
provenance retain Java method names and source details.

Audio decoding/mixing stays in `sound.rs`. Application cues are played before the
music update, simulation cues after it, preserving the existing jingle ordering.
Music remains the existing offline MIDI renders; no new decoder was introduced.

## Arithmetic and clocks

One simulation tick is exactly 32 ms. `elapsed_ms` freezes after measurement;
`phase_elapsed_ms` can continue for recovery and reactions. Display time remains
floating-point seconds in application flow. Do not combine these clocks.

Positions and velocities are recovered integer units, not generic SI vectors.
Race positions use 65536 units/metre; Long Jump measures `position * 100 / 106496` centimetres, and Javelin uses
`position * 100 / 88576`. Projection takes
positions shifted right eight bits. Pole geometry has additional 10-bit and
6-bit scales; retain shifts locally beside the equations. Scores and displayed
measurements use integers (hundredths of a second or centimetres).

Keep arithmetic order, signed division, shifts, explicit wrapping operations and
intermediate widths. In particular, the projectile uses i64 intermediates with
explicit i32 truncation at the original update boundaries. The pole's length and
direction approximations are deliberate. A square root or reordered expression
can change trajectories and animation. Keep gameplay and particle RNGs separate.

## Verification

From the project directory:

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
make web-build
make rust
make web
```

`make web-setup` installs the WASM target. Optional `.make.local` can select a
local toolchain/cache and is ignored. `make compare` exports software replay
frames; those exports and native/browser build output are ignored.

Java oracle fixtures test source-recovered state, camera, poses, equipment and
particles. `tests/application_flow.rs` separately checks confirmation/countdown,
pause and independent record detection; record serialization has a pure test.
`tests/architecture_regression.rs` preserves the pre-refactor Rust behavior across
all five events, seeds 1/17/12345, 1200 ticks per replay and ten rendered frames per
replay. `tests/fixtures/refactor.csv` was generated from commit
`3961a6fc257907264f2216020e46760bf224eaf2`, using the same replay inputs against
that commit's implementation. FNV-1a hashes cover selected state scalars,
phase/camera/animation and sound requests, plus RGBA frame bytes. This is a Rust
preservation fixture, **not additional proof of Java parity**. Do not regenerate
it merely to make a changed implementation pass.

During this refactor, all 245 existing replay PNG exports matched their baseline
byte for byte; the additional preservation fixture covers 18,000 simulation ticks
and 150 software frames. Existing Java fixtures were retained without changing
expected values. These checks are representative, not exhaustive equivalence.

Live verification remains limited: native launch from the agent process fails
before window creation at Miniquad's null OpenGL pixel-format assertion, after a
LaunchServices notification failure. Native computer inventory now works, but the
browser access host is unavailable; the fallback browser computer-use session
was ended by an access-policy rejection. No interactive graphics, browser input
or audible playback verification is claimed. These are existing environment
limitations, not resolved by architectural changes.

## Adding or changing a discipline

Add cohesive event data and a restricted phase enum, then add its `Discipline`
variant and event mapping. Implement fixed-step updates and action handlers in
its module; wire the small `Game` dispatch. Reuse `Runner` only when the mechanics
really match. Add application progression rules explicitly if they differ from
races or three-attempt field events. Define resource and audio mappings, then add
camera/animation/prompt selection and scene drawing. Rendering should never
perform a simulation step or consume random numbers.

First test transition boundaries and measured results with deterministic input
traces. Add focused equipment/effect or clipping fixtures where relevant. Exercise
native and browser input, release edges, pause and audio after automated checks.
Record any discovered parity defect separately; this refactor intentionally does
not change recovered physics or fill in missing game modes.

Suggested review commits: (1) simulation modules and named equipment, (2) exclusive
discipline/phase models and resource mappings, (3) application/audio/persistence
boundaries, (4) immutable presentation and shared renderer, (5) preservation tests
and documentation. Some steps depend on earlier API moves; all should be reviewed
with their corresponding test migrations. No commits were created automatically.
