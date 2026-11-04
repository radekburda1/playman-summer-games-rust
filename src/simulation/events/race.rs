use crate::simulation::Game;
use crate::simulation::audio::SoundCue;
use crate::simulation::discipline::RacePhase;
impl Game {
    pub(in crate::simulation) fn race_step(&mut self) {
        let event = self.event();
        let r = &mut self.runner;
        if event.dash() {
            if r.position < event.dive_start() {
                r.run(false)
            } else {
                r.move_by(0)
            }
            if r.position >= event.race_finish() {
                self.finish(Some(self.elapsed_ms * 100 / 1024))
            }
        } else {
            let old = r.position;
            if r.speed < 0 && self.elapsed_ms > 5000 {
                r.speed = 0;
            }
            if r.sequence_cursor < 3 {
                r.run(true)
            } else {
                r.move_by(0)
            }
            let offset = -117760;
            if (old + 327680 - offset) / 983040 < (r.position + 327680 - offset) / 983040
                && self.discipline.hurdles_mut().next_hurdle < 7
            {
                if r.sequence_cursor < 3 && r.mistakes == 0 {
                    r.mistakes = 1
                }
                let missed = r.mistakes > 0;
                if missed {
                    self.discipline.hurdles_mut().fallen |=
                        1 << self.discipline.hurdles_mut().next_hurdle
                }
                r.next_sequence();
                self.discipline.hurdles_mut().next_hurdle += 1;
                if self.discipline.hurdles_mut().next_hurdle >= 7 {
                    r.sequence_cursor = 3
                }
                self.sound_events.push(if r.previous_mistakes > 1 {
                    SoundCue::Failure
                } else {
                    SoundCue::Takeoff
                });
            }
            if self.runner.position >= 0x780000 {
                self.sound_events.push(SoundCue::RaceFinish);
                self.finish(Some(self.elapsed_ms * 100 / 1024))
            }
        }
    }
}

impl Game {
    pub(in crate::simulation) fn race_action(&mut self) {
        let min = self.event().dive_start();
        if self.runner.position >= min && self.runner.dive.is_none() {
            self.runner.dive = Some(self.runner.position);
            self.sound_events.push(SoundCue::Action);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Hurdles {
    pub(in crate::simulation) phase: RacePhase,
    pub next_hurdle: usize,
    pub fallen: u32,
}
