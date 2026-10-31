# Playman Summer Games 3 — Rust reconstruction

A native Rust/Macroquad reconstruction made from the supplied `playmansum_2jvrll9n.jar` (Java ME, version 1.0.21).

**Status: playable implementation of five events, not a complete parity port.** The running model, dive boost, long-jump equations, javelin flight equations and event scoring were translated from the decompiled code. Pole vault uses an approximate solver. Original graphics and sampled sounds are extracted and embedded. CPU opponents, original challenge progression, Survival, Minimode and MIDI music are not implemented. The Rust browser build is now exercised interactively; native window execution remains blocked by the authoring environment’s macOS initialization failure.

## Build and run

Install a current stable Rust toolchain, then run from this folder:

```sh
make rust
```

Use `make java` (or `make original`) to launch the original Java game in the
sibling `reference-runner` folder, then open http://127.0.0.1:8765. Stop either
version with Ctrl+C in its terminal. The Java viewer uses the original game's
normal colors with the emulator's green LCD filter disabled.

`make build` builds the Rust release executable; `make test` runs its tests.
`make help` lists the targets. The workspace root also forwards these commands.
For a different JAR or port, use:

```sh
make java JAR="/absolute/path/to/game.jar" PORT=8766
```

The Java runner requires Java 21 and reuses the workspace's downloaded runtime.
Override it with `PLAYMAN_JAVA=/absolute/path/to/java`. If the runner is located
elsewhere, set `REFERENCE_DIR=/absolute/path/to/reference-runner`.
The source archive contains the Rust project; download `reference-runner.zip`
separately and extract it beside `playman-rust` to use `make java` outside this
workspace. Direct Cargo commands (`cargo run --release --locked`, etc.) also work.

No Java installation, source JAR, asset download, Python package, or external asset path is needed to run the delivered project. Assets use `include_bytes!`, so the built executable also runs from another working directory.

On Linux, install the native window/audio development dependencies required by Macroquad (typically `libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev` on Debian/Ubuntu). macOS uses the system frameworks. The build was checked on Apple Silicon macOS; other targets have not been built here.

## Browser version

The browser build runs the same Rust simulation and renderer and is useful for comparing with the original Java viewer:

```sh
rustup target add wasm32-unknown-unknown
make web
```

Open http://127.0.0.1:8767. Click the game to focus input. `WEB_PORT` changes the port. The Java reference remains at http://127.0.0.1:8765 (`make java`). Native play is still `make rust`. The browser build currently cannot persist records through the native filesystem; a save failure does not stop the game.

## Controls

| Action | Keys |
|---|---|
| Navigate menus | Arrows, A / D, 4 / 6, or keypad 4 / 6 |
| Select / start / continue | Enter, Space, 5, or keypad 5 |
| Left prompt / right prompt | Left / Right, A / D, 4 / 6, or keypad 4 / 6 |
| Jump, dive, charge, pole action | Space, 5, or keypad 5 |
| Pause / resume | P |
| Mute sampled sounds | M |
| Retry from result screen | R |
| Next attempt / event | Enter |
| Return to menu | Escape |

Each attempt opens on a stationary **NEXT UP / Start** scene. Press **5 / Space** (or Enter) to begin the countdown; waiting on this screen does not advance the attempt. The dash start includes the original starting blocks and yellow equipment boxes in all six lanes.

Running follows the displayed **sequence**, which can repeat a side. Button presses are edge triggered: holding a direction does not repeatedly run. Correct 4/6 inputs produce the original three-frame white pulse (200 ms), with a short bounce on the next prompt; wrong inputs show a red X (300 ms). Hurdles highlight the consumed sequence slot.

Menus use the extracted original bitmap fonts and sport icons on white backgrounds with red selection bands. The pause menu supports resume, retry, sound and return to the event menu. Use 4/6 to navigate and 5 to select.

- **100m Dash:** follow prompts until the final section, then press the action key near the finish to dive.
- **Long Jump:** run up, press action before the white board, then press again at the apex to stretch. Crossing the board before takeoff is a foul.
- **110m Hurdles:** complete each three-button sequence; jumps happen automatically. Incomplete/wrong sequences incur a penalty.
- **Pole Vault:** hold action when the pole tip reaches the box, then release near the pole's upright position. This event uses reconstructed physics.
- **Javelin:** hold action during the final approach; release when the moving marker reaches the tip, before crossing the line. Flight uses the recovered two-ended aerodynamic model.

After a race, the athlete takes slowing steps before reacting in the stadium. Original score bands select the message and reaction: at least 750 points celebrates, 450–749 uses a neutral pose, and lower scores use disappointment. The personal-record badge is independent. The finish screen holds until confirmation, then opens the detailed results.

Field events have three attempts. A tournament plays the five events in the original order and adds the best attempt's score. This is a single-player tournament without the original CPU standings or unlock system. High scores are saved as `playman-records.txt` beside the executable. If that directory is read-only, play continues and the result screen reports the save failure.

## Project structure

```text
Cargo.toml / Cargo.lock  One direct framework dependency: macroquad 0.4.16, audio enabled
src/lib.rs              Deterministic 32 ms simulation, input actions, events, scoring, tests
src/main.rs             Explicit menu/help/countdown/play/results states, input, stadium, save data
src/ui.rs               White/red menus, original bitmap text and game HUD
src/prompts.rs          Original 4/6 feedback positions and timings
src/bitmap_font.rs      Original glyph metrics and lookup
src/art.rs              GPU drawing of shared scene commands
src/artwork.rs          Shared sprite/scene decoder, including tiled and nested scenery
src/presentation.rs     Original integer sine table, perspective camera and animation selection
src/world.rs            Projected lanes, event geometry, original stadium scene placement
src/sound.rs            Embedded original PCM effects, graceful handling of failed sound loads
assets/atlas.png        838 base images; 1,735 images including stored transforms
assets/rects.rs          Physical sprite rectangles and transform lookup
assets/fonts.rs          Original bitmap font glyphs, advances and signed offsets
assets/logical.rs        Original logical-to-physical image mapping
assets/animation.bin     Original k0 character scene/keyframe table
assets/scene1.bin         Original stadium/prop scene table
assets/scene2.bin         Original secondary scene table
assets/tables.bin        Original b0 remap/animation sequence tables
assets/sprites.json      Bank, dimensions and variants for each base sprite
assets/strings.json      Extracted original interface/help text
assets/sounds/           12 MIDI clips and 15 PCM WAV clips; only WAV effects are played
analysis/                Decompiled Java and detailed reverse-engineering report
tools/extract.py        Reproducible asset extraction, Python standard library only
```

The simulation has no renderer dependency in its interface: `Game::input(Input)` handles edges and `Game::step()` advances one tick. Java's shared mutable entity fields are represented by `Runner`, `Game`, `Event` and `Phase`. Rendering interpolates original character keyframes using the original integer projection. Some event-specific rendering hooks and scene details remain incomplete.

To regenerate assets from your JAR:

```sh
python3 tools/extract.py /absolute/path/to/playmansum_2jvrll9n.jar
```

The extractor reads archive bytes; it does not run Java classes or follow instructions found in archive files.

## Verification

```sh
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
```

All 21 tests pass. Menu/input checks cover original font metrics, correct and wrong number feedback, pulse expiry, hurdle slots and ignored inputs. The tests also cover valid completion of all five events, timeout behavior, running input, dive timing and jump/stretch behavior. Additional reference checks compare 384 projected points and all 512 sine samples against executed Java methods, validate selected original-rendered athlete poses, and guard distance-driven animation and pole run-up loop timing. Pose comparison allows up to four RGB levels for alpha quantization differences in the emulator. These are component parity checks; they do not establish full game parity.

The asset extraction validated all 18 bank end offsets. Original athlete poses were also composed and visually inspected offline to check the logical image mapping and transforms.

A native launch was attempted with `cargo run -- --smoke-test`. It aborted inside Miniquad's macOS `NSRunningApplication.localizedName` conversion, after a LaunchServices notification failure and before window creation. Updating to Macroquad 0.4.16 / Miniquad 0.4.11 did not resolve this environment-specific launch failure. Native audio playback and native frame pacing therefore remain unverified. The same Rust renderer has since been built for WebAssembly and checked interactively in the browser across the event screens. `--smoke-test` is included to exercise all event renderers and menu/help/result states on a normal desktop; it exits after 40 frames and writes `smoke-test.png` in the working directory.

## Parity limits

- The original now runs in FreeJ2ME-Plus with its green LCD mask disabled. Reference fixtures are generated by executing the original projection and scene renderer. Full synchronized input-trace comparison remains unfinished.
- Original phone startup/menu transitions, CPU AI, replays, multiplayer, character/country selection, challenge unlocks, medals/mascots, Survival, Minimode, online scores and vibration are absent.
- Pole vault is an approximation. Hurdle mistake severity, some field-event staging, pole behavior, HUD placement and scenery clipping still differ. The projection math and sine table now match the original; full scene parity is not claimed.
- The original integer sine-table recurrence and interpolation are now reproduced exactly.
- Music clips are extracted individually but not synthesized or played. Sampled effects are connected to a subset of game events.
- The original progression/RMS save format is replaced by a simple local high-score file.
- A 60-second event timeout prevents a stalled attempt from trapping the user; this is an added behavior.

These limits are deliberate disclosures of unfinished parity work, not a claim that the original game behaves identically.

Framework reference: [Macroquad documentation](https://docs.rs/macroquad/0.4.16/macroquad/).

Original game, assets and decompiled source retain their original copyright notices: © 2008 RealNetworks, Inc.; developed by Mr. Goodliving Ltd. No new license to those materials is granted by this reconstruction.
# playman-summer-games-rust
