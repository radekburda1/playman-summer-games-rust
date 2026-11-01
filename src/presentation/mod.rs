//! Original fixed-point projection and distance-driven animation selection.
pub mod effects;
pub mod javelin;
pub mod prompts;
pub mod vault_marker;
use crate::simulation::math::{cos, sin};
use crate::{assets::resources::animation, simulation::Event, simulation::Game, simulation::Phase};
fn lerp(a: i32, b: i32, x: i32, d: i32) -> i32 {
    (a * (d - x) + b * x) / d
}
fn curve(a: i32, b: i32, x: i32, d: i32) -> i32 {
    lerp(a, b, (4 * x * x / d - x) / 3, d)
}
fn ease(a: i32, b: i32, x: i32, d: i32) -> i32 {
    lerp(a, b, (sin(x * 65536 / d - 32768) + 16384) >> 7, 256)
}
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub angle: i32,
}
impl Camera {
    pub fn new(x: i32, y: i32, z: i32, angle: i32) -> Self {
        Self { x, y, z, angle }
    }
    /// x,y,z are original projection units (simulation values shifted right by 8).
    pub fn project(self, x: i32, y: i32, z: i32) -> (i32, i32) {
        let (x, y) = self.project_fixed(x, y, z);
        (x >> 16, y >> 16)
    }
    pub fn project_fixed(self, x: i32, y: i32, z: i32) -> (i32, i32) {
        let x = x - self.x - 128;
        let y = y - self.y;
        let z = z - self.z;
        let c = cos(-self.angle) as i64;
        let s = sin(-self.angle) as i64;
        let ry = ((c * y as i64 + s * z as i64) >> 14) as i32;
        let rz = ((-s * y as i64 + c * z as i64) >> 14) as i32;
        if rz >= 0 {
            return (if x < 0 { 0 } else { 239 << 16 }, 319 << 16);
        }
        let sx = 0x780000i32.wrapping_add((24641536i64 * x as i64 / (-rz) as i64) as i32);
        let sy = ((319i32 - 16 + 22) >> 1 << 16)
            .wrapping_sub((24641536i64 * ry as i64 / (-rz) as i64) as i32);
        (sx, sy)
    }
    /// Original d.B camera, before athlete-follow offsets are applied.
    pub fn race_at(target: i32) -> Self {
        if target <= 5120 {
            Self::new(target, 1792, 2048, 8192)
        } else if target >= 20480 {
            Self::new(target, 1280, 2304, 4608)
        } else {
            let t = (target - 5120) / 60;
            Self::new(
                target,
                ease(1792, 1280, t, 256),
                ease(2048, 2304, t, 256),
                ease(8192, 4608, t, 256),
            )
        }
    }
    pub fn for_game(g: &Game) -> Self {
        let p = g.runner.position >> 8;
        match g.event() {
            Event::Dash | Event::Hurdles => {
                let shift = if g.event() == Event::Hurdles { 5120 } else { 0 };
                let mut target = p;
                if p < 2560 {
                    target += ease(256, 0, p.max(0), 2560)
                }
                if let Some(dive) = g.runner.dive {
                    let start = dive >> 8;
                    target += ease(0, 256, (start - (19456 + shift)).clamp(0, 3072), 3072);
                    let d = p - start;
                    if d < 1536 {
                        target -= d / 3
                    } else if d < 3072 {
                        target -= 512 - (d - 1536) / 3
                    }
                } else if p > 22528 + shift {
                    target += 256
                } else if p > 19456 + shift {
                    target += ease(0, 256, p - (19456 + shift), 3072)
                }
                if p >= 25600 + shift {
                    target += ease(0, -256, (p - (25600 + shift)).min(3072), 3072)
                }
                Self::race_at(target)
            }
            Event::LongJump => {
                let target = match g.phase() {
                    Phase::Run | Phase::Prepare => p,
                    Phase::Approach => {
                        if p < -2048 {
                            p + lerp(0, 256, p + 4096, 2048)
                        } else {
                            p.min(0) + 256
                        }
                    }
                    Phase::Air if g.phase_elapsed_ms < 400 => {
                        p + ease(256, -128, g.phase_elapsed_ms, 400)
                    }
                    Phase::Air | Phase::Stretch | Phase::BadStretch => p - 128,
                    Phase::FoulRun => 256,
                    Phase::LandGood | Phase::LandBad if g.phase_elapsed_ms < 400 => {
                        p + ease(-128, -256, g.phase_elapsed_ms, 400)
                    }
                    _ => p - 256,
                };
                let t = if target < -4096 {
                    (target + 15192, 11096)
                } else {
                    (1, 1)
                };
                Self::new(
                    target,
                    1408 - ease(0, 512, t.0, t.1),
                    2304,
                    6144 - ease(0, 3584, t.0, t.1),
                )
            }
            Event::Javelin => {
                let phase = g.javelin_phase();
                let flight = phase == 7 || (phase == 6 && !g.discipline().javelin().foul);
                let mut target = if flight {
                    (g.discipline().javelin().projectile.x >> 8) as i32
                } else {
                    p.min(0)
                };
                let mut h = if flight {
                    ((g.discipline().javelin().projectile.y >> 8) as i32 - 512).max(0)
                } else {
                    0
                };
                if phase == 6 && !g.discipline().javelin().foul {
                    if g.discipline().javelin().projectile.y > 0x2e0000 {
                        h -= 1024;
                    } else if g.discipline().javelin().projectile.y > 393216 {
                        h -= ease(
                            0,
                            1024,
                            (g.discipline().javelin().projectile.y >> 8) as i32 - 1536,
                            10240,
                        );
                    }
                }
                if target > -7680 {
                    target += if target < -3840 {
                        lerp(0, 128, target + 7680, 3840)
                    } else if target < 3460 {
                        128
                    } else if target < 17300 {
                        lerp(128, 0, target - 3460, 13840)
                    } else if target < 34600 {
                        lerp(0, -512, target - 17300, 17300)
                    } else {
                        -512
                    };
                }
                if phase == 5 {
                    target += lerp(0, 128, g.phase_elapsed_ms, 800);
                } else if phase > 5 {
                    target += 128;
                }
                if phase == 7 {
                    target += if g.discipline().javelin().foul || g.phase_elapsed_ms > 300 {
                        128
                    } else {
                        128 - curve(0, 256, 300 - g.phase_elapsed_ms, 300)
                    };
                }
                let mut y = h - 512;
                let mut angle = 2560;
                if target > -11520 {
                    let t = (target + 11520).min(7680);
                    angle -= ease(0, 1280, t, 7680);
                    y -= ease(0, 128, t, 7680);
                }
                Self::new(target, 1408 + y, 2816, angle)
            }
            Event::PoleVault => {
                let h = (g.runner.height >> 8).max(0);
                let target = (g.runner.position
                    + (((g.discipline().vault().camera_offset >> 8)
                        * (g.discipline().vault().camera_offset >> 8))
                        >> 1))
                    >> 8;
                if g.vault_phase() < 3 {
                    Self::new(
                        target + 144,
                        1408 + lerp(100, -1024, p + 15360, 15360),
                        2304,
                        6144 + lerp(0, -8800, p + 15360, 15360),
                    )
                } else {
                    let end = if g.finished {
                        g.phase_elapsed_ms.min(3000)
                    } else {
                        0
                    };
                    Self::new(
                        target.min(1000),
                        384 + ((h >> 1) - 300).max(0) + (end >> 3),
                        2304,
                        6144 - 8800 - ((h * h) >> 10) + end,
                    )
                }
            }
        }
    }
    pub fn athlete_lane(event: Event) -> i32 {
        if event.race() { -833 } else { 0 }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Animation {
    pub offset: usize,
    pub len: usize,
    pub time: i32,
    pub interval: i32,
    pub looping: bool,
    pub loop_start: usize,
}
impl Animation {
    pub fn new(offset: usize, len: usize, time: i32, interval: i32, looping: bool) -> Self {
        Self {
            offset,
            len,
            time,
            interval,
            looping,
            loop_start: 0,
        }
    }
    pub fn loop_from(mut self, start: usize) -> Self {
        self.loop_start = start;
        self
    }
    pub fn for_game(g: &Game) -> Self {
        let r = &g.runner;
        let p = r.position >> 8;
        let t = g.phase_elapsed_ms;
        if g.event().race() {
            if g.finished && r.speed < 4000 {
                if g.finish_good {
                    return if t < 160 {
                        Self::new(animation::CELEBRATION_START, 3, t, 80, false)
                    } else {
                        Self::new(animation::CELEBRATION_JUMP, 13, t - 160, 80, true)
                    };
                }
                if g.event().score(g.result) >= 450 {
                    return Self::new(animation::SATISFIED, 5, t, 120, true);
                }
                return if t < 1560 {
                    Self::new(animation::DISAPPOINTED, 14, t, 120, false)
                } else {
                    Self::new(animation::DISAPPOINTED_IDLE, 5, t - 1560, 120, true)
                };
            }
            if r.speed < 0 {
                return Self::new(animation::RACE_ACCELERATE, 11, 0, 160, false);
            }
            if let Some(start) = r.dive {
                let d = (r.position - start) >> 8;
                if d < 4000 {
                    return Self::new(animation::RACE_DIVE, 21, d, 200, false);
                }
                return Self::new(animation::RACE_RUN, 11, d - 4000, 160, true);
            }
            if g.event() == Event::Hurdles {
                if p < 2100 {
                    return Self::new(animation::HURDLES_ACCELERATE, 16, p, 140, false);
                }
                if p < 28990 {
                    let offset = match r.previous_mistakes {
                        1 => animation::HURDLES_RECOVER_ONE,
                        2 => animation::HURDLES_RECOVER_TWO,
                        3 => animation::HURDLES_RECOVER_THREE,
                        _ => animation::HURDLES_CLEAR,
                    };
                    return Self::new(offset, 23, (p - 2100).rem_euclid(3840), 175, false);
                }
                return Self::new(animation::RACE_RUN, 11, p - 28990, 160, true);
            }
            if p < 1600 {
                return Self::new(animation::RACE_ACCELERATE, 11, p, 160, false);
            }
            return Self::new(animation::RACE_RUN, 11, p - 1600, 160, true);
        }
        if g.event() == Event::LongJump {
            match g.phase() {
                Phase::Prepare => {
                    return Self::new(animation::LONG_JUMP_PREPARE, 11, t, 120, false);
                }
                Phase::LandGood => return Self::new(animation::LONG_JUMP_LAND, 11, t, 120, false),
                Phase::LandBad if t < 600 => return Self::new(animation::FALL, 11, t, 60, false),
                Phase::LandBad => {
                    return Self::new(animation::LONG_JUMP_RECOVER, 6, t - 600, 120, false);
                }
                Phase::FoulRun => {
                    return Self::new(
                        animation::LONG_JUMP_FOUL,
                        9,
                        (r.position - g.discipline().long_jump().takeoff_position) >> 8,
                        280,
                        false,
                    );
                }
                _ => {}
            }
        }
        if g.event() == Event::PoleVault {
            return match g.phase() {
                Phase::Prepare => Self::new(animation::VAULT_PREPARE, 6, t, 160, false),
                Phase::Run | Phase::VaultPlant if !g.discipline().vault().held => {
                    Self::new(animation::VAULT_RUN, 15, p + 15360, 160, true).loop_from(4)
                }
                Phase::VaultPlant => Self::new(animation::VAULT_SWING, 8, 0, 1000, false),
                Phase::VaultBend | Phase::VaultStraighten => {
                    let h = r.height >> 9;
                    Self::new(
                        animation::VAULT_SWING,
                        8,
                        if h < 220 { 220 - (h - 220) * 3 / 2 } else { h },
                        90,
                        false,
                    )
                }
                Phase::VaultRelease if g.discipline().vault().held => {
                    Self::new(animation::VAULT_RELEASE, 7, t, 100, false)
                }
                Phase::VaultRelease => Self::new(animation::RELEASE_FALL, 2, t, 120, false),
                Phase::VaultFall if r.position > 0 => Self::new(
                    animation::VAULT_FALL,
                    5,
                    (g.discipline().vault().fall_height - r.height) >> 8,
                    ((g.discipline().vault().fall_height >> 8) / 6).max(1),
                    false,
                ),
                Phase::VaultFall => Self::new(animation::FALL, 11, t, 120, false),
                _ if t < 600 => Self::new(animation::VAULT_LAND, 5, t, 120, false),
                _ if g.result.is_none() => {
                    if t < 3720 {
                        Self::new(animation::FOUL_REACTION, 32, t, 120, false)
                    } else {
                        Self::new(animation::FOUL_IDLE, 5, t - 3720, 120, true)
                    }
                }
                _ if g.event().score(g.result) >= 750 => {
                    if t < 160 {
                        Self::new(animation::CELEBRATION_START, 3, t, 80, false)
                    } else if t < 2080 {
                        Self::new(animation::CELEBRATION_JUMP, 13, t - 160, 80, true)
                    } else if t < 2920 {
                        Self::new(animation::CELEBRATION_END, 8, t - 2080, 120, false)
                    } else {
                        Self::new(animation::CELEBRATION_IDLE, 5, t - 2920, 120, true)
                    }
                }
                _ if g.event().score(g.result) >= 450 => {
                    Self::new(animation::SATISFIED, 5, t, 120, true)
                }
                _ => {
                    if t < 1560 {
                        Self::new(animation::DISAPPOINTED, 14, t, 120, false)
                    } else {
                        Self::new(animation::DISAPPOINTED_IDLE, 5, t - 1560, 120, true)
                    }
                }
            };
        }
        if g.event() == Event::Javelin && g.phase() == Phase::Flight {
            return Self::new(animation::JAVELIN_WATCH, 15, t, 100, false);
        }
        if g.event() == Event::Javelin && g.phase() == Phase::FoulRun {
            return Self::new(animation::JAVELIN_RUN_CYCLE, 11, p, 160, true);
        }
        if g.event() == Event::Javelin && g.finished {
            return if g.result.is_none() {
                if t < 3720 {
                    Self::new(animation::FOUL_REACTION, 32, t, 120, false)
                } else {
                    Self::new(animation::FOUL_IDLE, 5, t - 3720, 120, true)
                }
            } else if g.event().score(g.result) >= 750 {
                if t < 160 {
                    Self::new(animation::CELEBRATION_START, 3, t, 80, false)
                } else if t < 2080 {
                    Self::new(animation::CELEBRATION_JUMP, 13, t - 160, 80, true)
                } else if t < 2920 {
                    Self::new(animation::CELEBRATION_END, 8, t - 2080, 120, false)
                } else {
                    Self::new(animation::CELEBRATION_IDLE, 5, t - 2920, 120, true)
                }
            } else if g.event().score(g.result) >= 450 {
                Self::new(animation::SATISFIED, 5, t, 120, true)
            } else if t < 1560 {
                Self::new(animation::DISAPPOINTED, 14, t, 120, false)
            } else {
                Self::new(animation::DISAPPOINTED_IDLE, 5, t - 1560, 120, true)
            };
        }
        if g.finished {
            return match g.event() {
                Event::LongJump if g.finish_good => {
                    if t < 160 {
                        Self::new(animation::CELEBRATION_START, 3, t, 80, false)
                    } else {
                        Self::new(animation::CELEBRATION_JUMP, 13, t - 160, 80, true)
                    }
                }
                Event::LongJump if g.event().score(g.result) >= 450 => {
                    Self::new(animation::SATISFIED, 5, t, 120, true)
                }
                Event::LongJump => {
                    if t < 1560 {
                        Self::new(animation::DISAPPOINTED, 14, t, 120, false)
                    } else {
                        Self::new(animation::DISAPPOINTED_IDLE, 5, t - 1560, 120, true)
                    }
                }
                Event::PoleVault if t < 600 => Self::new(animation::VAULT_LAND, 5, t, 120, false),
                _ => Self::new(animation::CELEBRATION_END, 8, t, 120, true),
            };
        }
        if r.speed < 0 {
            return match g.event() {
                Event::LongJump => {
                    Self::new(animation::LONG_JUMP_PREPARE, 11, g.elapsed_ms, 120, false)
                }
                Event::Javelin => {
                    Self::new(animation::JAVELIN_PREPARE, 11, g.elapsed_ms, 120, false)
                }
                _ => Self::new(animation::VAULT_PREPARE, 6, g.elapsed_ms, 160, false),
            };
        }
        match (g.event(), g.phase()) {
            (Event::LongJump, Phase::Run | Phase::Approach) => {
                Self::new(animation::LONG_JUMP_RUN, 11, p + 40, 160, true)
            }
            (Event::LongJump, Phase::Air) => {
                Self::new(animation::LONG_JUMP_TAKEOFF, 10, t, 80, false)
            }
            (Event::LongJump, Phase::BadStretch) => {
                Self::new(animation::LONG_JUMP_TAKEOFF, 10, 720, 80, false)
            }
            (Event::LongJump, Phase::Stretch) => {
                Self::new(animation::LONG_JUMP_STRETCH, 10, t, 76, false)
            }
            (Event::Javelin, Phase::Run) => {
                let d = p + 17920;
                if d < 1120 {
                    Self::new(animation::JAVELIN_RUN, 8, d, 160, false)
                } else {
                    Self::new(animation::JAVELIN_RUN_CYCLE, 11, p, 160, true)
                }
            }
            (Event::Javelin, Phase::Approach) => {
                Self::new(animation::JAVELIN_RUN_CYCLE, 11, p, 160, true)
            }
            (Event::Javelin, Phase::Charge) => {
                let d = ((r.speed as i64 * t as i64 / 32) >> 8) as i32;
                if d > 800 {
                    Self::new(animation::JAVELIN_CHARGE, 10, d - 800, 200, true)
                } else {
                    Self::new(animation::JAVELIN_APPROACH, 5, d, 200, false)
                }
            }
            (Event::Javelin, Phase::Throw) => Self::new(animation::JAVELIN_THROW, 9, t, 100, false),
            (Event::Javelin, Phase::Flight) => {
                Self::new(animation::JAVELIN_WATCH, 15, t, 100, false)
            }
            (Event::PoleVault, Phase::Run | Phase::VaultPlant) => {
                Self::new(animation::VAULT_RUN, 15, p + 15360, 160, true).loop_from(4)
            }
            (Event::PoleVault, Phase::VaultBend) => Self::new(
                animation::VAULT_SWING,
                8,
                {
                    let h = g.runner.height >> 9;
                    if h < 220 { 220 - (h - 220) * 3 / 2 } else { h }
                },
                90,
                false,
            ),
            (Event::PoleVault, Phase::VaultRelease) => {
                Self::new(animation::VAULT_RELEASE, 7, t, 100, false)
            }
            _ => Self::new(animation::CELEBRATION_END, 8, t, 120, true),
        }
    }
    pub fn ready(time: i32) -> Self {
        if time < 3000 {
            Self::new(animation::IDLE, 17, (time - 100).max(0), 120, false)
        } else {
            Self::new(animation::DEFAULT_POSE, 2, (time - 3100).max(0), 120, false)
        }
    }
    pub fn frames(self, tables: &[u8]) -> (usize, usize, i32) {
        let cycle = (self.len as i32 - 1) * self.interval;
        let t = if self.looping {
            let intro = self.loop_start as i32 * self.interval;
            if self.loop_start > 0 {
                let time = self.time.max(0);
                if time < intro {
                    time
                } else {
                    intro + (time - intro).rem_euclid(cycle - intro)
                }
            } else {
                self.time.rem_euclid(cycle)
            }
        } else {
            self.time.clamp(0, cycle)
        };
        let n = (t / self.interval) as usize;
        let next = (n + 1).min(self.len - 1);
        let f = if n == next {
            0
        } else {
            (t % self.interval) * 65536 / self.interval
        };
        let at = |i: usize| {
            u16::from_be_bytes([tables[self.offset + i * 2], tables[self.offset + i * 2 + 1]])
                as usize
        };
        (at(n), at(next), f)
    }
}

/// Immutable, borrowed scene description. Creating or drawing it never advances simulation.
/// Particle clocks are advanced by the fixed-step presentation update, not by drawing.
pub struct SceneFrame<'a> {
    pub game: &'a Game,
    pub effects: &'a effects::TimingEffect,
    pub camera: Camera,
    pub animation: Animation,
    pub waiting_highlight: bool,
    pub countdown: Option<i32>,
    pub scenery_time: i32,
    pub starter_time: Option<i32>,
}
impl<'a> SceneFrame<'a> {
    pub fn new(
        game: &'a Game,
        effects: &'a effects::TimingEffect,
        waiting_highlight: bool,
        countdown: Option<i32>,
        scenery_time: i32,
        starter_time: Option<i32>,
    ) -> Self {
        let mut c = Camera::for_game(game);
        let g = game;
        if g.event().race()
            && let Some(t) = countdown
        {
            let ease = |a, b, x: i32| {
                let w = (crate::simulation::math::sin(x * 65536 / 400 - 32768) + 16384) >> 7;
                (a * (256 - w) + b * w) / 256
            };
            c.x = if t < 100 {
                0
            } else if t < 500 {
                ease(0, 128, t - 100)
            } else if t < 3100 {
                128
            } else if t < 3500 {
                ease(128, 256, t - 3100)
            } else {
                256
            };
        }
        Self {
            game,
            effects,
            camera: c,
            animation: Animation::for_game(game),
            waiting_highlight,
            countdown,
            scenery_time,
            starter_time,
        }
    }
}
