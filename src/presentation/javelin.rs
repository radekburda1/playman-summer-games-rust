use crate::simulation::{Game, Phase};
/// Hook-space charge marker, using the same normalization as Java's rod renderer.
pub fn charge_marker(game: &Game) -> Option<[i32; 2]> {
    use crate::{
        assets::artwork::{Artwork, Command},
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
    let (a, b, f) = Animation::for_game(game).frames(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/tables.bin"
    )));
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
/// Clip field markings before projection. Java's legacy screen-coordinate
/// fallback/wrapping is unsuitable for segments crossing the camera plane.
pub fn ground_segment(
    camera: crate::presentation::Camera,
    a: [i32; 2],
    b: [i32; 2],
) -> Option<[[f32; 2]; 2]> {
    let c = i64::from(crate::simulation::math::cos(-camera.angle));
    let s = i64::from(crate::simulation::math::sin(-camera.angle));
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

pub(crate) fn update_effects(game: &Game, fx: &mut super::effects::TimingEffect, previous: Phase) {
    let marker = charge_marker(game);
    let projectile = game.discipline().javelin().projectile;
    let phase = game.phase();
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
