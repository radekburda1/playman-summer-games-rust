//! Original d.h/i(c,int,bool), d.e(c,bool) and spring constraints.
use crate::{Game, Phase};
#[derive(Clone, Debug)]
pub struct Vault {
    pub held: bool,
    pub clearance: i32,
    pub fall_height: i32,
    pub camera_offset: i32,
    pub timing: crate::vault_effect::TimingEffect,
}
impl Default for Vault {
    fn default() -> Self {
        Self {
            held: false,
            clearance: 655360000,
            fall_height: 0,
            camera_offset: 0,
            timing: crate::vault_effect::TimingEffect::default(),
        }
    }
}
/// Original fast length approximation d.o, rather than Euclidean sqrt.
pub fn length(x: i32, y: i32) -> i32 {
    let (x, y) = (x.abs(), y.abs());
    let (small, big) = if x < y { (x, y) } else { (y, x) };
    (small * 441 + big * if big < small << 4 { 967 } else { 1007 }) >> 10
}
/// Java d.j hook -> d.i direction approximation -> carried pole endpoints.
pub fn carried_endpoints([x, y, dx, dy]: [f32; 4]) -> [(i32, i32); 2] {
    let (dx, dy) = (dx as i32, -(dy as i32));
    let angle = direction(dx, dy);
    let start = (x as i32, (y as i32) - dy);
    let sine = crate::presentation::SINE;
    [
        start,
        (
            start.0 + ((sine[(angle & 511) as usize] * 184) >> 14),
            start.1 - ((sine[((128 + angle) & 511) as usize] * 184) >> 14),
        ),
    ]
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
fn constrain(p: &mut [i32; 12], mode: u8) {
    let sign = if mode == 1 { -1 } else { 1 };
    let dx = sign * (p[0] + p[2] - (p[4] + p[6]));
    let dy = sign * (p[1] + p[3] - (p[5] + p[7]));
    let n = length(dx, dy) >> 6;
    p[11] = n;
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
        p[6] += x;
        p[7] += y;
    }
    let at = if mode == 1 { 6 } else { 2 };
    p[at] -= x;
    p[at + 1] -= y;
}
impl Game {
    pub fn vault_phase(&self) -> i32 {
        match self.phase {
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
    pub(crate) fn vault_down(&mut self) {
        if self.vault.held {
            return;
        }
        match self.phase {
            Phase::VaultPlant => {
                self.pole[10] = (self.pole[10] + ((self.pole[4] >> 4) + 1500).min(0)).max(0);
                self.runner.speed = self.runner.speed.max(4000);
                self.pole[2] = self.runner.speed;
                self.runner.vertical += 4000;
                self.pole[3] = self.runner.vertical;
            }
            Phase::VaultBend | Phase::VaultStraighten => {
                self.pole[2] = (self.pole[2] + 3000).max(12500);
                self.pole[3] += 5000;
            }
            _ => return,
        }
        self.vault.held = true;
        self.sound_events.push(18);
    }
    pub(crate) fn vault_up(&mut self) {
        match self.phase {
            Phase::VaultPlant => {
                self.runner.vertical = 4000;
                self.runner.speed >>= 1;
                self.pole[2] >>= 1;
                self.pole[6] >>= 1;
            }
            Phase::VaultBend | Phase::VaultStraighten => {
                self.runner.speed = self
                    .runner
                    .speed
                    .max(((128000 - self.runner.pos) * 10) >> 8);
                self.runner.vertical =
                    (self.runner.vertical + (self.pole[10] - 2700) * 5 / 4).max(0);
                self.pole[2] = -5000;
                self.sound_events.push(20);
            }
            _ => return,
        }
        self.phase = Phase::VaultRelease;
        self.phase_time = 0;
    }
    pub(crate) fn vault_step(&mut self) {
        self.vault.timing.emit = false;
        let phase = self.vault_phase();
        if phase >= 3 && (118000..=138000).contains(&self.runner.pos) {
            self.vault.clearance = self.vault.clearance.min(self.runner.height);
        }
        if phase == 2 {
            if self.vault.camera_offset < 110592 {
                self.vault.camera_offset += ((110592 - self.vault.camera_offset) >> 3).max(1200);
            }
        } else if self.vault.camera_offset > 0 {
            self.vault.camera_offset -= 6000;
        }
        match phase {
            0 => {
                if self.phase_time >= 800 {
                    self.phase = Phase::Run;
                    self.phase_time = 0;
                    self.runner.speed = 0;
                }
            }
            1 => {
                self.runner.run(false);
                let p = &mut self.pole;
                p[0] += p[2];
                p[1] += p[3];
                p[6] = self.runner.speed;
                p[7] = self.runner.vertical;
                constrain(p, 1);
                p[4] += p[6];
                p[5] += p[7];
                p[2] = self.runner.speed;
                p[3] = self.runner.vertical;
                if self.runner.pos > -1310720 {
                    self.phase = Phase::VaultPlant;
                    p[10] = self.runner.speed - 16000;
                }
            }
            2 => {
                let p = &mut self.pole;
                if p[4] >= 0 {
                    p[2] = self.runner.speed;
                    p[6] = 0;
                    p[4] = 0;
                    p[5] = 0;
                    self.runner.height = p[1];
                    self.phase_time = 0;
                    self.phase = Phase::VaultBend;
                } else {
                    self.runner.pos += self.runner.speed;
                    self.runner.height += self.runner.vertical;
                    p[0] += p[2];
                    p[1] += p[3];
                    self.runner.vertical -= 700;
                    if self.vault.held {
                        self.runner.height = self.runner.height.max(0);
                    } else if self.runner.height + self.runner.vertical < 0 {
                        self.runner.vertical -= self.runner.height + self.runner.vertical;
                    }
                    let n = if self.vault.held {
                        50
                    } else {
                        50 - 50 * p[0] / -1310720
                    };
                    p[1] = self.runner.height + ((50 + n * 6 / 4) << 10);
                    p[6] = self.runner.speed;
                    p[7] = if self.vault.held {
                        p[7] - 2100
                    } else {
                        p[8] * (p[4] >> 8) / p[9] - p[5]
                    };
                    constrain(p, 1);
                    if p[5] + p[7] < 0 {
                        p[7] -= p[5] + p[7];
                    }
                    p[4] += p[6];
                    p[5] += p[7];
                    self.vault.timing.progress = (16384 - (p[5] >> 4)).max(0);
                    self.vault.timing.emit = true;
                    p[2] = self.runner.speed;
                    p[3] = self.runner.vertical;
                }
            }
            3 | 4 => {
                let p = &mut self.pole;
                p[0] += p[2];
                p[1] += p[3];
                p[3] -= 700;
                if p[1] + p[3] < 51200 {
                    if phase == 4 {
                        self.phase = Phase::VaultRelease;
                        self.phase_time = 0;
                        return;
                    }
                    p[3] = 51200 - p[1];
                }
                if self.vault.held && p[1] < 51200 && p[3] < 0 {
                    p[3] += 51200 - p[1];
                    self.phase = Phase::VaultFall;
                    self.runner.vertical = p[3];
                    self.vault.fall_height = 51200;
                    self.phase_time = 0;
                    return;
                }
                self.vault.timing.progress = (((p[4] - p[0]) >> 4) - 2000).min(16384);
                self.vault.timing.emit = true;
                if self.vault.held {
                    p[2] = if self.phase_time < 1000 && p[1] > 120000 {
                        p[2] + ((-p[2] * 50) >> 8) + ((p[4] - p[0]) >> 6)
                    } else {
                        p[2] - 300
                    };
                } else if p[2] > 0 {
                    p[2] -= (p[2] * 50) >> 8;
                }
                let previous = p[11];
                constrain(p, 0);
                if self.phase_time > 3000 || (phase == 3 && previous - p[11] < 0) {
                    self.phase = Phase::VaultStraighten;
                }
                self.runner.pos = p[0];
                self.runner.speed = p[2];
                if self.phase == Phase::VaultStraighten {
                    self.runner.height += self.runner.vertical;
                    let delta = p[1] - self.runner.height;
                    let delta = if delta < 0 {
                        (delta * 38) >> 8
                    } else {
                        (delta * 230) >> 8
                    };
                    self.runner.vertical = ((self.runner.vertical * 150) >> 8) + delta;
                } else {
                    self.runner.height = p[1];
                    self.runner.vertical = p[3];
                }
            }
            5..=7 => {
                let p = &mut self.pole;
                p[0] += p[2];
                p[1] += p[3];
                p[4] += p[6];
                p[5] += p[7];
                if phase == 7 {
                    if p[1] < 1000 {
                        p[2] = (p[2] * 100) >> 8;
                        p[3] = (p[3] * 100) >> 8;
                    } else {
                        p[2] = (p[2] * 230) >> 8;
                    }
                    if p[5] < 1000 {
                        p[6] = (p[6] * 100) >> 8;
                        p[7] = (p[7] * 100) >> 8;
                    } else {
                        p[6] = (p[6] * 230) >> 8;
                    }
                }
                p[3] -= 1400;
                p[7] -= 1400;
                constrain(p, 2);
                let (transfer, bounce) = if phase == 7 { (150, 90) } else { (200, 120) };
                if p[1] + p[3] < 0 {
                    p[7] += (-p[3] * transfer) >> 8;
                    p[3] = (-p[3] * bounce) >> 8;
                    p[1] = 0;
                    p[2] = (p[2] * 80) >> 8;
                }
                if p[5] + p[7] < 0 {
                    p[3] += (-p[7] * transfer) >> 8;
                    p[7] = (-p[7] * bounce) >> 8;
                    p[5] = 0;
                    p[6] = (p[6] * 80) >> 8;
                }
                if phase == 7 {
                    return;
                }
                self.runner.pos += self.runner.speed;
                self.runner.height += self.runner.vertical;
                if self.runner.pos > 128000 {
                    self.runner.speed =
                        (self.runner.speed * if self.runner.pos > 256000 { 240 } else { 250 }) >> 8;
                }
                self.runner.vertical -= 700;
                if self.runner.pos >= 47360 {
                    if self.runner.height + self.runner.vertical < 62720 {
                        if self.runner.pos < 128000 {
                            self.runner.speed = -6000;
                            self.runner.vertical = 12000;
                            self.runner.height = 62720;
                        } else {
                            self.runner.height = 62720;
                            self.vault_land(26);
                        }
                    }
                } else if self.runner.height + self.runner.vertical < 0 {
                    self.runner.vertical = (-self.runner.vertical >> 1) - self.runner.height;
                    self.runner.speed >>= 4;
                }
                if self.phase == Phase::VaultRelease && self.runner.vertical < -2000 {
                    self.phase = Phase::VaultFall;
                    self.phase_time = 0;
                    self.vault.fall_height = self.runner.height;
                }
                if self.runner.height < 65000
                    && self.runner.speed.abs() < 10
                    && self.runner.vertical.abs() < 1000
                {
                    self.vault_land(22);
                }
            }
            _ => {}
        }
    }
    fn vault_land(&mut self, sound: usize) {
        if self.vault.clearance > 1024000 {
            self.vault.clearance = 0;
        }
        self.finish(if self.vault.clearance > 0 {
            Some(self.vault.clearance / 855)
        } else {
            None
        });
        if sound == 26 || self.vault.clearance <= 0 {
            self.sound_events.push(sound);
        }
    }
}
