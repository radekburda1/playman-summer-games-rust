use crate::simulation::audio::SoundCue;

#[derive(Clone, Copy, Debug, Default)]
pub struct Projectile {
    pub x: i64,
    pub y: i64,
    pub velocity_x: i64,
    pub velocity_y: i64,
    pub angle: i64,
    pub angular_velocity: i64,
}
impl Projectile {
    /// Scalar order used only by the original Java trace fixtures.
    pub fn reference_values(self) -> [i64; 6] {
        [
            self.x,
            self.y,
            self.velocity_x,
            self.velocity_y,
            self.angle,
            self.angular_velocity,
        ]
    }
}
// Original Javelin state machine: d.c/d(c,bool), f/g(c,int,bool).
use crate::{
    simulation::Game,
    simulation::Phase,
    simulation::math::{cos, sin},
};
#[derive(Clone, Debug, Default)]
pub struct Javelin {
    pub(in crate::simulation) phase: crate::simulation::discipline::JavelinPhase,
    pub projectile: Projectile,
    pub foul: bool,
    pub charge_angle: i32,
    pub release_position: i32,
    pub initial_vertical: i32,
    pub landing: i32,
}
impl Game {
    pub fn javelin_phase(&self) -> i32 {
        match self.phase() {
            Phase::Prepare => 1,
            Phase::Run => 2,
            Phase::Approach => 3,
            Phase::Charge => 4,
            Phase::Throw => 5,
            Phase::Flight => 6,
            Phase::FoulRun => 8,
            _ => 7,
        }
    }
    pub(in crate::simulation) fn javelin_release(&mut self) {
        if self.phase() != Phase::Charge {
            return;
        }
        let launch = self.runner.position + 15 * self.runner.speed;
        self.discipline.javelin_mut().foul = launch > 0;
        let angle = (self.phase_elapsed_ms * 60) & 0x1ffff;
        self.discipline.javelin_mut().charge_angle = angle;
        self.discipline.javelin_mut().projectile = Projectile {
            x: launch as i64,
            y: 131072,
            velocity_x: ((cos(16384) * self.runner.speed * 3) >> 14) as i64,
            velocity_y: ((sin(16384) * self.runner.speed * 3) >> 14) as i64,
            angle: (0x400000 + ((angle - 65536) << 6)) as i64,
            angular_velocity: 0,
        };
        self.discipline.javelin_mut().release_position = launch;
        self.discipline.javelin_mut().initial_vertical =
            self.discipline.javelin_mut().projectile.velocity_y as i32;
        self.transition(Phase::Throw);
        self.phase_elapsed_ms = 0;
        self.sound_events
            .push(if self.discipline.javelin_mut().foul {
                SoundCue::Failure
            } else {
                SoundCue::WrongInput
            });
    }
    pub(in crate::simulation) fn javelin_step(&mut self) {
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
                if self.runner.position >= -1966080 {
                    self.transition(Phase::Approach);
                    self.phase_elapsed_ms = 0;
                }
            }
            Phase::Approach | Phase::Charge => {
                self.runner.move_by(0);
                if self.runner.position > 131072 {
                    self.discipline.javelin_mut().foul = true;
                    self.finish(None);
                    self.transition(Phase::FoulRun);
                }
            }
            Phase::FoulRun => self.runner.move_by(0),
            Phase::Throw => {
                if self.phase_elapsed_ms < 500 {
                    self.runner.move_by(0);
                }
                if self.phase_elapsed_ms >= 800 {
                    self.transition(Phase::Flight);
                    self.phase_elapsed_ms = 0;
                    if !self.discipline.javelin_mut().foul {
                        self.sound_events.push(SoundCue::Action);
                    }
                }
            }
            Phase::Flight => {
                // Original two-ended rod aerodynamic model, d.c(c,boolean).
                let javelin = self.discipline.javelin_mut();
                let p = &mut javelin.projectile;
                p.x = (p.x + p.velocity_x) as i32 as i64;
                p.y = (p.y + p.velocity_y) as i32 as i64;
                p.angle = (p.angle + p.angular_velocity) as i32 as i64;
                p.angular_velocity = (p.angular_velocity * 247) >> 8;
                let l = (cos((p.angle >> 8) as i32) << 2) as i64;
                let l2 = (sin((p.angle >> 8) as i32) << 2) as i64;
                let l3 = -l2;
                let l4 = l;
                let x1 = p.x + l;
                let y1 = p.y + l2;
                let x2 = p.x - l;
                let y2 = p.y - l2;
                let flow = ((-p.velocity_x * l3 - p.velocity_y * l4) >> 16) >> 6;
                let mut fx1 = (flow * 192 * l3) >> 24;
                let mut fx2 = (flow * 320 * l3) >> 24;
                let mut fy1 = -428 + ((flow * 192 * l4) >> 24);
                let mut fy2 = -256 + ((flow * 320 * l4) >> 24);
                if fy1 < 0 && y1 < 0 || fy2 < 0 && y2 < 0 {
                    if (y1 < 0) != (y2 < 0) && y2 != y1 {
                        p.x = (x1 + (x2 - x1) * -y1 / (y2 - y1)) as i32 as i64
                    }
                    p.y = 0;
                    javelin.landing = p.x as i32;
                    javelin.foul |= p.x < 0;
                    let score = if javelin.foul {
                        None
                    } else {
                        Some((p.x * 100 / 88576) as i32)
                    };
                    if !javelin.foul {
                        self.sound_events.push(SoundCue::Landing);
                    }
                    self.finish(score);
                } else {
                    let torque = (((fx1 * l3 + fy1 * l4) * 192 / 512) >> 16)
                        + (-((fx2 * l3 + fy2 * l4) * 320 / 512) >> 16);
                    p.angular_velocity = (p.angular_velocity + (torque << 6)) as i32 as i64;
                    fx1 -= (torque * l3 * 192 / 512) >> 16;
                    fx2 += (torque * l3 * 320 / 512) >> 16;
                    fy1 -= (torque * l4 * 192 / 512) >> 16;
                    fy2 += (torque * l4 * 320 / 512) >> 16;
                    p.velocity_x = (p.velocity_x + fx1 + fx2) as i32 as i64;
                    p.velocity_y = (p.velocity_y + fy1 + fy2) as i32 as i64;
                }
            }
            _ => {}
        }
        // Java enters the foul result presentation while the rod can still fly.
        if self.phase() == Phase::Flight
            && self.discipline.javelin_mut().foul
            && self.phase_elapsed_ms > 2000
        {
            self.finished = true;
        }
    }
}

impl Game {
    pub(in crate::simulation) fn javelin_action(&mut self) {
        if self.phase() == Phase::Approach {
            self.transition(Phase::Charge);
            self.phase_elapsed_ms = 0;
            self.sound_events.push(SoundCue::Takeoff);
        }
    }
}
