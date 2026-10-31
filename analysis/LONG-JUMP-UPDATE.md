# Long Jump parity work — incomplete

## Implemented

- Preparation animation (table u, 2618, 11 keyframes, 120 ms); a first running input skips it. Long Jump no longer plays the dash Ready/Set/Go countdown or its three sound cues.
- Original takeoff, apex/stretch, early-stretch penalty, landing deceleration, minimum landing speeds and recovery movement. Distance is measured at first ground contact and remains fixed during recovery. A foul now has its own run-out stage.
- Landing animations t (2596), v (2650) then w (2672), camera offsets during landing, score-based reactions and direct progression through three attempts. Other events retain their existing flow.
- Original crowd/advertising scenes, equipment scene 8212, official scene 16385, takeoff board and side markers 492/493, approach strips, twenty-band sand gradient, pit rails, metric ruler, takeoff marker 478/479, landing marker 482/462 and local-record flag 483. Foreground scene 8200 retains its source projection.
- Queued source sound IDs: 18 takeoff, 20 stretch, 22 bad stretch/foul/bad landing, 26 good landing, 19 wrong running input. Original WAV files are used. Queues prevent frame-dependent duplicate cues.

## Original-source map

`bg()` initializes an attempt; `b(c,boolean)` advances the original 32 ms state machine; `d(c,int,boolean)` handles input; `bk()` selects the camera and renders the event; `bm()` places stadium layers; `bn()` draws the board, runway, pit and ruler; `c(c,int,int)` selects athlete animation and clipping; `e(c,int,boolean)` draws measurement markers. The original JAR is unchanged.

## Executed reference evidence

`../reference-runner/long-jump-oracle.sh` runs the original JAR in an isolated FreeJ2ME session. It initializes each of three attempt indices, starts from a controlled approach state (position -65536, speed 28000), then executes valid, early-stretch, late-stretch and foul scenarios. This is a controlled original-method probe, **not a replay of the entire normal UI flow**.

The output contains 372 PNGs and matching state JSON, plus twelve 240-tick CSV traces. The exporter reports zero render errors. `long-jump/original-states.png` is a contact sheet of selected original frames. Source/JAR/trace hashes are in `tests/fixtures/long-jump/provenance.json`.

Rust tests compare 100 consecutive ticks for valid/early/late jumps and 60 foul ticks: position, speed, height and vertical velocity match exactly. Sampled camera x/y/z/angle values also match the original JSON. Additional tests check preparation skipping, the strict positive-position foul boundary, takeoff/foul sound IDs and the Rust three-attempt progression. The prior dash, camera, sprite and feedback tests remain passing.

## Verification and blockers

25 tests pass; strict Clippy, native release and WebAssembly release builds succeed. The live Java runner responds on port 8765. A native Rust launch was attempted again and aborts before window creation in Miniquad's macOS `apple_util.rs`, after a LaunchServices notification failure. Browser automation timed out on the game canvas and reset its runtime; subsequent Edge operations encountered concurrent user changes, and no isolated browser provider is available. Consequently no new matched-state live Rust screenshots or audio-listening comparison were obtained.

## Remaining differences — no full parity claim

- Sand spray/particle effects and all scene clipping/details have not been matched pixel by pixel.
- Original challenge/CPU/country/profile-dependent official/result behavior and exact result-screen layout/choreography remain incomplete. The local record marker uses the Rust saved personal best.
- Foul run-out duration and some presentation transitions still require normal-flow comparison. The tests cover controlled simulation states rather than all original menu/input transitions.
- The three attempt indices were probed in isolation; a full original-vs-Rust three-attempt live replay remains outstanding.
- MIDI assets are extracted but still not synthesized. Sample IDs are source-mapped, but audible volume/mixing and timing have not been listened to side by side.

## Build and compare

From the workspace root: `make rust`, `make java`, or `make web`. Run the isolated reference probe with `outputs/reference-runner/long-jump-oracle.sh`. Test with `make test` and inspect `tests/long_jump.rs`. Do not interpret successful builds or the numerical component checks as proof of full visual/audio parity.

Approach scenery correction: restored scene 8196 using the original `d.h(8196,-512,256,-4608)` repeat-to-left placement, period (scene width + 256), depth -512 and source cutoff at world x=-4608. This is the omitted approach equipment layer. Also restored the distant scene 8222 at depth -128000.
