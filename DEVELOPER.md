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
  platform/                 Window/input adapter, audio playback and record storage
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
files. Record comparison/encoding is pure; executable-adjacent native file I/O and
browser localStorage imports are private to the platform. The browser bridge in
`web/records.js` stores the original five slots plus an appended optional sixth line for 400m under `playman-records-v1`.
R on the main menu opens a reset confirmation; Escape cancels without changing
records. Storage failures remain visible rather than silently reporting success.

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
playback and music scheduling. `App::background_music()` gates background playback:
menus and confirmed results allow it; AwaitStart/countdown/play/pause/finish do not.
The platform stops the menu/intro track at attempt entry, without disabling sampled
effects or explicit record jingles. Result/celebration/record requests share an
ordered platform queue with original resource-table delays. Music decoding runs
in a coroutine so WebAudio readiness cannot suspend drawing. Application cues
precede the music update;
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

Live browser verification (2026-11-01/02) covered representative states in all five
events, using the existing Rust server at 8767. The Java viewer at 8765 was reachable,
but live visual states were not synchronized for comparison.

- Dash: selection/help/start/countdown, correct/wrong input feedback, action near
  the finish, run-out and results; a 12.53s record survived a full reload.
- Long Jump: start/run-up, foul recovery, attempt 2/3, valid takeoff/stretch,
  4.46m landing, recovery and record badge.
- Hurdles: countdown, three-slot prompts, collision/recovery and a 15.50s finish.
- Pole Vault: carried/running pole, hold/release, airborne/fall/landing and foul
  finish. Successful clearance remains unverified live.
- Javelin: carried rod/run-up, hold/release, flight, flight skip, valid 26.30m
  result and next-attempt scene.

Pause/resume, pause-menu Retry/Sound/Main Menu and M sound-state changes were
observed. Browser music and effects were confirmed audible by the user; exact
mixing, instruments and output timing remain unverified. Selected executed Java
cue probes and the background-music policy are documented in
[the audio audit](analysis/AUDIO-PARITY.md).

Record reset/cancel/reload was checked with a disposable storage key seeded with
10.00s, preserving the real record key. `node tests/web_records.cjs` additionally
covers all slots, malformed values and blocked writes. After `make web-build`, run
`node tests/web_bundle.cjs` to check the staged JavaScript. Staging applies the
pinned quad_net strict-mode fix and versions WASM/JavaScript URLs by content hash;
unchanged assets retain their URLs. Native and WASM release builds, the Rust suite,
strict Clippy and formatting checks passed during this verification.

Remaining live coverage includes successful vault clearance, complete three-attempt
and tournament flows, R retry from detailed race results, sustained frame pacing,
synchronized Java visuals and detailed audible parity. Deterministic fixtures and
successful builds do not establish these behaviors.

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

## Local multiplayer flow

ModeSelect precedes the event menu; PlayerSetup supports one singleplayer athlete or 2–6 multiplayer athletes.
App owns the roster, active player, round and per-player event bests/totals.
No multiplayer state enters simulation or changes the 32ms mechanics.
The active best is saved only when confirming a completed turn. Field events
rotate players before advancing the round. Standings gates the next tournament
event; points are awarded once after the final turn. Setup names are ASCII,
limited to 10 characters and kept in memory. The platform suppresses gameplay
letter aliases, Space confirmation and M mute while editing names.

`tests/multiplayer_flow.rs` covers mode/setup boundaries, two/six-player rotation
for all five events, best preservation through fouls, stable ties, retry and
complete tournament totals. These are application flow checks, not additional
Java physics parity fixtures. Source inspection of original d.s(int), d.aG(),
d.ap() and d.aq() established roster bounds, round rotation and score ordering.
CPU opponents and body/outfit/country setup are deferred.

Live browser verification of the initial multiplayer implementation covered mode
selection, the six-player setup layout, reducing back to two players, typing ADA
without triggering A/D navigation, named NEXT UP/finish screens, and a complete
two-player Dash (ADA 9.15s, PLAYER 2 9.12s) with correctly ordered standings.
Six-player gameplay, full field-event rotation and complete tournaments have
application-test coverage but have not yet been played through live. The native
app was rebuilt, not exercised live for multiplayer.

## Multiplayer race replays

Original d.aP/d.aW replay tick-stamped inputs for previously controlled runners;
d.aQ/d.ba draw only CPU/prior/current runners in separate lanes. Original
d.n(rank,result) selects place-dependent reactions; reaction 5 uses the 3472/3536
seated recovery sequence. The high-score reaction override is not applied to
multiplayer placement reactions, so two strong performances still show a winner
and defeated runner. Singleplayer animation selection is unchanged.

Rust records compact immutable RaceSample values after each 32ms mechanics tick,
rather than re-simulating input logs. Position, animation (including dive), and
fallen-hurdle masks replay on the active attempt's fixed-step clock. Countdown,
pause, retry and next-event reset are independent of replay playback. Sampling
stops once the recorded runner has stopped; later frames extrapolate reaction
time, bounding memory even when a finish screen remains open indefinitely.
Replays retain one mechanics snapshot at the finish crossing until rebuilding
post-measurement motion, then release it. They own no effects, audio or storage
callbacks. SceneFrame carries immutable race athlete descriptions; rendering
cannot step the replay.

Multiplayer uses the continuous athlete-follow camera through Play and Finish.
The original d.b(c,target) coast/brake calculation now advances the active runner
toward 29440 (+5120 for Hurdles) without teleporting. Prior runners' gameplay
samples stay intact; their post-measurement run-out is rebuilt from a crossing
snapshot toward 28672 + d.h(scene_seed + 813387 * player) % 1536 (+5120 for
Hurdles), with the original wrapping hash. The Rust session seed supplies this
scene seed; Java randomly initializes its seed, so exact stopping offsets across
independent sessions need not match. Original lane drawing order is retained;
these offsets reduce overlap but are not a general sprite collision solver.
The singleplayer step_finish path and measured race arithmetic remain unchanged.
The brief transition wash remains. Dash dive requests emit the action sound
directly without the non-original DIVE! running text.

`tests/race_replay.rs` checks both race recordings tick by tick, dive poses,
fallen-hurdle masks, live-state/audio independence, pause, lane bounds, placement
reactions and bounded idle sampling. These validate the Rust replay behavior;
they are not new executed-Java multiplayer oracle fixtures.

Live browser verification covered two-player Dash with both runners on NEXT UP
and moving together, a Player 2 dive without DIVE! running text, and finish
transitions with athletes retained through run-out and reactions. Runs covered
both a winning first player and a winning second player. The final separate-stop
check showed Player 2 (8.34s) celebrating in front and Player 1 (9.12s) recovering
seated behind at a distinct position. The transition wash remains visible.
Screenshots were saved in the desktop chat outputs directory. Verification runs
improved the shared Dash record to 8.34s; records were not reset.

Hurdles replay/collisions, dive-adjusted braking, frozen measurements and six-player
lane bounds have automated coverage. Native multiplayer, full six-player finish
layouts, synchronized Java replay comparisons and a frame-by-frame browser video
audit remain unverified. The fixed-tick finish regression checks both races
through crossing and 100 subsequent ticks for continuous physical positions and
viewport visibility. These checks do not establish complete original-game parity.

## Player appearance

Player owns assets::character::Appearance separately from simulation state. Its
bounded Face value cycles the 12 original character sets plus the custom brother, Kubak, Radek, Burdas and Tereza; default is Playman
(index 1), preserving existing software/GPU fixtures. Original d.Y uses portrait
sprites 41 + face (small) and 53 + face (selected). Original b0 character metadata
at byte 646 contains five i32 values per face; the fourth value maps animation
slots 21–24 to four head views. The existing decoded athlete scenes already use
resolved default sprites. frame_with_face chooses the original male/female pose
bank and reverses its default body/head sprites into mapping slots. It substitutes
the selected character's body/skin table (b0 offsets 262/346/430/514) and head views,
retaining transforms, layer order and equipment hooks. No new raster assets or
physics changes are needed.

Art::with_appearance creates a scoped renderer sharing textures and Arc-backed
immutable scene banks. It applies the active athlete's complete appearance through all five events
and each RaceAthlete's own appearance during multiplayer replay rendering.
Appearance never enters Game, RNG, scoring or record storage. Singleplayer also
opens setup; player count stays fixed at one. Setup uses arrows for faces and
plain text for names. Choices stay in memory, matching current name persistence.

Portrait selection now includes its matching body bank, skin and original
vertical anchor scaling. The default Playman retains 100% scale for compatibility
with existing pixel fixtures (original metadata specifies 95%); other characters
use their recovered height percentage. Country uniform colours remain fixed at
the existing default. Independent body/outfit edits remain future work.
`tests/character_faces.rs` covers portrait/head/body resources, all 318 poses for
all 12 characters, default frame compatibility, equipment hook presence,
singleplayer setup and independent appearances in active/replayed athletes.
Existing pixel/state fixtures remain unchanged.

Live browser face verification selected distinct original portraits for two
players, completed Player 1's Dash with the matching animated head, checked both
heads on Player 2's NEXT UP/replay scene, and completed Player 2's Dash with both
custom faces retained in the final reactions. All 12 face resources and prepared
poses in all five disciplines have automated coverage. Native custom faces and
complete customized field-event attempts have not been checked live. The browser
verification improved the shared Dash record to 8.31s without resetting records.

The matching-body correction was verified live with Vera in Dash preparation and
running, showing matching dark skin and the female body bank, and in Pole Vault
preparation with the pole attached to the selected character. All 12 characters'
318 pose frames decode in automated checks; complete custom-body field attempts,
multiplayer finish reactions and native custom bodies remain unverified live.

## Custom brother character

Face index 12 selects a custom male preset, with unchanged gameplay. Pure
assets::custom_character defines virtual sprites 4096 onward: four generated
head views, two supplied-reference portrait crops, 21 broader body parts and
four round-belly uniform views. Rendering::atlas assembles these below the
original atlas at startup for both GPU and software renderers, preserving every
original atlas byte and coordinate. All eight original sprite transforms are
provided; custom sprites retain original keyframes, anchors and equipment hooks.
The fuller build affects sprite silhouettes only, never Game or collision state.

The reference image is copied unchanged to assets/custom. Head views were made
with built-in image_gen; the exact prompt is in assets/custom/README.md. Generated
heads are alpha-bounded and sampled down to game sprite dimensions. The portrait
uses the reference crop directly. No generated asset is referenced outside the
checkout. Tests check selectable assets, transforms, fuller torso widths, custom
head alpha and byte-for-byte preservation of the original atlas region.

Browser checks verified selection/name editing, running, an 11.65s Dash finish and
the carried-pole pose. Full customized field attempts, custom multiplayer replay
and native live appearance remain unverified; existing rendering fixtures and
all character pose decoding checks still pass.

Kubak is face index 13. His six custom sprites follow the brother's existing
virtual sprites, so all prior custom coordinates remain stable. Four generated
head views and two crops of the supplied portrait use the same atlas compositor.
His body, uniform and height use the standard Playman preset; the rounded torso
warp is restricted to the brother's indices 27–30, never custom head sprites.
Tests check the independent heads/portraits, all sprite transforms, custom alpha
and byte-for-byte reuse of the standard body/uniform sprite mappings.

Kubak verification: browser selector and Dash running animation inspected live;
standard body mapping and all custom transforms checked by tests. Full suite,
Clippy, web storage/bundle checks, and web/macOS builds pass. Native Kubak
visuals and every event were not individually inspected in this pass.

Brother head revision: all four views end at jaw/skull base without a neck. Both selector sizes sample the alpha-bounded frontal head, retaining transparent corners instead of the original opaque portrait crop. Rear head height is 28 pixels to preserve its compact silhouette.

Kubak also uses neckless jaw/skull-base silhouettes in all four views; rear-view height is 28 pixels. Standard body and supplied selector portrait are preserved.

Brother torso slots 612–615 represent successive lean poses, not camera views.
Runtime samples the profile of `brother-torsos-shoulders.png`, drawn with a compact
chest and large low abdomen. Pure asset metadata records calibrated shoulder/hip
landmarks for each original pose. `torso_sample` inversely maps the shoulder to
(18%,17%) and bottom waistband to (47%,94%) in the source, attaching the hem
to the shorts/leg joint. Front width scales 1.5x across the torso axis; length is
fixed by the two joints. No opaque belly fill is applied. A regression bounds
the hem within 15% of the shoulder-to-hip length beyond the hip, preventing the
previous 70% source anchor from extending the belly toward the knees. Sprites
use 96x96 transparent cells before all eight transforms, with no post-placement
offsets. Arms/lower limb pieces retain 1.45x width; hip and shorts-bearing upper-leg
pieces (custom slots 16,23–26) keep original widths to avoid an exaggerated
black wedge during celebration jumps. Original pixels and mechanics remain unchanged.
The open shoulder socket replaces a tall closed loop, with a transparent arm-joint
regression alongside the hem-to-leg regression. Previous generated sheets remain in the checkout. Prompts/provenance are in
assets/custom/README.md.

Radek is Face 14 (17 total choices including Burdas and Tereza); his six virtual sprites append at BASE+37. Four neckless head views and two transparent menu sizes share radek-heads.png. Body/uniform mapping remains standard male, independent of simulation. Generated heads are separated by transparent gutters to avoid neighboring strips.

Radek verification: transparent menu preview and Dash running pose inspected live; all 76 tests, Clippy, formatting, web storage/bundle checks and web/macOS builds pass. Other events and native visuals were not individually inspected in this pass.

Custom head turns: Brother, Kubak, Radek, Burdas and Tereza map requested rear head view 3 to
existing three-quarter view 1 through one shared Face helper. Both head_sprite
and sprite_for_scene use it, so prepared animation frames and direct head draws
agree. Rear artwork remains stored for provenance; original 12 characters retain
original view mappings. The all 318-pose regression checks every custom face for
absence of the rear atlas rectangle, including its transforms. This is cosmetic
and does not alter pose clocks, equipment hooks, replay or mechanics.

Burdas is Face15, appending six sprites at BASE+43 (current COUNT55): four neckless heads
and two transparent menu previews sampling the frontal head. Source is
assets/custom/burdas-heads.png, alpha-bounded through transparent gutters before
nearest-neighbor resizing. Existing virtual IDs are unchanged. Standard male
body/uniform/height mapping; no mechanics or original-pixel changes. Setup footer
shows Burdas; user-entered names remain independent. All 17 faces ×318 poses are
covered by decoding/equipment checks; Burdas selection and Dash were inspected
live. Exact generation prompt and identity/style reference provenance are in
assets/custom/README.md.

Tereza is Face 16, with six virtual sprites at BASE+49 (COUNT55). table_value uses
original female preset 0 for her: female scene 1, light-skin body bank 514 and
standard female uniform 624..627. Other custom presets remain male. Her neckless
heads use compact curled hair at 39x34 / 37x36 / 33x36 / 36x32, inside the original
female hair envelope. Portraits sample the frontal head; rear turns use the
three-quarter face. Source tereza-heads.png uses
transparent-gutter segmentation. Hazel eyes and slightly longer ponytail are
recorded in assets/custom/README.md with both generation prompts and references.

All five custom characters’ gameplay heads use a 3px anchor offset toward the head top, transformed
through all eight orientations. This reveals the body neck
that the taller heads otherwise cover. Source heads remain neckless; menu
portraits, torsos and all other custom assets have no anchor offset.

## Experimental 400m dash

`Event::Dash400` shares the Dash runner, input handling, countdown, dive,
animations and sound dispatch. Pure event metadata supplies the 400m finish,
380m input/dive cutoff and finish-area translation. No stamina, bends or new
running equations are introduced. The attempt timeout is 240s rather than 60s.
400m points use the original Dash curve on elapsed hundredths divided by four;
this is an experimental comparison scale, not an original-game scoring rule.

`Event::ALL` retains the original five events for tournament order and oracle
fixtures. `SELECTABLE` appends 400m at slot 5; `COUNT` sizes records and per-player
results. Tournament occupies menu slot COUNT. Both storage adapters read old
five-line records without moving slots and write six lines under the same key/
filename. The sixth record is independent; no records are reset by migration.

Track finish props, lane markings, camera finish offsets and multiplayer active/
replay stopping targets shift together by 300m. Existing 100m/Hurdles coordinates
and reference fixtures remain intact. Original running artwork is reused; 400m
has an EXPERIMENTAL menu label and the running icon. New regression tests cover
runner equivalence, dive/finish boundaries, frozen measured time, extended run-out,
legacy record migration, menu selection and two-player completion/replays.
