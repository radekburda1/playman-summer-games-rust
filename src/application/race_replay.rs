//! Display-only race recordings, sampled after each fixed simulation tick.
//! Replays never feed inputs, sounds, records or random numbers into the live game.
use crate::{presentation::Animation, simulation::Game};

#[derive(Clone, Debug)]
pub struct RaceSample {
    pub position: i32,
    pub height: i32,
    pub animation: Animation,
    pub stopped: bool,
    pub reaction_ms: i32,
    pub fallen: u32,
}
impl RaceSample {
    pub fn capture(game: &Game) -> Self {
        Self {
            position: game.runner.position,
            height: game.runner.height,
            animation: Animation::for_game(game),
            stopped: game.finished && game.runner.speed < 4000,
            reaction_ms: game.phase_elapsed_ms,
            fallen: game
                .discipline()
                .as_hurdles()
                .map_or(0, |state| state.fallen),
        }
    }
}
#[derive(Default)]
pub struct RaceReplay {
    pub samples: Vec<RaceSample>,
    pub result: Option<i32>,
    pub crossing: Option<(usize, Game)>,
}
impl RaceReplay {
    /// Re-run only the post-measurement motion at the replay lane's target.
    /// Gameplay samples (and thus the measured race) remain untouched.
    pub fn finish_at(&mut self, target: i32) {
        let Some((tick, mut game)) = self.crossing.take() else {
            return;
        };
        if !game.finished {
            return;
        }
        self.samples.truncate(tick + 1);
        for _ in 0..2048 {
            game.step_race_finish_to(target);
            self.samples.push(RaceSample::capture(&game));
            if game.runner.speed < 0 {
                break;
            }
        }
    }

    pub fn sample(&self, tick: usize) -> Option<(&RaceSample, i32)> {
        let last = self.samples.len().checked_sub(1)?;
        Some((
            &self.samples[tick.min(last)],
            tick.saturating_sub(last) as i32 * 32,
        ))
    }
}

/// Original d.h hash and race stop targets (projection units shifted by 8).
pub fn stop_target(seed: u64, player: usize, active: bool, hurdles: bool) -> i32 {
    let position = if active {
        29440
    } else {
        let n = (seed as i32).wrapping_add(813387i32.wrapping_mul(player as i32));
        let n = n.wrapping_shl(13) ^ n;
        let hash = n
            .wrapping_mul(n.wrapping_mul(n).wrapping_mul(15731).wrapping_add(789221))
            .wrapping_add(1376312589)
            & i32::MAX;
        28672 + hash % 1536
    };
    (position + if hurdles { 5120 } else { 0 }) << 8
}

/// Preserve original stop spacing, translating the entire finish area for 400m.
pub fn stop_target_for_event(
    seed: u64,
    player: usize,
    active: bool,
    event: crate::simulation::Event,
) -> i32 {
    stop_target(seed, player, active, false) + (event.race_shift() << 8)
}
