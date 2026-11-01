use crate::simulation::audio::SoundCue;
use crate::simulation::discipline::RacePhase;
use crate::{simulation::Event, simulation::Game};
impl Game {
    pub(in crate::simulation) fn race_step(&mut self) {
        let event = self.event();
        let r = &mut self.runner;
        if event == Event::Dash {
            if r.position < 0x500000 {
                r.run(false)
            } else {
                r.move_by(0)
            }
            if r.position >= 0x640000 {
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
        let min = if self.event() == Event::Dash {
            0x500000
        } else {
            0x640000
        };
        if self.runner.position >= min && self.runner.dive.is_none() {
            self.runner.dive = Some(self.runner.position);
            if self.event() == Event::Hurdles {
                self.sound_events.push(SoundCue::Action);
            } else {
                self.message("DIVE!");
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Hurdles {
    pub(in crate::simulation) phase: RacePhase,
    pub next_hurdle: usize,
    pub fallen: u32,
}
