# Extending the game

## Layout and dependency direction

```text
src/
  lib.rs                    Layer declarations only
  main.rs                   Macroquad bootstrap only
  application/
    mod.rs                  Menu, attempts, tournament, pause and transitions
    gameplay.rs             Fixed-step physics/effects choreography and flight skip
    records.rs              Record decisions and pure text codec
  simulation/
    mod.rs                  Game state, inputs and fixed-step dispatch
    discipline.rs           Exclusive event data and restricted phases
    event.rs                Event descriptions, scoring and result formatting
    runner.rs               Shared acceleration, input history and RNG
    math.rs                 Original integer trigonometry
    audio.rs                Logical audio requests
    events/                 Shared race, Long Jump, Vault and Javelin mechanics
  presentation/
    mod.rs                  Camera, animation and immutable SceneFrame
    prompts.rs              Input feedback layout
    effects.rs              Visual particle state, RNG and updates
    vault_marker.rs         Pole timing and carried-equipment geometry
    javelin.rs              Pose-hook markers, particle triggers and ground clipping
  rendering/                Artwork drawing, software/GPU primitives, world and UI
  platform/                 Window/input adapter, audio playback and record-file I/O
  assets/                   Pure artwork/font decoding and named resource mappings
  bin/compare.rs            Development-only software replay exporter
```

The root does not re-export the old flat namespace. Callers import from the
owning layer. Binary resources stay in the existing top-level `assets/` directory;
compile-time includes are anchored to the manifest directory, not a workstation.

Dependency direction is enforced by visibility and an architectural regression
check in `tests/layer_boundaries.rs`: simulation imports no application,
presentation, rendering, platform, Macroquad, filesystem or environment APIs.
Application orchestrates simulation and presentation. Presentation reads simulation
and pure assets; rendering consumes both. Platform adapters perform device and I/O
work. The compare binary shares the library renderer without path-based modules.

## Ownership and event flow

`simulation::Game` owns mechanics and domain feedback only. It owns no camera,
particle RNG, texture, device or file handle. `Discipline` holds exactly one event's
state. Dash/Hurdles share race equations; each field discipline keeps its own
mechanics. Event identity is derived from its state, never separately writable.
`Game::phase()` provides a common read vocabulary, while each event stores a
restricted phase type. Checked `set_phase` supports oracle setup and leaves state
unchanged on error. Internal transitions retain invariant assertions. Public
`as_vault`/`as_javelin`/`as_long_jump`/`as_hurdles` accessors return `Option`;
infallible accessors are crate-private and follow explicit event dispatch.

`application::App` owns the game and a separate visual-effects timeline. Keyboard
aliases are sampled once into `FrameInput` by the platform. `App::update` owns the
32 ms accumulator and returns audio requests without polling devices or writing
files. Record comparison/encoding is pure; executable-adjacent I/O is private to
the platform. Browser save-failure behavior remains unchanged.

For complete replay/presentation behavior, use `application::gameplay::input`,
`step` and `step_finish`, passing both the game and its effects timeline. These
functions coordinate the independent layers. Low-level `Game::step` advances
physics only. Pole mechanics emit timing progress/activation as domain outputs;
application then advances the marker and particle clock. Javelin flight-skip is
application orchestration: it repeats complete physics/effects ticks. Do not call
raw physics steps in a replay intended to verify visual effects or flight skip.

Flow remains Menu → Help → AwaitStart → Ready → Play → Finish. Races continue to
Results; field events advance through three attempts before returning to the menu
or continuing a tournament. Finish movement does not advance measured race time.
Javelin foul results can appear while the rod still flies. Performance feedback
and record detection are independent decisions.

## Presentation, rendering and assets

`SceneFrame` borrows a game and its effects timeline immutably and carries explicit
presentation clocks, camera and animation. Creating or drawing it cannot advance
physics, particles or random numbers. Equipment attachments stay inside pose hooks
to preserve sprite ordering. `assets::artwork` decodes commands without a graphics
dependency; `rendering::art` executes them using shared GPU/software primitives.
The software backend remains thread-local as before; replacing its backend model
would be a separate change with additional rendering risk.

`assets::resources` names original animation offsets, scenes and equipment sprites.
`simulation::audio::SoundCue` describes logical requests; platform audio performs
playback and music scheduling. Application cues still precede the music update and
simulation cues follow it. The existing offline MIDI renders and mixing settings
are unchanged. Pole/projectile `reference_values()` methods retain only the oracle
CSV order. Java names, equations and trace provenance remain in reference notes.

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
The layer-boundary check prevents simulation from acquiring outer-layer imports.
`tests/architecture_regression.rs` preserves the pre-refactor Rust behavior across
all five events, seeds 1/17/12345, 1200 ticks per replay and ten rendered frames per
replay. `tests/fixtures/refactor.csv` was generated from commit
`3961a6fc257907264f2216020e46760bf224eaf2`, using the same replay inputs against
that commit's implementation. FNV-1a hashes cover selected state scalars,
phase/camera/animation and sound requests, plus RGBA frame bytes. This is a Rust
preservation fixture, **not additional proof of Java parity**. Do not regenerate
it merely to make a changed implementation pass.

During both architectural passes, all 245 existing replay PNG exports matched their baseline
byte for byte; the additional preservation fixture covers 18,000 simulation ticks
and 150 software frames. Existing Java fixtures were retained without changing
expected values. These checks are representative, not exhaustive equivalence.

Native verification is now available through a macOS app bundle. `make rust`
builds/stages `dist/Playman Rust.app` and launches it through LaunchServices on
macOS; other platforms still run the executable directly. `make mac-app` stages
without launching. `make rust-direct` retains the original CLI launch for
diagnostics. The release executable is unchanged by packaging. Existing records
beside the direct executable are copied on first bundle creation only; subsequent
packaging preserves bundled records. Build products remain ignored.

The direct agent-process launch still fails at Miniquad's null OpenGL pixel-format
assertion after a LaunchServices notification failure. The same executable launched
as a desktop app renders successfully. Live native checks verified menu/confirmation,
Space-to-start, 4/6 input and pause, plus Long Jump and Pole Vault scene rendering
(including the carried pole). The user confirmed audible music and M mute/unmute.
This removes the native rendering/input/music blocker; it is not a complete
five-event visual or sound-trigger parity claim. ScreenCaptureKit screenshots
occasionally fail with capture error -3811, independently of the running game.

Browser automation remains a separate client limitation: inventory and fresh-tab
creation report `The browser access host is unavailable`, including after the user
opened/attached the tab and the computer-use connection was reset and rebound by URL. Building and serving the
WASM version does not repair that host service. The existing server at `http://127.0.0.1:8767/` was reachable after granting the
agent network permission; its HTML, WASM and JS matched the current staged build
byte for byte. Browser input and audio have not been checked live in this session. No browser access-policy bypass was attempted.

## Adding or changing a discipline

Add cohesive event data and a restricted phase enum, then add its `Discipline`
variant and event mapping. Implement fixed-step updates and action handlers in
its `simulation/events/` module; wire the small `Game` dispatch. Reuse `Runner` only when the mechanics
really match. Add application progression rules explicitly if they differ from
races or three-attempt field events. Define resource and audio mappings, then add
camera/animation/prompt selection and scene drawing. Rendering should never
perform a simulation step or consume random numbers.

First test transition boundaries and measured results with deterministic input
traces. Add focused equipment/effect or clipping fixtures where relevant. Exercise
native and browser input, release edges, pause and audio after automated checks.
Record any discovered parity defect separately; this refactor intentionally does
not change recovered physics or fill in missing game modes.

Suggested review commits: (1) module moves, explicit imports and resource include
paths, (2) independent simulation/presentation clocks and application orchestration,
(3) visibility, architectural checks and replay migrations, (4) documentation.
The second step needs its replay-test changes in the same commit. No commits are
created automatically.
