# Playman Rust desktop handoff — 2026-11-03

This snapshot records completed work and constraints, not authorization for new features. Inspect live files/diff before editing. Latest work corrects the brother torso artwork and lean-angle mapping. Await the user's next task after initialization; do not invent cleanup work or commit.

## Start here

- Reuse checkout `/Users/Radek.Burda/Documents/Codex/2026-10-25/you-are-an-expert-systems-engineer/outputs/playman-rust`.
- Read applicable AGENTS.md, repository DEVELOPER.md and README.md, `git status`, `git diff`, and untracked files before edits. Root AGENTS.md now provides durable startup instructions and points to this document.
- Older historical handoff: `/Users/Radek.Burda/Documents/Codex/2026-10-25/you-are-an-expert-systems-engineer/work/DESKTOP-HANDOFF.md`. This repository-local document supersedes its current-state claims and the earlier chat-local TAKEOVER-HANDOFF.md copy.
- Preserve ALL uncommitted work. Do not stage, commit, push, reset or regenerate reference fixtures unless explicitly requested. Earlier requests to prepare commits meant cleanup and suggested messages; the user commits manually.
- Be autonomous on requested work, concise, and distinguish live verification from tests and remaining limitations. Do not repeatedly request confirmation for reversible implementation choices.
- No subagents unless user or applicable instructions explicitly request delegation.

## Project / reference

Rust/Macroquad reconstruction of Playman Summer Games 3. Browser game `http://127.0.0.1:8767/`; original Java reference viewer `http://127.0.0.1:8765/` may need restarting. Original JAR `/Users/Radek.Burda/Downloads/playmansum_2jvrll9n.jar`; decompiled source workspace `work/decompiled/d.java`; runner `outputs/reference-runner`.

Workspace root `/Users/Radek.Burda/Documents/Codex/2026-10-25/you-are-an-expert-systems-engineer`. Current chat artifact folder `/Users/Radek.Burda/Documents/Codex/2026-11-01/new-chat/outputs`.

HEAD at this pass: `a22fe71 Add custom character implementation and rendering support`.
Earlier commits: `ec8f51d` race replay/finish fixes, `1acdf96` local multiplayer, `ce51845` audio fixes. Custom brother/Kubak/Radek implementation was committed before this pass. The checkout was clean on initialization.

This pass modifies brother torso atlas composition, dimensions, regression tests, documentation and the staged web content hash. Runtime uses new assets/custom/brother-torsos-shoulders.png; the previous sheets are retained. No commits or staging performed.

## Architecture to preserve

Simulation owns mechanics and logical audio; no graphics, platform, application, filesystem dependencies. App orchestrates a 32ms fixed tick, effects, menu flow, records, player roster and replays. Presentation computes immutable frames. Rendering consumes frames and pure artwork. Platform handles devices and storage. Tests enforce boundaries.

Appearance is independent of scoring, RNG and physics. Player names and appearances are currently in memory only (reload resets setup). Shared records persist. Do not make cosmetic character choices alter mechanics.

Singleplayer and local turn-based multiplayer (2–6 players) use setup; arrows select rows/face/count, typing edits name, Backspace erases, Enter advances. Race replays show previous athletes live and at finish. Measured race time freezes during original-style coast/brake run-out. Winner celebrates, loser recovers. Non-original running NICE!/WRONG BUTTON/DIVE! text was removed; finish feedback remains. Music stops during a run and returns after result confirmation. User confirmed music/effects and celebration audible; agent cannot independently hear browser audio.

## Current character implementation

`Face::COUNT=15`: original 12 at 0..11, default Playman index1; BROTHER=12, KUBAK=13, RADEK=14. Names editable independently; custom identity shown in setup footer. From default Playman: Left twice Radek, three times Kubak, four times Brother.

Original metadata maps sex/body bank, skin and head resources. All custom faces use standard male metadata; Brother uniquely substitutes wider limbs and torso. Kubak and Radek use original standard male limbs/uniform. Default original assets remain pixel-identical.

`src/assets/custom_character.rs`: virtual IDs BASE4096, COUNT43, CELL96, ATLAS_WIDTH2048, ORIGINAL_HEIGHT1249. Existing original atlas never rewritten. Runtime composite appends all custom sprites in all 8 transforms. GPU and software renderer share it.

Virtual indices relative to BASE:
- 0..3 brother head profile / three-quarter / front / rear; 4/5 transparent small/large portrait.
- 6..26 brother limbs from original male bank (1.45x width).
- 27..30 brother torso lean-pose slots, sampling the original generated profile with shoulder/hip registration. Each96x96 including transparent padding. Map original uniform612..615.
- 31..34 Kubak heads; 35/36 transparent menu previews using the same neckless FRONT head (not opaque reference portrait).
- 37..40 Radek heads; 41/42 transparent menu previews using the same neckless FRONT head.

`src/rendering/atlas.rs` embeds assets and resizes alpha-bounded views with nearest-neighbor. Original bytes copied unchanged. Brother/Kubak head sheets use quarter-cell bounds. Torso and Radek views use transparent gutters near quarter boundaries before alpha bounding. This is crucial: generated torsos were unevenly spaced, and exact quarter cuts previously picked up neighboring strips and shifted visible centers.

`custom_character::torso_sample` maps unchanged calibrated shoulder/hip joints
into source landmarks (18%,17%)/(47%,94%). The bottom waistband attaches to the
shorts/leg joint. Front width scales 1.5x across the body axis, without extending
body length. The previous 70% anchor caused the belly to cover the legs and is removed.
`anchor_offset` returns zero; registration is baked into the padded sprites
before all eight transforms.

Current brother heads: bald under black BACKWARD cap, full brown beard. All heads end at jaw/skull base with NO NECK. Rear cap brim remains backward. Kubak: swept light-brown hair, blue eyes, short beard. Radek: bald, blue eyes, light stubble, smiling.

## Brother shoulder correction — current pass

User accepted the leg fit but reported the upper torso not fitting shoulders.
Measured the old enclosed armhole center at 25.5%/17.35% in alpha-bounded space;
previous guessed registration was 28%/22%. The tall enclosed shoulder loop also
protruded. Built-in image_gen edited only the upper torso using the second sheet
(brother-torsos-v2.png) as an open-shoulder reference, retaining low-belly artwork.
Runtime source is brother-torsos-shoulders.png, with the shallow open socket at
18%/17% and unchanged bottom waistband at47%/94%. Front width 1.5x; no leg extension.
All old generated sheets remain. Exact prompt/provenance in assets/custom/README.md.

Browser standing and running inspected. Screenshots in
`/Users/Radek.Burda/.codex/visualizations/2026/11/02/01a19e70-a6c1-72f3-87ff-0d545c49e17f/`:
`brother-shoulder-standing.png`, `brother-shoulder-running.png`.
New test checks transparent room at all four arm joints; hem-to-leg regression
and transformed clipping checks remain. Native visuals and complete field-event
sequences were not rechecked.

## Assets / generation

Runtime files in repo assets/custom/: brother-heads.png, brother-torsos-shoulders.png, brother-reference.png (previous torso sheets retained), kubak-heads.png, kubak-reference.png, radek-heads.png and README.md. Reference images retained for provenance; selectors now use generated transparent FRONT head for all custom people.

Built-in image_gen was used, transparent_background=true; no API key or fallback CLI. Skill `/Users/Radek.Burda/.codex/skills/.system/imagegen/SKILL.md`. Use that skill for further raster generation/edits. Inspect local images before editing. Runtime assets copied into repo, never reference only CODEX_HOME generated files. Exact prompts/provenance in assets/custom/README.md and current chat outputs/*-notes.md. Preserve bald/backward-cap corrections and NO NECK requirements.

Latest Radek generated source `/Users/Radek.Burda/.codex/generated_images/01a1994b-1b5d-7cb0-9e11-95147c99ecb5/exec-340695d4-1a9d-407d-8e84-03249125ab08.png`, copied to assets/custom/radek-heads.png and outputs/radek-heads.png. Reference `/var/folders/mc/6p3jqxn56_99_xrd6t1qftbh0000gn/T/codex-clipboard-e34cdca7-b6bd-41a4-a5ff-63c0c7497a68.png` (may be ephemeral).

## Verification / commands

Prior Radek pass: all76 tests passed, Clippy with warnings denied, formatting and diff checks, web bundle/storage checks, web and macOS release builds passed. Tests cover all faces/poses/resource transforms, unchanged original pixels, standard body mapping for Kubak/Radek, independent appearance, menu transparency, torso strip regression and transformed anchors.

Commands from repo:
```
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
git diff --check
node tests/web_bundle.cjs
node tests/web_records.cjs
make web-build
make mac-app
```
Ignored .make.local sets existing wasm toolchain/cache (Cargo1.90); preserve it. Native app output dist/Playman Rust.app. Avoid direct agent launch of native executable (OpenGL environment failures); use LaunchServices if needed. Do not discard native records.

Current logs: /tmp/shoulder-tests.log, /tmp/shoulder-custom.log,
/tmp/shoulder-clippy.log, /tmp/shoulder-web.log, /tmp/shoulder-native.log.
All 79 tests passed, including arm-socket clearance, joint fit, hem-to-leg,
transformed clipping and original-pixel preservation checks. Clippy, formatting,
diff checks, web bundle/storage and web/macOS release builds passed. Standing
and running were observed in the browser. Native visuals were not rechecked.

Prior live verification: Radek transparent selector and Dash run with standard body. Screenshots outputs/radek-selection.jpg and outputs/radek-running.jpg. Original12 + all custom all318pose/resource checks automated; not all15 characters ×5 events visually inspected. Native custom-character visuals were not individually checked. Audio was user-confirmed earlier, not re-verified for custom faces. Browser settings/appearances reset on reload.

## Browser workflow

Use available Browser/CUA tool, not raw browser networking/CDP. IAB is requested/preferred for game testing. Current tabs from old turns may be stale; rebind existing localhost tab or create one. CUA often gives `browser.user is unavailable` after turn changes; reset CUA session and initialize with exactly one documented entry-point call, then read docs. Avoid repeated browser selection.

Canvas needs focus. Fresh AX image index changes after reload; click it before keypresses. Game menu Singleplayer default. Return→setup, Down→player1, Left twice selects Radek from Playman. Four Returns from player row proceed through Continue/event/help to AwaitStart. `5` held5600ms covers countdown. Press4/6 to run.

For reproducible first Dash start input sequence, seed1034:
```
let expected=(1034>>6)&1, previous=(1034>>4)&1;
let rng=(1034n^0x5DEECE66Dn)&((1n<<48n)-1n);
for(let i=0;i<12;i++){
  await tab.pressKey(null,expected===0?'4':'6',{durationMs:100});
  if(expected===previous) expected=1-expected;
  else { previous=expected; rng=(rng*25214903917n+11n)&((1n<<48n)-1n); expected=Number((rng>>5n)&1n); }
}
```
First screenshot after12 is running. For completion use roughly120 inputs; tool gaps let time advance, so don't claim timing parity from wall-clock automation. Save screenshot with supported API, embed absolute image path in final. Mark deliverable only if user should retain tab; otherwise temporary tabs clean up.

## Next model behavior

Read current code, docs and diff first. Report initialization briefly and wait for next requested change. No outstanding required Radek feature work. If user requests commit prep, review all custom-character uncommitted work together, preserve assets and provenance, validate proportionately, provide commit message; don't commit unless explicitly asked. Older appended DEVELOPER/asset notes contain superseded iterations; current code plus latest sections are authoritative.
