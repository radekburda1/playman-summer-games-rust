# Hotseat field summaries

Inspected the supplied original decompilation (`work/decompiled/d.java`), not an
inferred slideshow or a new physics implementation.

The original field events enter state 15 after the human turns in each round
(`d.aG`, cases 13/15/16). State 15 initializes a simultaneous replay of that
round's attempts, including fouls. States 16 (rounds 1/2) and 20 (round 3) show
results using the best mark so far: `d.z(16/20)` calls `d.aH`, which sorts slot 3,
and the table reads `d.l(player, 3)`. State 16 then clears logs, increments the
round and starts player zero. Final event results precede tournament totals
(state 21) where applicable.

The port follows this flow. Current-round recordings and each player's event
best are separate; a later foul never substitutes an earlier successful replay.
Singleplayer and race flows retain their existing behavior.

## Source mechanisms

- `d.bi` / `d.b` (around lines 9832–9927): Long Jump sorts by current result,
  reinitializes seeded athletes, resets input cursors, and consumes timestamped
  inputs while fast-forwarding to rank-staggered runway positions. Athletes get
  a slight depth separation. `d.bj` advances all participants with scoring disabled.
- `d.bv` / `d.d` (10797–10873) and `d.bw`: Javelin uses the same pattern, retaining
  action-down/up input distinctions. Fast-forward stops before the throw animation;
  it must not consume the complete flight when someone throws early.
- `d.bJ` / `d.f` (11989–12061) and `d.bK`: Pole Vault restores every athlete and
  replays their plant/release inputs together, with effects and separate poles.
- `d.bk/M` (10120 onward): Long Jump pans from the board to the leading landing
  distance over three seconds. `d.bx/Q` (11115 onward) frames the throwing line;
  `d.bL/m` (12499 onward) moves across and up/down around the vault over five seconds.

## Port

`application/field_replay.rs` retains initial state plus ordered inputs at the
32 ms simulation boundary. Keeping ordered inputs preserves multiple events
between ticks, including releases. Playback uses the same gameplay/effects
choreography as live attempts, in isolated cloned states. Its simulation and
recovery are bounded; drawing cannot step or mutate either timeline. The live
Javelin flight-skip shortcut is recorded but ignored while watching, allowing
the whole actual trajectory to play. Successful finish reactions are restored.

Summary cameras use the recovered paths. Display adaptations keep early
throws and fast landings visible, and high/inverted vault poses clear of the
desktop header and right edge. These change only the replay camera. All shared scenery uses one
camera; depth separation applies to actors and their equipment. Screen-space
effects are shifted into that view. The summary lists every current-round name/result and
supports Space for round results, R to rewind, Escape to leave. It honors Handicap
and emits no duplicated simulation audio. Event points are awarded once after round three, never on
round-one/two replays, rewinds or result-screen exits.

Overlap remains possible, as in the original. The port currently uses fixed
depth offsets and staggered starts; it does not reproduce the original randomized
depth variation or exact ranking-based draw order. These are known presentation
parity gaps, not changes to recorded attempts or simulation.

## Evidence

Focused tests replay original Pole Vault and Javelin input fixtures, Long Jump
jump/stretch/foul scenarios, and a real Javelin flight skip. Terminal mechanics
and effects match; only application finish metadata/audio draining differs.
Application tests cover 2/6 players, three disciplines/rounds, current-round selection,
fouls after successful rounds, best-so-far tables, next-round rotation, separate
tournament totals, replay reset/exit, score/record/RNG isolation and immutable repeated
rendering. Existing default state/pixel fixtures remain unchanged.

Live browser verification and final check results are in docs/CURRENT-STATE.md.
