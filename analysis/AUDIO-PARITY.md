# Audio parity audit — 2026-11-02

A first executable-reference audit found and corrected the wrong 100m Dash finish
sound. This is a cue/asset audit, not a claim of full audible parity.

## Executed original reference

The unmodified supplied Java JAR was loaded in a separate FreeJ2ME process. After
initialization, its `Player` objects were replaced with observers that log calls
to `start()`. Original gameplay methods were invoked at controlled states; neither
the JAR nor the live reference viewer was modified. A fresh emulator working
directory isolates profiles. Repeated oracle runs produced identical CSV output. Setup and SHA-256 hashes are in `tests/fixtures/audio/provenance.json`.

| Scenario | Original resource IDs | Original tick indices |
|---|---|---|
| Dash countdown | 23, 24, 25 | 0, 94, 157 |
| Dash finish crossing | 13 | 0 |
| Long Jump takeoff/stretch/landing | 18, 20, 26 | 0, 19, 52 |
| Long Jump early stretch/failure landing | 18, 22, 22 | 0, 3, 37 |
| Long Jump foul takeoff | 22 | 0 |

Each tick is 32 ms. Rust's selected Long Jump scenarios match the resource IDs and
tick indices exactly. The application-level countdown has the same three cue IDs
and order within one 32 ms display tick after alignment to Ready; this is not
sample-accurate output timing. Application timers use display frames while the
original uses integer ticks.

The new Dash finish test failed before the correction (`[26]` versus original
`[13]`). `App::update` now requests `SoundCue::RaceFinish` (13), rather than the
landing cue (26), for a successful Dash finish. The test then passed. Timeout
failure handling remains unchanged. No physics, original asset bytes or existing
oracle expected values were modified. The rebuilt browser completed a Dash at
12.96s; this is a runtime smoke check, not a recording or listening confirmation
of the corrected cue. That check exposed an independent upstream JavaScript error:
the bundled quad_net registration assigned `register_plugin` without declaring it
under strict mode. Staging now gives that binding local scope, with a Node VM
regression test. Script URLs are content-versioned as well as WASM. A fresh browser
tab rendered the menu with no captured warnings/errors after this fix.

## Result celebration and music-loading fixes

An additional executed probe calls the original result queue in base mode (`aJ=0`)
and advances it in 32 ms ticks. `result-cues.csv` records:

| Result | Resource IDs | Tick indices |
|---|---|---|
| Good time | 12, 14 | 0, 12 |
| Poor time | 12 | 0 |
| Good time and record | 12, 14, 8 | 0, 12, 27 |

Rust now requests result cue 12 and, for at least 750 points, celebration cue 14.
For races these requests start after run-out, when the reaction is ready. They are
emitted once; the record jingle follows when applicable. The platform queue uses
the original sound-table delays of 373 ms and 460 ms rather than the WAV lengths.
Those queue delays intentionally allow sounds to overlap. A unit test reproduces
the original player-start tick sequences; application tests cover good/poor/record
outcomes, run-out gating and duplicate suppression. Other original mode variants
that select resources 15 or 16 are outside this base-mode implementation.

The menu flash was traced to awaiting `load_sound_from_bytes` inside the main
render-loop future. On WASM that function awaits `next_frame` while decoding,
advancing undrawn frames. Music decoding now runs in a Macroquad coroutine while
the main loop keeps rendering. Pending decodes are canceled when a new attempt
starts, and jingle duration is measured from actual playback readiness. Music is
not predecoded in bulk. The rebuilt browser completed a 9.12s Dash and returned
to the menu. The user confirmed the celebration was audible after run-out.
Screenshot sampling is not a frame-by-frame flicker or audio capture.

## Asset and music audit

All 27 extracted MIDI/WAV resources match their corresponding bytes in the
original JAR. All 12 shipped OGG files and MIDI inputs match the existing music
provenance hashes. This confirms the inputs and shipped versions; it does not
prove matching output from different synthesizers.

`MidiAudit` measured all twelve MIDI sequence durations. The existing Gervill WAV
renders are two seconds longer than their respective sequence durations, within
sample rounding. The background tracks 1–3 each contain 221.538s of MIDI timeline
and are rendered as 223.538s files. Rust loops the whole OGG file, so its loop period
includes that added tail.

The final three seconds of the source WAV renders were measured in 20 ms RMS
windows at -60 dBFS. For background tracks 1, 2 and 3, the terminal interval below
that threshold was approximately 0.86s, 0.62s and 1.06s respectively. These are
measurements of the offline Gervill source renders, not a comparison with captured
FreeJ2ME/browser output. No automatic trimming was applied: cutting tails without
checking the reference loop and note releases could introduce a different defect.

The WAV effects are original samples, while music is lossy Vorbis encoded from
Gervill General MIDI renders with reverb and chorus. Live instrument selection,
envelopes and mixing cannot be inferred from resource equality. Rust applies
music/effect gains of 0.45/0.55; these have not been calibrated against actual
reference output or handset playback.

## Background music lifecycle correction

The original stops background music before an attempt and resumes it after result
confirmation. Source state entry `z(10)` calls `b(true)` to stop current music
before the start prompt; result/menu paths request background
tracks separately. Rust previously called its background scheduler unconditionally
every frame, allowing music to continue through gameplay.

`App::background_music()` now requests no background track for AwaitStart, Ready,
Play (including pause), or Finish. Menu/help/reset, confirmed race Results and
Complete permit it. The platform stops the active menu/intro sound on attempt
entry and prevents automatic background fallback while the attempt is active.
Confirming the finish and completing its transition permits background playback
again. Starting/retrying an attempt stops it again. Explicit record jingles and
sampled gameplay effects remain separate, matching the original's separate cue
requests. This policy is covered across all five disciplines by an application-flow
test, including the finish transition and paused gameplay. Actual speaker-output
transition timing remains a listening/recording check.

## Validation and remaining coverage

The new cue tests, complete Rust test suite, strict Clippy, staged JavaScript VM
test, storage tests, native release/app staging and WASM build pass.
The repeated Java oracle agrees with the retained fixture.

The user previously confirmed browser music/effects are audible. No system-audio
recording capability is exposed by the current tools. Consequently no cross-output
waveform correlation, onset-latency measurement, loudness calibration or instrument
listening comparison has been performed. Original and reconstructed gameplay
recordings are still required for those checks.

The selected executed probes do not cover every Hurdles, Pole Vault or Javelin
cue, all menu/result music selections and interruptions, simultaneous-sound mixing
or original phone hardware. Selected result/record queue requests are covered
above; their actual mixed output still needs recording. Existing numerical/visual
tests are not evidence for these uncovered audio paths. See `tools/audio/README.md` for reproduction.
