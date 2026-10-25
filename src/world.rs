//! Perspective track and scene placement, using the original world coordinate system.
use crate::art::Art;
use macroquad::prelude::*;
use playman_rust::{
    Event, Game, Phase,
    presentation::{Animation, Camera},
};
fn color(rgb: u32) -> Color {
    Color::from_rgba((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8, 255)
}
fn project(c: Camera, x: i32, y: i32, z: i32) -> Vec2 {
    let (x, y) = c.project(x, y, z);
    vec2(x as f32, y as f32)
}
fn line(c: Camera, a: (i32, i32, i32), b: (i32, i32, i32), width: f32, col: Color) {
    let a = project(c, a.0, a.1, a.2);
    let b = project(c, b.0, b.1, b.2);
    draw_line(a.x, a.y, b.x, b.y, width, col);
}
fn quad(c: Camera, x0: i32, x1: i32, z0: i32, z1: i32, col: Color) {
    let a = project(c, x0, 0, z0);
    let b = project(c, x1, 0, z0);
    let d = project(c, x0, 0, z1);
    let e = project(c, x1, 0, z1);
    draw_triangle(a, b, d, col);
    draw_triangle(b, e, d, col);
}
fn repeat(art: &Art, c: Camera, id: usize, z: i32, spacing: i32, time: i32) {
    let width = art.scene_width(id) + spacing;
    if width <= 0 {
        return;
    }
    let p = project(c, 0, 0, z);
    let mut x = (p.x as i32 % width) - width;
    while x < 240 {
        art.scene(
            id,
            if matches!(id, 8194 | 8195) {
                (time / 120).rem_euclid(4) as usize
            } else {
                0
            },
            (x + spacing) as f32,
            p.y + 1.,
        );
        x += width;
    }
}
fn bands(c: Camera, ends: &[(i32, u32)]) {
    let mut y = 0.;
    for &(z, rgb) in ends {
        let end = if z == i32::MAX {
            320.
        } else {
            project(c, c.x, 0, z).y.clamp(0., 320.)
        };
        if end > y {
            draw_rectangle(0., y, 240., end - y, color(rgb));
            y = end
        }
    }
}
pub fn draw(art: &Art, g: &Game, countdown: Option<i32>) {
    let mut c = Camera::for_game(g);
    if g.event.race()
        && let Some(t) = countdown
    {
        let ease = |a, b, x: i32| {
            let w = (playman_rust::presentation::sin(x * 65536 / 400 - 32768) + 16384) >> 7;
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
    let ci = 12934952u32;
    let cj = 14715962;
    let ck = 13666640;
    clear_background(color(11725567));
    if g.event.race() {
        let mut strips = vec![(-128000, 11725567), (-17920, 10143323), (-15360, ck)];
        let greens = [10143323, 12374893, 10143323, 11914837, 9814862, 11783245];
        for (n, col) in greens.iter().enumerate() {
            strips.push((-13056 + 11034 * n as i32 / 5, *col));
        }
        for n in 0..6 {
            strips.push(((-6 + n) * 333 + 12, 0xffffff));
            strips.push(((-6 + n + 1) * 333 - 12, ci));
        }
        strips.push((24, 0xffffff));
        strips.push((i32::MAX, 0x3f3f3f));
        bands(c, &strips);
        repeat(art, c, 8208, -29440, 0, g.elapsed);
        repeat(art, c, 8207, -24320, 0, g.elapsed);
        repeat(art, c, 8204, -20480, 120, g.elapsed);
        repeat(art, c, 8202, -14848, art.scene_width(8202), g.elapsed);
        repeat(art, c, 8201, -2664, 64, g.elapsed);
        let finish = if g.event == Event::Dash { 25600 } else { 30720 };
        let first = ((c.x - 1920).max(0) / 2560) * 2560;
        for x in (first..first + 12800).step_by(2560) {
            line(
                c,
                (x, 0, -1998),
                (x, 0, 0),
                if x == 0 || x == finish { 3. } else { 1. },
                WHITE,
            )
        }
        line(c, (finish, 0, -1998), (finish, 0, 0), 3., WHITE);
        if c.x < 1920 {
            for lane in 0..6 {
                let p = project(c, 384, 0, (-6 + lane) * 333 + 166);
                art.sprite(486 + lane as usize, p.x, p.y, true);
            }
        }
        if g.event == Event::Hurdles {
            for i in 0..7 {
                let x = (-117760 + 655360 + i * 983040) >> 8;
                let p = project(c, x, 0, -833);
                let fallen = g.tripped & (1 << i) != 0;
                draw_line(p.x - 13., p.y - 23., p.x + 13., p.y - 23., 3., BLUE);
                draw_line(
                    p.x - 12.,
                    p.y,
                    p.x - 12.,
                    p.y - if fallen { 4. } else { 23. },
                    2.,
                    WHITE,
                );
            }
        }
    } else {
        bands(
            c,
            &[
                (-16640, 11725567),
                (-14080, ci),
                (-280, ci),
                (-244, 0xffffff),
                (244, cj),
                (280, 0xffffff),
                (i32::MAX, ci),
            ],
        );
        repeat(art, c, 8208, -10496, 0, g.elapsed);
        repeat(art, c, 8207, -5376, 0, g.elapsed);
        repeat(art, c, 8192, -1536, 0, g.elapsed);
        if g.event == Event::LongJump {
            quad(c, 416, 5408, -400, 400, color(16049328));
            line(c, (0, 0, -244), (0, 0, 244), 5., WHITE);
            line(c, (-4096, 0, -244), (-4096, 0, 244), 2., WHITE);
        }
        if g.event == Event::Javelin {
            line(c, (0, 0, -512), (0, 0, 512), 4., WHITE);
            for m in (10..=120).step_by(10) {
                line(
                    c,
                    (m * 346, 0, -512),
                    (m * 346, 0, 512),
                    1.,
                    color(0xe6b79c),
                );
            }
        }
        if g.event == Event::PoleVault {
            quad(c, 200, 1800, -256, 256, BLUE);
            let b = project(c, 500, 0, -256);
            let t = project(c, 500, (500 * 855) >> 8, -256);
            let far = project(c, 500, (500 * 855) >> 8, 256);
            draw_line(b.x, b.y, t.x, t.y, 2., YELLOW);
            draw_line(t.x, t.y, far.x, far.y, 2., RED);
            if matches!(g.phase, Phase::Run | Phase::VaultPlant | Phase::VaultBend) {
                let hand = project(c, g.runner.pos >> 8, (g.runner.height >> 8) + 350, 0);
                let tip = project(c, g.pole[4] >> 8, 0, 0);
                draw_line(hand.x, hand.y, tip.x, tip.y, 2., YELLOW);
            }
        }
    }
    let lane = Camera::athlete_lane(g.event);
    let x = g.runner.pos >> 8;
    let ground = project(c, x, 0, lane);
    let y = g.runner.height >> 8;
    let p = project(c, x, y, lane);
    if p.x > -120. && p.x < 360. && p.y > -160. && p.y < 480. {
        art.sprite(
            518 + ((g.runner.pos / 81920) & 3) as usize,
            ground.x - 9.,
            ground.y,
            true,
        );
        if let Some(t) = countdown.filter(|_| g.event.race()) {
            art.animation(Animation::ready(t), p.x, p.y)
        } else {
            art.athlete(g, p.x, p.y)
        }
    }
    if g.event == Event::Javelin {
        let (center, direction) =
            if matches!(g.phase, Phase::Flight | Phase::Done) && g.projectile[0] != 0 {
                let r = g.projectile[0];
                let h = g.projectile[1];
                let angle = (g.projectile[4] >> 8) as i32;
                let center = project(c, (r >> 8) as i32, (h >> 8) as i32, 0);
                let end = project(
                    c,
                    ((r + ((playman_rust::presentation::cos(angle) as i64) << 4)) >> 8) as i32,
                    ((h + ((playman_rust::presentation::sin(angle) as i64) << 4)) >> 8) as i32,
                    0,
                );
                let delta = end - center;
                (
                    center,
                    if delta.length_squared() > 0. {
                        delta.normalize()
                    } else {
                        vec2(1., 0.)
                    },
                )
            } else {
                art.athlete_hook(g, p.x, p.y)
                    .unwrap_or((vec2(p.x - 12., p.y - 68.), vec2(1., 0.)))
            };
        let start = center - direction * 64.;
        let end = center + direction * 57.;
        draw_line(start.x, start.y, end.x, end.y, 1.5, YELLOW);
        let tip = center + direction * 40.;
        draw_line(tip.x, tip.y, end.x, end.y, 1.5, LIGHTGRAY);
        if g.phase == Phase::Charge {
            let n = (g.phase_time * 60) & 0x1ffff;
            let n = if n > 65536 { 131072 - n } else { n };
            let spark = center + direction * (-64. + 121. * n as f32 / 65536.);
            art.sprite(469, spark.x, spark.y, true);
        }
    }
}
