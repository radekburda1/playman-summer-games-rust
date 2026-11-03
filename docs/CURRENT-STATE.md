# Playman Rust desktop handoff — 2026-11-03

This snapshot records completed work and constraints, not authorization for new features. Inspect live files/diff before editing. Latest work extends the neck-revealing head anchor to all four custom male characters. Await the user's next task after initialization; do not invent cleanup work or commit.

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

HEAD at start of Tereza pass: `ed8e8db Add Burdas to the game`.
The checkout was clean before Tereza work. New code/assets and the hazel-eye,
longer-ponytail follow-up and all custom head neck corrections remain uncommitted. No staging/commits performed.

## Architecture to preserve

Simulation owns mechanics and logical audio; no graphics, platform, application, filesystem dependencies. App orchestrates a 32ms fixed tick, effects, menu flow, records, player roster and replays. Presentation computes immutable frames. Rendering consumes frames and pure artwork. Platform handles devices and storage. Tests enforce boundaries.

Appearance is independent of scoring, RNG and physics. Player names and appearances are currently in memory only (reload resets setup). Shared records persist. Do not make cosmetic character choices alter mechanics.

Singleplayer and local turn-based multiplayer (2–6 players) use setup; arrows select rows/face/count, typing edits name, Backspace erases, Enter advances. Race replays show previous athletes live and at finish. Measured race time freezes during original-style coast/brake run-out. Winner celebrates, loser recovers. Non-original running NICE!/WRONG BUTTON/DIVE! text was removed; finish feedback remains. Music stops during a run and returns after result confirmation. User confirmed music/effects and celebration audible; agent cannot independently hear browser audio.

## Current character implementation

`Face::COUNT=17`: original 12 at 0..11, default Playman index1; BROTHER=12, KUBAK=13, RADEK=14, BURDAS=15, TEREZA=16. Names editable independently; custom identity shown in setup footer. From default Playman: Left twice Tereza, three times Burdas, four times Radek, five times Kubak, six times Brother.

Original metadata maps sex/body bank, skin and head resources. Tereza uses original light-skinned female preset0; other custom faces use standard male metadata; Brother uniquely substitutes wider limbs and torso. Kubak, Radek and Burdas use original standard male limbs/uniform. Default original assets remain pixel-identical.

`src/assets/custom_character.rs`: virtual IDs BASE4096, COUNT55, CELL96, ATLAS_WIDTH2048, ORIGINAL_HEIGHT1249. Existing original atlas never rewritten. Runtime composite appends all custom sprites in all 8 transforms. GPU and software renderer share it.

Virtual indices relative to BASE:
- 0..3 brother head profile / three-quarter / front / rear; 4/5 transparent small/large portrait.
- 6..26 brother limbs from original male bank: arms/lower limb pieces1.45x; hip/shorts-bearing slots 16,23–26 use original widths.
- 27..30 brother torso lean-pose slots, sampling the original generated profile with shoulder/hip registration. Each96x96 including transparent padding. Map original uniform612..615.
- 31..34 Kubak heads; 35/36 transparent menu previews using the same neckless FRONT head (not opaque reference portrait).
- 37..40 Radek heads; 41/42 transparent menu previews using the same neckless FRONT head.
- 43..46 Burdas heads; 47/48 transparent menu previews from the frontal head.
- 49..52 Tereza heads; 53/54 transparent menu previews from the frontal head.

`src/rendering/atlas.rs` embeds assets and resizes alpha-bounded views with nearest-neighbor. Original bytes copied unchanged. Brother/Kubak head sheets use quarter-cell bounds. Torso and Radek views use transparent gutters near quarter boundaries before alpha bounding. This is crucial: generated torsos were unevenly spaced, and exact quarter cuts previously picked up neighboring strips and shifted visible centers.

`custom_character::torso_sample` maps unchanged calibrated shoulder/hip joints
into source landmarks (18%,17%)/(47%,94%). The bottom waistband attaches to the
shorts/leg joint. Front width scales 1.5x across the body axis, without extending
body length. The previous 70% anchor caused the belly to cover the legs and is removed.
Brother registration is baked into padded sprites before all eight transforms.
All five custom characters’ gameplay heads receive a 3px offset toward the top of the head,
rotated/reflected with each transform; portraits, torsos and other assets have zero offset.

Current brother heads: bald under black BACKWARD cap, full brown beard. All heads end at jaw/skull base with NO NECK. Rear cap brim remains backward. Kubak: swept light-brown hair, blue eyes, short beard. Radek: bald, blue eyes, light stubble, smiling.

## Male neck alignment — latest pass

Extended Tereza’s 3px head-axis lift to Brother, Kubak, Radek and Burdas.
All four were inspected live in the browser in starting stance and Dash running;
original male starting stance was also compared. Heads stay attached while
exposing the body neck; Brother’s torso registration is unchanged. Raster
assets and head-only portraits are unchanged. The transform/render-command
test now covers all five custom characters and excludes their portraits.
Screenshots: {brother,kubak,radek,burdas}-neck-{standing,running}.png and
original-male-neck-standing.png in the visualization directory below.
Native visuals and every event/pose were not individually inspected live.

## Tereza — prior pass

Added Face16, appending six sprites at BASE+49. Tereza uses the original female
pose/body/uniform preset0, not the male custom metadata. Heads are neckless and
transparent, with a compact brown ponytail tucked/curling beside the jaw like
the original women. Original female runtime heads790..793/806..809 were inspected.
User requested a slightly longer tail after approving v1, and clarified hazel
(brown/green) eyes. Final revision uses39x34,37x36,33x36,36x32 head rectangles;
face size is retained while giving the side curl a little more width.

Runtime tereza-heads.png is the hazel-eye/longer-tail revision. Initial sheet is
retained as tereza-heads-v1.png. Identity photo and three supplied hair/style
references are in assets/custom. Exact prompts/provenance are in its README.
Rear-turn replacement also covers Tereza. No changes to simulation or scoring.
Live browser selection and female start/Dash running were inspected. Hair fits
without clipping or extending onto shoulders. Screenshots tereza-selection.png
and tereza-running.png under
`/Users/Radek.Burda/.codex/visualizations/2026/11/02/01a19e70-a6c1-72f3-87ff-0d545c49e17f/`.
The original blonde head/body were also rendered separately: the visible neck
belongs to the female body, not the head asset. Tereza’s taller head covered it.
A 3px gameplay-only lift now exposes that existing neck; all eight transforms
are covered by an anchor/render-command test. No neck pixels were added.
Live browser starting stance and Dash running were inspected after the fix,
with screenshots tereza-neck-standing.png and tereza-neck-running.png in the
same visualization directory; original-female-neck-validation.png records the
head/body decomposition. Native visuals and all events were not checked live.

## Burdas — prior pass

Added Face15 (16 choices), six sprites at BASE+43, with standard male body and
unchanged mechanics. Generated burdas-heads.png from supplied identity photo and
original-character lineup style reference, both retained in assets/custom.
All four views are neckless; selector uses the transparent frontal head. The
existing rear-turn-to-three-quarter rule also applies to Burdas. New atlas views
use transparent-gutter segmentation; existing resource coordinates remain intact.
Exact prompt/provenance in assets/custom/README.md.

Live browser: selected Burdas (Left twice from default), edited name to BURDAS,
inspected transparent menu portrait and Dash running head/body fit. Screenshots:
`burdas-selection.png`, `burdas-running.png` under
`/Users/Radek.Burda/.codex/visualizations/2026/11/02/01a19e70-a6c1-72f3-87ff-0d545c49e17f/`.
Native visuals and every Burdas event were not individually checked live.

## Head turns and celebration shorts — prior pass

Finished validation/documentation of the previously interrupted head-turn change.
All custom faces (Brother/Kubak/Radek/Burdas/Tereza) replace original requested rear head view 3
with existing three-quarter view 1, via shared Face::custom_head used by direct
head draws and scene remapping. Original 12 mappings remain unchanged. Rear source
artwork is retained, but no prepared custom pose should draw its atlas cells.
Automated coverage checks all 318 poses ×5 custom faces and all rear transforms.

User's pointed black shape was traced in software celebration frames to widened
hip/upper-leg shorts parts, not the torso. Slots16 and 23–26 now retain original
widths. Arms/other limb pieces stay1.45x; torso and joint positions are unchanged.
A 40-frame software celebration contact sheet was inspected before/after; original
shorts silhouette remains, without the exaggerated horizontal stretch.

Live browser Dash checks: Brother8.34s and Radek8.21s, run-out and celebration.
Radek triggered HIGH SCORE and saved 8.21s; records were not reset. The high-score
overlay obscures part of his celebration sprite, so detailed pose inspection also
used the software renderer. Screenshots:
`brother-face-visible-celebration.png` and `radek-face-visible-celebration.png` in
`/Users/Radek.Burda/.codex/visualizations/2026/11/02/01a19e70-a6c1-72f3-87ff-0d545c49e17f/`.
Kubak head turns are covered by pose checks, not an additional live run. Native
visuals and all event-specific head turns remain unverified live.

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

Runtime files in repo assets/custom/: brother-heads.png, brother-torsos-shoulders.png, brother-reference.png (previous torso sheets retained), kubak-heads.png, kubak-reference.png, radek-heads.png, burdas-heads.png, burdas-reference.png, burdas-style-reference.png and README.md. Reference images retained for provenance; selectors now use generated transparent FRONT head for all custom people.

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

Current logs: /tmp/male-neck-tests.log, /tmp/male-neck-custom.log,
/tmp/male-neck-clippy.log, /tmp/male-neck-web.log, /tmp/male-neck-native.log.
All 84 tests passed, covering all 17 faces/318 poses, female body mapping,
transparent portraits, compact head bounds and custom rear-turn suppression.
Clippy, formatting/diff, web bundle/storage checks and web/macOS builds passed.
Native visuals were not rechecked.

Prior live verification: Radek transparent selector and Dash run with standard body. Screenshots outputs/radek-selection.jpg and outputs/radek-running.jpg. Original 12 + all custom all 318pose/resource checks automated; not all15 characters ×5 events visually inspected. Native custom-character visuals were not individually checked. Audio was user-confirmed earlier, not re-verified for custom faces. Browser settings/appearances reset on reload.

## Browser workflow

Use available Browser/CUA tool, not raw browser networking/CDP. IAB is requested/preferred for game testing. Current tabs from old turns may be stale; rebind existing localhost tab or create one. CUA often gives `browser.user is unavailable` after turn changes; reset CUA session and initialize with exactly one documented entry-point call, then read docs. Avoid repeated browser selection.

Canvas needs focus. Fresh AX image index changes after reload; click it before keypresses. Game menu Singleplayer default. Return→setup, Down→player1, Left twice selects Tereza from Playman. Four Returns from player row proceed through Continue/event/help to AwaitStart. `5` held5600ms covers countdown. Press4/6 to run.

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

## Commit preparation — 2026-11-03

Reviewed the pending Tereza addition and all custom neck-anchor changes together.
Cleaned the asset README heading, generalized the head regression diagnostic,
and covered both portrait sizes for all five custom faces. Retained generated
v1 and all supplied references as documented provenance. Focused custom tests,
formatting, diff whitespace and web bundle/storage checks passed after cleanup;
prior full-suite/build/live results above still apply. Nothing staged or committed.

## Next model behavior

Read current code, docs and diff first. Report initialization briefly and wait for next requested change. Tereza is the latest requested feature. If user requests commit prep, review all custom-character uncommitted work together, preserve assets and provenance, validate proportionately, provide commit message; don't commit unless explicitly asked. Older appended DEVELOPER/asset notes contain superseded iterations; current code plus latest sections are authoritative.
