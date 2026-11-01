use crate::simulation::audio::SoundCue;
use crate::simulation::discipline::LongJumpPhase;
use crate::{simulation::Game, simulation::Phase};
impl Game {
    pub(in crate::simulation) fn jump_step(&mut self) {
        match self.phase() {
            Phase::Prepare => {
                if self.phase_elapsed_ms >= 1200 {
                    self.transition(Phase::Run);
                    self.phase_elapsed_ms = 0;
                    self.runner.speed = 0;
                }
            }
            Phase::Run => {
                self.runner.run(false);
                if self.runner.position >= -1048576 {
                    self.transition(Phase::Approach);
                    self.phase_elapsed_ms = 0;
                }
            }
            Phase::Approach => {
                self.runner.move_by(0);
                if self.runner.position >= 131072 {
                    self.transition(Phase::FoulRun);
                    self.phase_elapsed_ms = 0;
                    self.sound_events.push(SoundCue::Failure);
                    self.message("FOUL");
                }
            }
            Phase::FoulRun => {
                self.runner.move_by(0);
                if self.phase_elapsed_ms > 2000 {
                    self.finish(None);
                }
            }
            Phase::Air | Phase::Stretch | Phase::BadStretch => {
                if self.phase() == Phase::BadStretch
                    || self.phase() == Phase::Air && self.runner.vertical_velocity <= 0
                {
                    self.runner.speed = (self.runner.speed - 600).max(0)
                }
                self.runner.move_by(650);
                if self.runner.height < 0 {
                    self.sound_events.push(
                        if self.phase() == Phase::Stretch && self.runner.position >= 0 {
                            SoundCue::Landing
                        } else {
                            SoundCue::Failure
                        },
                    );
                    self.runner.height = 0;
                    self.runner.vertical_velocity = 0;
                    if self.runner.position < 0 {
                        self.transition(Phase::FoulRun);
                        self.result = None;
                    } else {
                        self.result = Some(self.runner.position * 100 / 106496);
                        self.discipline.long_jump_mut().landing_good =
                            self.phase() == Phase::Stretch;
                        self.transition(if self.discipline.long_jump().landing_good {
                            Phase::LandGood
                        } else {
                            Phase::LandBad
                        });
                    }
                    self.runner.speed =
                        self.runner
                            .speed
                            .max(if self.discipline.long_jump_mut().landing_good {
                                20000
                            } else {
                                16000
                            });
                    self.phase_elapsed_ms = 0;
                }
            }
            Phase::LandGood | Phase::LandBad => {
                self.runner.speed = (self.runner.speed - 1400).max(0);
                self.runner.move_by(0);
                let bump = 1080..1200;
                if bump.contains(&self.phase_elapsed_ms) {
                    self.runner.position += 4000;
                }
                if self.phase_elapsed_ms > 2000 {
                    self.finish(self.result);
                }
            }
            _ => {}
        }
    }
}

impl Game {
    pub(in crate::simulation) fn jump_action(&mut self) {
        if self.phase() == Phase::Approach {
            self.discipline.long_jump_mut().takeoff_position = self.runner.position;
            if self.runner.position > 0 {
                self.runner.speed = 16000 + (self.runner.speed - 16000) / 2;
                self.transition(Phase::FoulRun);
                self.phase_elapsed_ms = 0;
                self.sound_events.push(SoundCue::Failure);
                self.message("FOUL");
                return;
            }
            self.transition(Phase::Air);
            self.sound_events.push(SoundCue::Takeoff);
            self.phase_elapsed_ms = 0;
            self.runner.vertical_velocity = (4000 + self.runner.speed * 300 / 1000).min(13000);
            self.runner.speed = 16000 + (self.runner.speed - 16000) / 2;
        } else if self.phase() == Phase::Air {
            if self.runner.vertical_velocity - 650 <= 0 {
                self.runner.vertical_velocity +=
                    (8000 - self.runner.vertical_velocity.abs() / 4).max(0);
                self.transition(Phase::Stretch);
                self.phase_elapsed_ms = 0;
                self.sound_events.push(SoundCue::Action);
                self.message("STRETCH!");
            } else {
                self.transition(Phase::BadStretch);
                self.phase_elapsed_ms = 0;
                self.sound_events.push(SoundCue::Failure);
                self.message("TOO EARLY!")
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LongJump {
    pub(in crate::simulation) phase: LongJumpPhase,
    pub takeoff_position: i32,
    pub landing_good: bool,
}
