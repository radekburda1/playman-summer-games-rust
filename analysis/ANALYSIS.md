# Reverse-engineering notes

This is the initial analysis. See [RENDERING-UPDATE.md](RENDERING-UPDATE.md) for the subsequent executed-original comparisons and corrected renderer.

## Input and method

Input: `playmansum_2jvrll9n.jar`, 865,443 bytes. Manifest identifies Playman Summer Games 3 1.0.21, CLDC 1.1 / MIDP 2.0, main MIDlet `S`, 240 × 320 logical display. No original executable was run.

The archive was extracted and inspected as data. CFR 0.152, running under a downloaded Temurin Java 21 runtime, produced the included Java files. The Java ME / Nokia APIs were unavailable; CFR nevertheless recovered most code. `d.java` contains warnings for four methods (`a()`, one string helper, `W()`, `af()`). Duplicate field names in the bytecode make these decompiled files unsuitable for direct recompilation. They are analysis evidence, not the Rust implementation.

All bundled strings, manifests and ancillary text were treated as untrusted game content, not as task instructions. No bundled URL was opened and no score/network action was performed.

## Classes and architecture

| Class | Observed responsibility |
|---|---|
| S | MIDlet startup, pause, teardown and display ownership |
| d | Canvas/game thread, input queue, renderer, resources, menus, all event simulations, storage |
| c | Shared event/athlete state: position, velocity, random sequence, jump/projectile/pole state |
| e | Player/profile and performance data |
| a | Named/numbered configuration record |
| b | Particle/effect record |

`d.e()` (boolean, near the end of d.java) clamps elapsed time to 125 ms, accumulates it, and simulates fixed 32 ms ticks. This is 31.25 updates per second. Rendering uses a separate 240 × 320 coordinate system. Input callbacks enqueue press/release events; main-thread processing dispatches them according to menu/game state.

The Rust implementation mirrors the fixed tick using an accumulator and converts queued action edges into explicit `Input` variants. It uses composition and enums rather than Java inheritance or globally aliased fields.

## Coordinates and collision

This is not a tile/grid game. Event state uses integer/fixed-point world coordinates and event-specific scales; `d.l(...)` and related helpers project into the display.

- Dash finish: `0x640000` (100 × 65,536); prompt-based acceleration ends at `0x500000`. World x is effectively 16.16 along this track.
- Hurdles finish: `0x780000`; sequence boundaries use a 983,040-unit repeat with offset −117,760. The event is labelled 110m even though its internal track is 120 coordinate units. Treating every coordinate unit as a physical meter would be wrong.
- Long jump run-up starts at −3,889,152, approach begins at −1,048,576, takeoff board is x=0. Landing distance in hundredths of a meter is `x*100/106496`.
- Javelin run-up starts at −4,587,520; charging becomes available at −1,966,080. Landing distance is `projectile_x*100/88576`.
- Pole vault run-up starts at −3,932,160. The original pole is a two-ended constrained model with angular/spring behavior; the clearance height is accumulated near x=118,000..138,000 and scored using `/855`.

Collision is mainly event-specific crossing, height and state tests, rather than generic hitboxes: runway line crossing, landing below ground, hurdle sequence boundary, pole/box contact and clearance sampling.

## Running model translated

See Java `a(c,int,boolean)`, `a(c)`, `c(c)`, `a(c,int)`, around the shared event helpers.

Ten input-age samples begin at 22. A correct press shifts the history and inserts zero; a wrong press fills the oldest available samples by a 50-unit penalty. Ages increase as ticks advance. Speed is computed from the sample sum:

```text
speed = 24000 - 8000*(sum - 10*base)/(10*(22-base))
base = 8 (normal), 4 (hurdles)
```

The next requested side is generated with a 48-bit Java-style LCG (`25214903917*x+11`, masked to 48 bits). It is not just strict alternation. Hurdles use three-button sequences, a mistake counter and boundary penalties. The original also has rapid-input detection; the reconstruction retains a compact version of it, with potential behavioral differences in unusually fast input.

Diving stores the start position. Each tick adds +3,900 position units while within 409,600 units of that point; afterward the contribution is −3,900. Timing therefore matters.

## Long jump translated

Java `b(c,boolean)` and `d(c,int,boolean)`:

- Takeoff vertical speed: `min(13000,4000+speed*300/1000)`.
- Horizontal takeoff speed: `16000+(speed-16000)/2`.
- Gravity per tick: 650.
- Without a successful stretch, descending flight loses 600 horizontal speed per tick.
- Stretch is accepted when `vertical-650 <= 0`; it adds `max(0,8000-abs(vertical)/4)` vertical speed.
- An earlier second press marks a bad stretch.
- Takeoff beyond x=0 is foul. Distance is recorded at first landing, before the cosmetic landing animation.

## Javelin translated

Java `f/g/d(c,boolean)` and `c(c,boolean)`:

The hold duration drives a triangular visual marker through `phase_time*60 & 0x1ffff`. Release creates a two-ended rod with position, velocity, orientation and angular velocity. Aerodynamic forces depend on perpendicular velocity at the two ends, unequal lever weights (192 and 320), gravity terms −428 and −256, and angular damping of 247/256. Landing is interpolated between the rod endpoints. Rust uses i64 intermediates corresponding to Java long calculations.

The reconstruction uses a 300 ms throw travel / 400 ms transition instead of deriving every staging duration from the original animation arrays. This can change foul thresholds slightly. Recovered sine lookup behavior is approximated with rounded mathematical samples.

## Asset formats

| Resource | Format and extraction |
|---|---|
| p | 8-bit palette count; each palette has 8-bit size and RGB565 shorts for colors 1..n; index 0 transparent |
| g0..g17 | Repeated big-endian u16 width, u16 height, u8 palette index, raw pixel payload, u8 transform count in low nibble, transform bytes |
| i | 938 bytes, 1,778 big-endian shorts, 324 big-endian ints: image mappings, banks, sound offsets and other config |
| b0 | Big-endian numeric/animation arrays at offsets referenced in d.java |
| k0 | 34,433 big-endian shorts: multipart character scene/keyframe table |
| k1,k2 | Other scene/font/UI tables; not interpreted by the reconstruction |
| l | Java-style length-prefixed UTF strings, including event instructions |
| s | Packed audio bank, not one MIDI file: 12 MIDI clips plus 15 8-bit mono PCM WAV clips |
| i.png | 16 × 16 MIDlet icon |

A palette index of 255 denotes raw BGRA data. Other pixels are palette indices. The extractor converts RGB565 by bit replication and writes RGBA PNG with Python's standard `struct` and `zlib` modules.

`i` shorts 355..373 delimit all 18 physical image banks. Shorts 374..1211 map 838 logical sprite IDs onto 1,735 physical slots, including transforms. This logical mapping is essential: a scene's logical image 794 is not physical slot 794. Every bank's parsed end was checked against the next delimiter.

Transforms use the original custom 0..7 ordering. The extractor materializes all stored rotations/reflections into the atlas, avoiding renderer-specific transform conventions.

Character scene 0 contains 318 frames. Commands include sprite, rectangle, tiled image, nested scene and drawing hooks, with bitmasks selecting per-frame values. Rust interprets the sprite/rectangle portion and interpolates coordinates between keyframes; unsupported hooks are omitted. For sprite selection, it uses the nearest keyframe as the original does. Original character keyframe sequences are read from b0.

Audio metadata is read from i's int table: offsets at 3+n, lengths at 65+n, type bytes at 907+n. Files are split at those exact bounds rather than by searching for magic signatures. Original PCM effects are embedded and connected to countdown, feedback and result states. MIDI synthesis is not supplied.

## Verification and next parity work

Asset bank boundaries and transformed pose assembly were checked. `cargo test`, warning-free Clippy and optimized compilation were performed; the README records the native launch failure. No Java ME emulator/reference playthrough was available, so identical behavior is not established.

For a complete port, the remaining major work is: replace the approximate pole solver with the original constrained model; port all scene drawing hooks/projection and appearance remaps; translate AI/replay simulation and original competition progression; add Survival/Minimode, multiplayer/profile setup, MIDI playback and original persistence. Then record identical input-edge traces against an emulator and compare integer states/results tick by tick, followed by visual/audio checks.

Input SHA-256: `abeba23badbf5e937b806c7e24165420c7dde08301c694f798b2d3c12f16962b`.
