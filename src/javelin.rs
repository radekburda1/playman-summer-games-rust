use crate::audio::SoundCue;

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
    Game, Phase,
    presentation::{cos, sin},
};
#[derive(Clone, Debug, Default)]
pub struct Javelin {
    pub(crate) phase: crate::discipline::JavelinPhase,
    pub projectile: Projectile,
    pub foul: bool,
    pub charge_angle: i32,
    pub release_position: i32,
    pub initial_vertical: i32,
    pub landing: i32,
    pub effects: crate::effects::TimingEffect,
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
    pub(crate) fn javelin_release(&mut self) {
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
    pub(crate) fn javelin_step(&mut self) {
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

/// Hook-space charge marker, using the same normalization as Java's rod renderer.
pub fn charge_marker(game: &Game) -> Option<[i32; 2]> {
    use crate::{
        artwork::{Artwork, Command},
        presentation::{Animation, Camera},
    };
    use std::sync::OnceLock;
    if !matches!(game.phase(), Phase::Charge | Phase::Throw) {
        return None;
    }
    static ART: OnceLock<Artwork> = OnceLock::new();
    let art = ART.get_or_init(Artwork::new);
    let c = Camera::for_game(game);
    let p = c.project(game.runner.position >> 8, game.runner.height >> 8, 0);
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
                let angle = if game.phase() == Phase::Charge {
                    (game.phase_elapsed_ms * 60) & 0x1ffff
                } else {
                    game.discipline().javelin().charge_angle
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
        let projectile = self.discipline.javelin().projectile;
        let phase = self.phase();
        let fx = &mut self.discipline.javelin_mut().effects;
        fx.clock += 32;
        if previous == Phase::Throw && phase == Phase::Flight {
            let n = (projectile.angle >> 11) as i32;
            let scale = (n * n) >> 10;
            fx.emit_preset(2, fx.anchor, None, scale);
        }
        if previous == Phase::Flight && phase == Phase::Done && projectile.x > 0 {
            fx.emit_preset(4, [(projectile.x as i32) << 2, 0], None, 1024);
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

impl Game {
    pub(crate) fn javelin_action(&mut self) {
        if self.phase() == Phase::Approach {
            self.transition(Phase::Charge);
            self.phase_elapsed_ms = 0;
            self.sound_events.push(SoundCue::Takeoff);
        }
    }
}
