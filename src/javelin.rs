//! Original Javelin state machine: d.c/d(c,bool), f/g(c,int,bool).
use crate::{
    Game, Phase,
    presentation::{cos, sin},
};
#[derive(Clone, Debug, Default)]
pub struct Javelin {
    pub foul: bool,
    pub charge_angle: i32,
    pub release_position: i32,
    pub initial_vertical: i32,
    pub landing: i32,
    pub effects: crate::vault_effect::TimingEffect,
}
impl Game {
    pub fn javelin_phase(&self) -> i32 {
        match self.phase {
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
    pub(crate) fn javelin_release(&mut self) {
        if self.phase != Phase::Charge {
            return;
        }
        let launch = self.runner.pos + 15 * self.runner.speed;
        self.javelin.foul = launch > 0;
        let angle = (self.phase_time * 60) & 0x1ffff;
        self.javelin.charge_angle = angle;
        self.projectile = [
            launch as i64,
            131072,
            ((cos(16384) * self.runner.speed * 3) >> 14) as i64,
            ((sin(16384) * self.runner.speed * 3) >> 14) as i64,
            (0x400000 + ((angle - 65536) << 6)) as i64,
            0,
        ];
        self.javelin.release_position = launch;
        self.javelin.initial_vertical = self.projectile[3] as i32;
        self.phase = Phase::Throw;
        self.phase_time = 0;
        self.sound_events
            .push(if self.javelin.foul { 22 } else { 19 });
    }
    pub(crate) fn javelin_step(&mut self) {
        match self.phase {
            Phase::Prepare => {
                if self.phase_time >= 1200 {
                    self.phase = Phase::Run;
                    self.phase_time = 0;
                    self.runner.speed = 0;
                }
            }
            Phase::Run => {
                self.runner.run(false);
                if self.runner.pos >= -1966080 {
                    self.phase = Phase::Approach;
                    self.phase_time = 0;
                }
            }
            Phase::Approach | Phase::Charge => {
                self.runner.move_by(0);
                if self.runner.pos > 131072 {
                    self.javelin.foul = true;
                    self.finish(None);
                    self.phase = Phase::FoulRun;
                }
            }
            Phase::FoulRun => self.runner.move_by(0),
            Phase::Throw => {
                if self.phase_time < 500 {
                    self.runner.move_by(0);
                }
                if self.phase_time >= 800 {
                    self.phase = Phase::Flight;
                    self.phase_time = 0;
                    if !self.javelin.foul {
                        self.sound_events.push(20);
                    }
                }
            }
            Phase::Flight => {
                // Original two-ended rod aerodynamic model, d.c(c,boolean).
                let p = &mut self.projectile;
                p[0] = (p[0] + p[2]) as i32 as i64;
                p[1] = (p[1] + p[3]) as i32 as i64;
                p[4] = (p[4] + p[5]) as i32 as i64;
                p[5] = (p[5] * 247) >> 8;
                let l = (cos((p[4] >> 8) as i32) << 2) as i64;
                let l2 = (sin((p[4] >> 8) as i32) << 2) as i64;
                let l3 = -l2;
                let l4 = l;
                let x1 = p[0] + l;
                let y1 = p[1] + l2;
                let x2 = p[0] - l;
                let y2 = p[1] - l2;
                let flow = ((-p[2] * l3 - p[3] * l4) >> 16) >> 6;
                let mut fx1 = (flow * 192 * l3) >> 24;
                let mut fx2 = (flow * 320 * l3) >> 24;
                let mut fy1 = -428 + ((flow * 192 * l4) >> 24);
                let mut fy2 = -256 + ((flow * 320 * l4) >> 24);
                if fy1 < 0 && y1 < 0 || fy2 < 0 && y2 < 0 {
                    if (y1 < 0) != (y2 < 0) && y2 != y1 {
                        p[0] = (x1 + (x2 - x1) * -y1 / (y2 - y1)) as i32 as i64
                    }
                    p[1] = 0;
                    self.javelin.landing = p[0] as i32;
                    self.javelin.foul |= p[0] < 0;
                    let score = if self.javelin.foul {
                        None
                    } else {
                        Some((p[0] * 100 / 88576) as i32)
                    };
                    if !self.javelin.foul {
                        self.sound_events.push(26);
                    }
                    self.finish(score);
                } else {
                    let torque = (((fx1 * l3 + fy1 * l4) * 192 / 512) >> 16)
                        + (-((fx2 * l3 + fy2 * l4) * 320 / 512) >> 16);
                    p[5] = (p[5] + (torque << 6)) as i32 as i64;
                    fx1 -= (torque * l3 * 192 / 512) >> 16;
                    fx2 += (torque * l3 * 320 / 512) >> 16;
                    fy1 -= (torque * l4 * 192 / 512) >> 16;
                    fy2 += (torque * l4 * 320 / 512) >> 16;
                    p[2] = (p[2] + fx1 + fx2) as i32 as i64;
                    p[3] = (p[3] + fy1 + fy2) as i32 as i64;
                }
            }
            _ => {}
        }
        // Java enters the foul result presentation while the rod can still fly.
        if self.phase == Phase::Flight && self.javelin.foul && self.phase_time > 2000 {
            self.finished = true;
        }
    }
}

/// Hook-space charge marker, using the same normalization as Java's rod renderer.
pub fn charge_marker(game: &Game) -> Option<[i32; 2]> {
    use crate::{
        artwork::{Artwork, Command},
        presentation::{Animation, Camera},
    };
    use std::sync::OnceLock;
    if !matches!(game.phase, Phase::Charge | Phase::Throw) {
        return None;
    }
    static ART: OnceLock<Artwork> = OnceLock::new();
    let art = ART.get_or_init(Artwork::new);
    let c = Camera::for_game(game);
    let p = c.project(game.runner.pos >> 8, game.runner.height >> 8, 0);
    let (a, b, f) = Animation::for_game(game).frames(include_bytes!("../assets/tables.bin"));
    art.frame(0, a, b, f, p.0 as f32, p.1 as f32)
        .into_iter()
        .find_map(|command| {
            if let Command::Hook {
                bounds: [x, y, dx, dy],
            } = command
            {
                let (x, y, dx, dy) = (x as i64, (y + dy - 1.) as i64, dx as i64, -(dy as i64));
                let norm = ((((dx * dx + dy * dy) << 8) as u64).isqrt() as i64) << 4;
                if norm == 0 {
                    return None;
                }
                let (vx, vy) = ((dx << 16) / norm, (dy << 16) / norm);
                let angle = if game.phase == Phase::Charge {
                    (game.phase_time * 60) & 0x1ffff
                } else {
                    game.javelin.charge_angle
                };
                let angle = if angle > 65536 { 131072 - angle } else { angle };
                let t = -64 + ((121 * angle) >> 16);
                Some([
                    ((2 * (x << 8) + 2 * i64::from(t) * vx + 256) << 1) as i32,
                    ((2 * (y << 8) + 2 * i64::from(t) * vy + 256) << 1) as i32,
                ])
            } else {
                None
            }
        })
}
impl Game {
    pub(crate) fn javelin_effects_step(&mut self, previous: Phase) {
        let marker = charge_marker(self);
        let fx = &mut self.javelin.effects;
        fx.clock += 32;
        if previous == Phase::Throw && self.phase == Phase::Flight {
            let n = (self.projectile[4] >> 11) as i32;
            let scale = (n * n) >> 10;
            fx.emit_preset(2, fx.anchor, None, scale);
        }
        if previous == Phase::Flight && self.phase == Phase::Done && self.projectile[0] > 0 {
            fx.emit_preset(4, [(self.projectile[0] as i32) << 2, 0], None, 1024);
        }
        fx.update_particles();
        if let Some(marker) = marker {
            fx.anchor = marker;
            fx.emit_preset(0, marker, None, 1024);
        }
    }
}

/// Clip field markings before projection. Java's legacy screen-coordinate
/// fallback/wrapping is unsuitable for segments crossing the camera plane.
pub fn ground_segment(
    camera: crate::presentation::Camera,
    a: [i32; 2],
    b: [i32; 2],
) -> Option<[[f32; 2]; 2]> {
    let c = i64::from(crate::presentation::cos(-camera.angle));
    let s = i64::from(crate::presentation::sin(-camera.angle));
    let transform = |p: [i32; 2]| {
        let y = -i64::from(camera.y);
        let z = i64::from(p[1]) - i64::from(camera.z);
        [
            f64::from(p[0]) - f64::from(camera.x) - 128.,
            ((c * y + s * z) >> 14) as f64,
            ((-s * y + c * z) >> 14) as f64,
        ]
    };
    let (mut a, mut b) = (transform(a), transform(b));
    const NEAR: f64 = -1.;
    if a[2] >= NEAR && b[2] >= NEAR {
        return None;
    }
    if a[2] > NEAR {
        let t = (NEAR - b[2]) / (a[2] - b[2]);
        a = std::array::from_fn(|i| b[i] + (a[i] - b[i]) * t);
    }
    if b[2] > NEAR {
        let t = (NEAR - a[2]) / (b[2] - a[2]);
        b = std::array::from_fn(|i| a[i] + (b[i] - a[i]) * t);
    }
    let project = |p: [f64; 3]| {
        [
            (120. + 376. * p[0] / -p[2]).floor(),
            (162. - 376. * p[1] / -p[2]).floor(),
        ]
    };
    let (a, b) = (project(a), project(b));
    let d = [b[0] - a[0], b[1] - a[1]];
    let (mut enter, mut exit) = (0f64, 1f64);
    for (p, q) in [
        (-d[0], a[0]),
        (d[0], 239. - a[0]),
        (-d[1], a[1]),
        (d[1], 319. - a[1]),
    ] {
        if p == 0. {
            if q < 0. {
                return None;
            }
            continue;
        }
        let t = q / p;
        if p < 0. {
            enter = enter.max(t);
        } else {
            exit = exit.min(t);
        }
        if enter > exit {
            return None;
        }
    }
    Some([enter, exit].map(|t| std::array::from_fn(|i| (a[i] + d[i] * t) as f32)))
}
