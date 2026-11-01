use crate::simulation::audio::SoundCue;
use crate::simulation::discipline::VaultPhase as Stage;

#[derive(Clone, Copy, Debug, Default)]
pub struct Pole {
    pub grip_x: i32,
    pub grip_y: i32,
    pub grip_velocity_x: i32,
    pub grip_velocity_y: i32,
    pub tip_x: i32,
    pub tip_y: i32,
    pub tip_velocity_x: i32,
    pub tip_velocity_y: i32,
    pub initial_tip_height: i32,
    pub planting_reference: i32,
    pub stored_energy: i32,
    pub length: i32,
}
impl Pole {
    /// Scalar order used only by the original Java trace fixtures.
    pub fn reference_values(self) -> [i32; 12] {
        [
            self.grip_x,
            self.grip_y,
            self.grip_velocity_x,
            self.grip_velocity_y,
            self.tip_x,
            self.tip_y,
            self.tip_velocity_x,
            self.tip_velocity_y,
            self.initial_tip_height,
            self.planting_reference,
            self.stored_energy,
            self.length,
        ]
    }
}
// Original d.h/i(c,int,bool), d.e(c,bool) and spring constraints.
use crate::{simulation::Game, simulation::Phase};
#[derive(Clone, Debug)]
pub struct Vault {
    pub(in crate::simulation) phase: crate::simulation::discipline::VaultPhase,
    pub pole: Pole,
    pub held: bool,
    pub clearance: i32,
    pub fall_height: i32,
    pub camera_offset: i32,
    pub timing_progress: i32,
    pub emit_timing: bool,
}
impl Default for Vault {
    fn default() -> Self {
        Self {
            phase: crate::simulation::discipline::VaultPhase::Prepare,
            pole: Pole::default(),
            held: false,
            clearance: 655360000,
            fall_height: 0,
            camera_offset: 0,
            timing_progress: -1,
            emit_timing: false,
        }
    }
}
/// Original fast length approximation d.o, rather than Euclidean sqrt.
pub fn length(x: i32, y: i32) -> i32 {
    let (x, y) = (x.abs(), y.abs());
    let (small, big) = if x < y { (x, y) } else { (y, x) };
    (small * 441 + big * if big < small << 4 { 967 } else { 1007 }) >> 10
}
pub fn direction(dx: i32, dy: i32) -> i32 {
    if dx == 0 {
        if dy > 0 { 256 } else { 0 }
    } else if dx < 0 {
        511 - if dy < 0 {
            (dx << 7) / (dx + dy)
        } else {
            ((dx - dy - dy) << 7) / (dx - dy)
        }
    } else if dy < 0 {
        (dx << 7) / (dx - dy)
    } else {
        ((dx + dy + dy) << 7) / (dx + dy)
    }
}
fn constrain(p: &mut Pole, mode: u8) {
    let sign = if mode == 1 { -1 } else { 1 };
    let dx = sign * (p.grip_x + p.grip_velocity_x - (p.tip_x + p.tip_velocity_x));
    let dy = sign * (p.grip_y + p.grip_velocity_y - (p.tip_y + p.tip_velocity_y));
    let n = length(dx, dy) >> 6;
    p.length = n;
    if n == 0 {
        return;
    }
    let delta = i64::from(n - 5160);
    let mut x = (delta * i64::from(dx) / i64::from(n)) as i32;
    let mut y = (delta * i64::from(dy) / i64::from(n)) as i32;
    if mode == 0 && delta < 0 {
        x >>= 6;
        y >>= 6;
    }
    if mode == 2 {
        x >>= 3;
        y >>= 3;
        p.tip_velocity_x += x;
        p.tip_velocity_y += y;
    }
    if mode == 1 {
        p.tip_velocity_x -= x;
        p.tip_velocity_y -= y;
    } else {
        p.grip_velocity_x -= x;
        p.grip_velocity_y -= y;
    }
}
impl Game {
    pub fn vault_phase(&self) -> i32 {
        match self.phase() {
            Phase::Prepare => 0,
            Phase::Run => 1,
            Phase::VaultPlant => 2,
            Phase::VaultBend => 3,
            Phase::VaultStraighten => 4,
            Phase::VaultRelease => 5,
            Phase::VaultFall => 6,
            _ => 7,
        }
    }
    pub(in crate::simulation) fn vault_down(&mut self) {
        if self.discipline.vault_mut().held {
            return;
        }
        match self.phase() {
            Phase::VaultPlant => {
                self.discipline.vault_mut().pole.stored_energy =
                    (self.discipline.vault_mut().pole.stored_energy
                        + ((self.discipline.vault_mut().pole.tip_x >> 4) + 1500).min(0))
                    .max(0);
                self.runner.speed = self.runner.speed.max(4000);
                self.discipline.vault_mut().pole.grip_velocity_x = self.runner.speed;
                self.runner.vertical_velocity += 4000;
                self.discipline.vault_mut().pole.grip_velocity_y = self.runner.vertical_velocity;
            }
            Phase::VaultBend | Phase::VaultStraighten => {
                self.discipline.vault_mut().pole.grip_velocity_x =
                    (self.discipline.vault_mut().pole.grip_velocity_x + 3000).max(12500);
                self.discipline.vault_mut().pole.grip_velocity_y += 5000;
            }
            _ => return,
        }
        self.discipline.vault_mut().held = true;
        self.sound_events.push(SoundCue::Takeoff);
    }
    pub(in crate::simulation) fn vault_up(&mut self) {
        match self.phase() {
            Phase::VaultPlant => {
                self.runner.vertical_velocity = 4000;
                self.runner.speed >>= 1;
                self.discipline.vault_mut().pole.grip_velocity_x >>= 1;
                self.discipline.vault_mut().pole.tip_velocity_x >>= 1;
            }
            Phase::VaultBend | Phase::VaultStraighten => {
                self.runner.speed = self
                    .runner
                    .speed
                    .max(((128000 - self.runner.position) * 10) >> 8);
                self.runner.vertical_velocity = (self.runner.vertical_velocity
                    + (self.discipline.vault_mut().pole.stored_energy - 2700) * 5 / 4)
                    .max(0);
                self.discipline.vault_mut().pole.grip_velocity_x = -5000;
                self.sound_events.push(SoundCue::Action);
            }
            _ => return,
        }
        self.transition(Phase::VaultRelease);
        self.phase_elapsed_ms = 0;
    }
    pub(in crate::simulation) fn vault_step(&mut self) {
        self.discipline.vault_mut().emit_timing = false;
        let phase = self.discipline.vault().phase;
        if matches!(
            phase,
            Stage::VaultBend
                | Stage::VaultStraighten
                | Stage::VaultRelease
                | Stage::VaultFall
                | Stage::Done
        ) && (118000..=138000).contains(&self.runner.position)
        {
            self.discipline.vault_mut().clearance = self
                .discipline
                .vault_mut()
                .clearance
                .min(self.runner.height);
        }
        if matches!(phase, Stage::VaultPlant) {
            if self.discipline.vault_mut().camera_offset < 110592 {
                self.discipline.vault_mut().camera_offset +=
                    ((110592 - self.discipline.vault_mut().camera_offset) >> 3).max(1200);
            }
        } else if self.discipline.vault_mut().camera_offset > 0 {
            self.discipline.vault_mut().camera_offset -= 6000;
        }
        match phase {
            Stage::Prepare => {
                if self.phase_elapsed_ms >= 800 {
                    self.transition(Phase::Run);
                    self.phase_elapsed_ms = 0;
                    self.runner.speed = 0;
                }
            }
            Stage::Run => {
                self.runner.run(false);
                let vault = self.discipline.vault_mut();
                let p = &mut vault.pole;
                p.grip_x += p.grip_velocity_x;
                p.grip_y += p.grip_velocity_y;
                p.tip_velocity_x = self.runner.speed;
                p.tip_velocity_y = self.runner.vertical_velocity;
                constrain(p, 1);
                p.tip_x += p.tip_velocity_x;
                p.tip_y += p.tip_velocity_y;
                p.grip_velocity_x = self.runner.speed;
                p.grip_velocity_y = self.runner.vertical_velocity;
                if self.runner.position > -1310720 {
                    vault.phase = crate::simulation::discipline::VaultPhase::VaultPlant;
                    p.stored_energy = self.runner.speed - 16000;
                }
            }
            Stage::VaultPlant => {
                let vault = self.discipline.vault_mut();
                let p = &mut vault.pole;
                if p.tip_x >= 0 {
                    p.grip_velocity_x = self.runner.speed;
                    p.tip_velocity_x = 0;
                    p.tip_x = 0;
                    p.tip_y = 0;
                    self.runner.height = p.grip_y;
                    self.phase_elapsed_ms = 0;
                    self.transition(Phase::VaultBend);
                } else {
                    self.runner.position += self.runner.speed;
                    self.runner.height += self.runner.vertical_velocity;
                    p.grip_x += p.grip_velocity_x;
                    p.grip_y += p.grip_velocity_y;
                    self.runner.vertical_velocity -= 700;
                    if vault.held {
                        self.runner.height = self.runner.height.max(0);
                    } else if self.runner.height + self.runner.vertical_velocity < 0 {
                        self.runner.vertical_velocity -=
                            self.runner.height + self.runner.vertical_velocity;
                    }
                    let n = if vault.held {
                        50
                    } else {
                        50 - 50 * p.grip_x / -1310720
                    };
                    p.grip_y = self.runner.height + ((50 + n * 6 / 4) << 10);
                    p.tip_velocity_x = self.runner.speed;
                    p.tip_velocity_y = if vault.held {
                        p.tip_velocity_y - 2100
                    } else {
                        p.initial_tip_height * (p.tip_x >> 8) / p.planting_reference - p.tip_y
                    };
                    constrain(p, 1);
                    if p.tip_y + p.tip_velocity_y < 0 {
                        p.tip_velocity_y -= p.tip_y + p.tip_velocity_y;
                    }
                    p.tip_x += p.tip_velocity_x;
                    p.tip_y += p.tip_velocity_y;
                    vault.timing_progress = (16384 - (p.tip_y >> 4)).max(0);
                    vault.emit_timing = true;
                    p.grip_velocity_x = self.runner.speed;
                    p.grip_velocity_y = self.runner.vertical_velocity;
                }
            }
            Stage::VaultBend | Stage::VaultStraighten => {
                let vault = self.discipline.vault_mut();
                let p = &mut vault.pole;
                p.grip_x += p.grip_velocity_x;
                p.grip_y += p.grip_velocity_y;
                p.grip_velocity_y -= 700;
                if p.grip_y + p.grip_velocity_y < 51200 {
                    if matches!(phase, Stage::VaultStraighten) {
                        self.transition(Phase::VaultRelease);
                        self.phase_elapsed_ms = 0;
                        return;
                    }
                    p.grip_velocity_y = 51200 - p.grip_y;
                }
                if vault.held && p.grip_y < 51200 && p.grip_velocity_y < 0 {
                    p.grip_velocity_y += 51200 - p.grip_y;
                    vault.phase = crate::simulation::discipline::VaultPhase::VaultFall;
                    self.runner.vertical_velocity = p.grip_velocity_y;
                    vault.fall_height = 51200;
                    self.phase_elapsed_ms = 0;
                    return;
                }
                vault.timing_progress = (((p.tip_x - p.grip_x) >> 4) - 2000).min(16384);
                vault.emit_timing = true;
                if vault.held {
                    p.grip_velocity_x = if self.phase_elapsed_ms < 1000 && p.grip_y > 120000 {
                        p.grip_velocity_x
                            + ((-p.grip_velocity_x * 50) >> 8)
                            + ((p.tip_x - p.grip_x) >> 6)
                    } else {
                        p.grip_velocity_x - 300
                    };
                } else if p.grip_velocity_x > 0 {
                    p.grip_velocity_x -= (p.grip_velocity_x * 50) >> 8;
                }
                let previous = p.length;
                constrain(p, 0);
                if self.phase_elapsed_ms > 3000
                    || (matches!(phase, Stage::VaultBend) && previous - p.length < 0)
                {
                    vault.phase = crate::simulation::discipline::VaultPhase::VaultStraighten;
                }
                self.runner.position = p.grip_x;
                self.runner.speed = p.grip_velocity_x;
                if matches!(
                    vault.phase,
                    crate::simulation::discipline::VaultPhase::VaultStraighten
                ) {
                    self.runner.height += self.runner.vertical_velocity;
                    let delta = p.grip_y - self.runner.height;
                    let delta = if delta < 0 {
                        (delta * 38) >> 8
                    } else {
                        (delta * 230) >> 8
                    };
                    self.runner.vertical_velocity =
                        ((self.runner.vertical_velocity * 150) >> 8) + delta;
                } else {
                    self.runner.height = p.grip_y;
                    self.runner.vertical_velocity = p.grip_velocity_y;
                }
            }
            Stage::VaultRelease | Stage::VaultFall | Stage::Done => {
                let vault = self.discipline.vault_mut();
                let p = &mut vault.pole;
                p.grip_x += p.grip_velocity_x;
                p.grip_y += p.grip_velocity_y;
                p.tip_x += p.tip_velocity_x;
                p.tip_y += p.tip_velocity_y;
                if matches!(phase, Stage::Done) {
                    if p.grip_y < 1000 {
                        p.grip_velocity_x = (p.grip_velocity_x * 100) >> 8;
                        p.grip_velocity_y = (p.grip_velocity_y * 100) >> 8;
                    } else {
                        p.grip_velocity_x = (p.grip_velocity_x * 230) >> 8;
                    }
                    if p.tip_y < 1000 {
                        p.tip_velocity_x = (p.tip_velocity_x * 100) >> 8;
                        p.tip_velocity_y = (p.tip_velocity_y * 100) >> 8;
                    } else {
                        p.tip_velocity_x = (p.tip_velocity_x * 230) >> 8;
                    }
                }
                p.grip_velocity_y -= 1400;
                p.tip_velocity_y -= 1400;
                constrain(p, 2);
                let (transfer, bounce) = if matches!(phase, Stage::Done) {
                    (150, 90)
                } else {
                    (200, 120)
                };
                if p.grip_y + p.grip_velocity_y < 0 {
                    p.tip_velocity_y += (-p.grip_velocity_y * transfer) >> 8;
                    p.grip_velocity_y = (-p.grip_velocity_y * bounce) >> 8;
                    p.grip_y = 0;
                    p.grip_velocity_x = (p.grip_velocity_x * 80) >> 8;
                }
                if p.tip_y + p.tip_velocity_y < 0 {
                    p.grip_velocity_y += (-p.tip_velocity_y * transfer) >> 8;
                    p.tip_velocity_y = (-p.tip_velocity_y * bounce) >> 8;
                    p.tip_y = 0;
                    p.tip_velocity_x = (p.tip_velocity_x * 80) >> 8;
                }
                if matches!(phase, Stage::Done) {
                    return;
                }
                self.runner.position += self.runner.speed;
                self.runner.height += self.runner.vertical_velocity;
                if self.runner.position > 128000 {
                    self.runner.speed = (self.runner.speed
                        * if self.runner.position > 256000 {
                            240
                        } else {
                            250
                        })
                        >> 8;
                }
                self.runner.vertical_velocity -= 700;
                if self.runner.position >= 47360 {
                    if self.runner.height + self.runner.vertical_velocity < 62720 {
                        if self.runner.position < 128000 {
                            self.runner.speed = -6000;
                            self.runner.vertical_velocity = 12000;
                            self.runner.height = 62720;
                        } else {
                            self.runner.height = 62720;
                            self.vault_land(SoundCue::Landing);
                        }
                    }
                } else if self.runner.height + self.runner.vertical_velocity < 0 {
                    self.runner.vertical_velocity =
                        (-self.runner.vertical_velocity >> 1) - self.runner.height;
                    self.runner.speed >>= 4;
                }
                if self.phase() == Phase::VaultRelease && self.runner.vertical_velocity < -2000 {
                    self.transition(Phase::VaultFall);
                    self.phase_elapsed_ms = 0;
                    self.discipline.vault_mut().fall_height = self.runner.height;
                }
                if self.runner.height < 65000
                    && self.runner.speed.abs() < 10
                    && self.runner.vertical_velocity.abs() < 1000
                {
                    self.vault_land(SoundCue::Failure);
                }
            }
        }
    }
    fn vault_land(&mut self, sound: crate::simulation::audio::SoundCue) {
        if self.discipline.vault_mut().clearance > 1024000 {
            self.discipline.vault_mut().clearance = 0;
        }
        let clearance = self.discipline.vault().clearance;
        self.finish(if clearance > 0 {
            Some(clearance / 855)
        } else {
            None
        });
        if sound == SoundCue::Landing || self.discipline.vault_mut().clearance <= 0 {
            self.sound_events.push(sound);
        }
    }
}
