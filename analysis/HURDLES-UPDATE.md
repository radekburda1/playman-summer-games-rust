# 110 m Hurdles source/oracle update

## Implemented

- Original 4/6 input gate: `(position + 327680) % 983040` must be in `[0,865280)`. Wrong inputs consume a slot; ignored inputs do not consume slots or generate new highlights.
- Sequence reset reproduces the original age-array penalty, speed recalculation and previous-mistake count. The previous mistake count selects clean/mild/medium/severe animation tables G/H/I/J at offsets 2866/2912/2958/3004 (23 keyframes, 175 distance units per keyframe).
- Original hurdle encounter threshold with offset -117760 and spacing 983040 simulation units. Incomplete sequences produce at least one mistake; the fallen bit is preserved. Count 1 uses sample 18; counts 2/3 use sample 22, matching Java rather than treating every fallen hurdle identically.
- Idle start after the original five-second grace period; 100 m dive gate; original 110 m finish timing and post-finish target/deceleration to 0x870000. Finish sample 13 is queued once.
- Original scene 16384 frame 0/1 replaces placeholder hurdles. All six lanes have seven hurdles at 10,25,40,55,70,85,100 world-metre positions. Both legs, white bar and black edging follow the original screen-coordinate drawing. Fallen visuals switch only after the runner passes the original visual threshold. Full-width sprint distance lines are no longer drawn in Hurdles.
- Waiting uses the original idle table (2378, five keyframes, 200 ms). Existing ready/set/go, starter gun, camera transitions, finish panels, performance feedback and independent personal-record flag are retained.

## Original source map

`aU()` allocates athletes; `aY()` initializes the countdown; `aZ()` advances timers and the race; `a(c,boolean)` advances a hurdle runner; `c(c,int,boolean)` gates input. `a(c)` resets sequences and penalties. `b(c,int)` and `e(c)` implement run-out/deceleration. `ba()` chooses camera/athlete animation; `H(int)` places stadium and finish equipment; `J(int)` places six-lane markings and hurdles; `a(int,int,int,int,boolean)` draws upright/fallen hurdle bars. Existing score mapping `r(int)` is unchanged.

## Executed evidence

Run `outputs/reference-runner/hurdles-oracle.sh` from the workspace, or `./hurdles-oracle.sh` from the runner folder. It loads the unmodified JAR into isolated FreeJ2ME and executes original methods with seed 12345, one athlete and 32 ms ticks. Cases: clean, wrong, incomplete, boundary, dive, late-dive and idle. It also exports waiting/ready/set world frames. Rendering reports zero errors.

The Rust replay tests match **3,726 ticks** of position, speed, sequence index, current/previous mistakes, fallen mask, dive start, all three next-sequence values and all four camera parameters. Measured finish times also match. Example controlled results: clean 11.34 s; incomplete 14.90 s; wrong 15.37 s; idle 20.25 s. These are controlled input strategies, not claims about optimal human performance.

## Reproducing and scope

`tests/fixtures/hurdles/provenance.json` records the source/JAR/trace hashes.
The fixture generator is preserved in `tools/oracle/HurdlesOracle.java`; it uses
FreeJ2ME's mobile classes. The companion reference runner's `hurdles-oracle.sh`
compiles and executes it against the original JAR with seed 12345 and 32 ms ticks.
Athlete appearance is normalized for frame comparison; game equations are unmodified.

`make compare` renders the same input traces with the shared Rust world renderer.
Generated replay PNGs and comparison sheets are ignored by Git. Tests retain
12 original asset fixtures and four original world frames. Asset comparisons allow
four RGB levels for emulator alpha quantization; world comparisons additionally
allow at most 100 differing pixels out of 76,800. The complete replay comparison
is unmasked, so Java's GO/input overlays absent from the Rust world-only harness
appear as differences.

Numerical fixtures, rendering tests, strict Clippy and native/WASM release builds
pass. These checks do not prove complete visual/audio parity: live GPU rendering,
start/finish UI choreography and audible timing/mixing still need verification.
Music uses Gervill renders of the original MIDI resources; see
`tools/audio/README.md`. CPU opponents and challenge/profile progression remain
outside the single-player implementation.
