use crate::resources::{scene, sprite};
// Perspective track and scene placement, using the original world coordinate system.
use crate::art::Art;
use crate::render::{clear_background, draw_line, draw_rectangle, draw_triangle};
use crate::{
    Event, Game, Phase,
    presentation::{Animation, Camera},
};
use macroquad::prelude::*;
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
    repeat_at(art, c, id, z, spacing, time, project(c, 0, 0, z).y);
}
fn repeat_at(art: &Art, c: Camera, id: usize, z: i32, spacing: i32, time: i32, y: f32) {
    let width = art.scene_width(id) + spacing;
    if width <= 0 {
        return;
    }
    let p = project(c, 0, 0, z);
    let mut x = (p.x as i32 % width) - width;
    let animated = matches!(id, 8194 | 8195 | 8203 | scene::CROWD);
    let frame = if animated {
        (time / 120).rem_euclid(4) as usize
    } else {
        0
    };
    let next = if animated { (frame + 1) % 4 } else { 0 };
    let fraction = if animated {
        time.rem_euclid(120) * 65536 / 120
    } else {
        0
    };
    while x < 240 {
        let cutoff = match (id, z) {
            (scene::SKYLINE, -128000) => Some(-19712),
            (scene::DISTANT_BUILDINGS, -19712) => Some(-14592),
            (scene::NEAR_BUILDINGS, -14592) => Some(-10752),
            (scene::DISTANT_BUILDINGS, -29440) => Some(-24320),
            (scene::NEAR_BUILDINGS, -24320) => Some(-20480),
            (scene::DISTANT_BUILDINGS, -10496) => Some(-5376),
            (scene::NEAR_BUILDINGS, -5376) => Some(-1536),
            _ => None,
        };
        if let Some(z) = cutoff {
            let bottom = project(c, 0, 0, z).y;
            art.scene_clipped(
                id,
                [frame, next],
                fraction,
                vec2((x + spacing) as f32, y + 1.),
                bottom,
            );
        } else {
            art.scene_tween(id, frame, next, fraction, (x + spacing) as f32, y + 1.);
        }
        x += width;
    }
}
// Original d.h: approach-side equipment repeats only to the left of -4608.
fn approach_equipment(art: &Art, c: Camera) {
    let period = art.scene_width(8196) + 256;
    if period <= 0 {
        return;
    }
    let anchor = project(c, -4608, 0, -512);
    let mut x = anchor.x as i32;
    if x > x % period + period {
        x = x % period + period;
    }
    while x > 0 {
        x -= period;
        art.scene_tween(8196, 0, 0, 0, (x + 256) as f32, anchor.y + 1.);
    }
}
fn bands(c: Camera, ends: &[(i32, u32)]) {
    // Java d.J integrates 16.16 band coverage within each screen scanline.
    // Preserve subpixel white separators rather than snapping boundaries.
    let segments: Vec<_> = ends
        .iter()
        .map(|&(z, rgb)| {
            (
                if z == i32::MAX {
                    320 << 16
                } else {
                    c.project_fixed(c.x, 0, z).1.clamp(0, 320 << 16)
                },
                rgb,
            )
        })
        .collect();
    bands_fixed(&segments);
}
fn bands_fixed(segments: &[(i32, u32)]) {
    for row in 0..320 {
        let lo = row << 16;
        let hi = (row + 1) << 16;
        let mut start = 0;
        let mut sum = [0i64; 3];
        for &(end, rgb) in segments {
            let coverage = (end.min(hi) - start.max(lo)).max(0) as i64;
            sum[0] += (rgb >> 16 & 255) as i64 * coverage;
            sum[1] += (rgb >> 8 & 255) as i64 * coverage;
            sum[2] += (rgb & 255) as i64 * coverage;
            start = start.max(end);
            if start >= hi {
                break;
            }
        }
        let rgb =
            ((sum[0] >> 16) as u32) << 16 | ((sum[1] >> 16) as u32) << 8 | (sum[2] >> 16) as u32;
        draw_rectangle(0., row as f32, 240., 1., color(rgb));
    }
}
// Original two-tone rod rasterizer d.a(x,y,x2,y2,width,color1,color2).
fn rod(a: Vec2, b: Vec2, width: i32, mut bright: u32, mut shade: u32) {
    let (x, y, x2, y2) = (a.x as i32, a.y as i32, b.x as i32, b.y as i32);
    let (dx, dy) = (y2 - y, x - x2);
    let (ax, ay) = (dx.abs(), dy.abs());
    let (sx, sy) = (if dx < 0 { -1 } else { 1 }, if dy < 0 { -1 } else { 1 });
    let (minor_x, minor_y, diag_x, diag_y, minor, major) = if ax > ay {
        (sx, 0, sx, sy, ay, ax)
    } else {
        (0, sy, sx, sy, ax, ay)
    };
    if major == 0 {
        return;
    }
    let mut left = 0;
    let mut down = 0;
    let mut remaining = ((width * major) >> 1) + 1;
    let mut error = major >> 1;
    let step = crate::vault::length(major, minor).max(1);
    if x > x2 {
        std::mem::swap(&mut bright, &mut shade);
    }
    let segment = |ox: i32, oy: i32, rgb| {
        draw_line(
            (x + ox) as f32,
            (y + oy) as f32,
            (x2 + ox) as f32,
            (y2 + oy) as f32,
            1.,
            color(rgb),
        )
    };
    loop {
        error -= minor;
        if error < 0 {
            error += major;
            left += diag_x;
            segment(left, down, bright);
            segment(-left, -down, shade);
            down += diag_y;
        } else {
            left += minor_x;
            down += minor_y;
        }
        remaining -= step;
        if remaining < 0 {
            segment(-left, -down, 0);
            segment(0, 0, bright);
            return;
        }
        segment(left, down, bright);
        segment(-left, -down, shade);
    }
}
fn vault_pillar(art: &Art, c: Camera, z: i32) {
    let bottom = project(c, 500, 0, z);
    let top = project(c, 500, 2500, z);
    rod(bottom, top, 7, 16771584, 0x938338);
    if z > 0 {
        art.sprite(480, bottom.x, bottom.y, true);
    }
}
fn vault_mat(art: &Art, c: Camera, g: &Game) {
    vault_pillar(art, c, -256);
    let height = (if g.runner.position >= 118000 {
        g.discipline().vault().clearance
    } else {
        g.runner.height
    }) >> 8;
    let marker = project(c, 500, height, -256);
    art.sprite(481, marker.x, marker.y, true);
    if let Some(record) = g.record_distance {
        let r = project(c, 500, (record * 855) >> 8, -256);
        draw_line(r.x, r.y, r.x + 22., r.y, 1., color(16606208));
        draw_line(r.x, r.y + 1., r.x + 22., r.y + 1., 1., color(7284224));
        art.sprite(483, r.x + 22., r.y, true);
    }
    let a = project(c, 200, 250, -256);
    let b = project(c, 1800, 250, -256);
    let d = project(c, 200, 250, 256);
    let e = project(c, 1800, 250, 256);
    let ground = project(c, 200, 0, 256);
    draw_triangle(a, b, d, color(16765490));
    draw_triangle(b, d, e, color(16765490));
    if a.x < d.x {
        draw_triangle(a, d, vec2(d.x, d.y + 10.), color(12292608));
        draw_triangle(
            a,
            vec2(a.x, a.y + 10.),
            vec2(d.x, d.y + 10.),
            color(12292608),
        );
        draw_triangle(
            vec2(a.x, a.y + 10.),
            vec2(d.x, d.y + 10.),
            ground,
            color(2041415),
        );
    }
    draw_rectangle(d.x, d.y, e.x - d.x, ground.y - d.y, color(1517941));
    for x in ((d.x as i32)..(e.x as i32)).step_by(59) {
        art.sprite(514, x as f32, d.y, false);
    }
    for x in [d.x + 27., e.x - 27., (d.x + e.x) / 2.] {
        art.sprite(515, x, (d.y + ground.y) / 2., true);
    }
    let y = if g.runner.position >= 118000 {
        g.discipline().vault().clearance
    } else {
        g.runner.height
    } >> 8;
    if g.runner.position >= 128000 && y > 250 {
        let a = project(c, 500, y, -256);
        let b = project(c, 500, y, 256);
        draw_line(a.x, a.y, b.x, b.y, 5., WHITE);
    }
}
fn vault_pole(c: Camera, g: &Game, carried: Option<[f32; 4]>) {
    use crate::{presentation::sin, vault::length};
    let p = g.discipline().vault().pole;
    let (a, b) = if let Some(bounds) = carried {
        let [a, b] = crate::vault::carried_endpoints(bounds);
        (vec2(a.0 as f32, a.1 as f32), vec2(b.0 as f32, b.1 as f32))
    } else {
        (
            project(c, p.grip_x >> 8, p.grip_y >> 8, 0),
            project(c, p.tip_x >> 8, p.tip_y >> 8, 0),
        )
    };
    let dx = ((b.x - a.x) as i32) << 10;
    let dy = ((b.y - a.y) as i32) << 10;
    let n = if g.vault_phase() < 3 {
        184
    } else {
        length(dx, dy) >> 10
    };
    if n <= 0 {
        return;
    }
    let (vx, vy) = (dx / n, dy / n);
    let bend = ((184 - n) >> 1).max(0);
    let (mut x, mut y) = ((a.x as i32) << 10, (a.y as i32) << 10);
    let mut previous = a;
    for i in 0..=16 {
        let offset = (bend * sin((256 * i / 16) << 8)) >> 14;
        let current = vec2(
            ((x + offset * vy) >> 10) as f32,
            ((y - offset * vx) >> 10) as f32,
        );
        let rgb = if i >= 15 {
            6441289
        } else if i % 2 == 0 {
            8045824
        } else {
            6067456
        };
        rod(
            previous,
            current,
            3,
            rgb,
            if i >= 15 {
                4602176
            } else if i % 2 == 0 {
                22272
            } else {
                18176
            },
        );
        previous = current;
        x += dx / 16;
        y += dy / 16;
    }
}
fn javelin_rod(g: &Game, center: Vec2, delta: Vec2, clip: Option<i32>) {
    use crate::{presentation::SINE, vault::direction};
    if center.x < -64. || center.x > 304. || center.y < -64. || center.y > 384. {
        return;
    }
    let (x, y, dx, dy) = (
        center.x as i64,
        center.y as i64,
        delta.x as i64,
        delta.y as i64,
    );
    let norm = ((((dx * dx + dy * dy) << 8) as u64).isqrt() as i64) << 4;
    if norm == 0 {
        return;
    }
    let (vx, vy) = ((dx << 16) / norm, (dy << 16) / norm);
    let landed = g.javelin_phase() == 7;
    let [tail, grip0, grip1, metal] = if landed {
        [-106, -47, -37, -2]
    } else {
        [-64, -5, 5, 40]
    };
    let at = |t: i32| {
        [
            ((2 * (x << 8) + 2 * i64::from(t) * vx + 256) >> 9) as i32,
            ((2 * (y << 8) + 2 * i64::from(t) * vy + 256) >> 9) as i32,
        ]
    };
    let a = at(tail);
    let b = at(57);
    let lerp = |a: i32, b: i32, t: i32| (a * (57 - tail - t) + b * t) / (57 - tail);
    let grip_a = [
        lerp(a[0], b[0], grip0 - tail),
        lerp(a[1], b[1], grip0 - tail),
    ];
    let grip_b = [
        lerp(a[0], b[0], grip1 - tail),
        lerp(a[1], b[1], grip1 - tail),
    ];
    let tip = [
        lerp(a[0], b[0], metal - tail),
        lerp(a[1], b[1], metal - tail),
    ];
    let angle = direction(b[0] - a[0], b[1] - a[1]);
    let cosine = SINE[((angle + 128) & 511) as usize];
    let sine = SINE[(angle & 511) as usize];
    let (mut u, mut v, mut u2, mut v2) = (
        (-2 * cosine) >> 14,
        (-2 * sine) >> 14,
        (3 * cosine) >> 14,
        (3 * sine) >> 14,
    );
    let swap = (v - v2).abs() > (u - u2).abs();
    if swap {
        std::mem::swap(&mut u, &mut v);
        std::mem::swap(&mut u2, &mut v2);
    }
    let mut reverse = false;
    if u > u2 {
        std::mem::swap(&mut u, &mut u2);
        std::mem::swap(&mut v, &mut v2);
        reverse = true;
    }
    let width = u2 - u;
    let height = (v2 - v).abs();
    let mut error = -(width + 1) / 2;
    let mut row = v;
    let mut row_step = if v < v2 { 1 } else { -1 };
    let mut step = 1;
    if !reverse {
        std::mem::swap(&mut u, &mut u2);
        step = -1;
        row_step = -row_step;
    }
    let colors = [
        15128068, 5862914, 0x2e0e00, 2780928, 3025408, 0, 0xfcefef, 11499374, 5451825,
    ];
    let line = |mut a: [i32; 2], mut b: [i32; 2], rgb| {
        if let Some(bottom) = clip {
            if a[1] >= bottom && b[1] >= bottom {
                return;
            }
            if a[1] >= bottom {
                a[0] = b[0] + (a[0] - b[0]) * (bottom - 1 - b[1]) / (a[1] - b[1]);
                a[1] = bottom - 1;
            }
            if b[1] >= bottom {
                b[0] = a[0] + (b[0] - a[0]) * (bottom - 1 - a[1]) / (b[1] - a[1]);
                b[1] = bottom - 1;
            }
        }
        draw_line(
            a[0] as f32,
            a[1] as f32,
            b[0] as f32,
            b[1] as f32,
            1.,
            color(rgb),
        );
    };
    let mut toggle = 0;
    let mut column = u;
    while column != u2 {
        let (ox, oy) = if swap { (row, column) } else { (column, row) };
        let advance = toggle ^ 1;
        if advance != 0 {
            error += height;
            if error >= 0 {
                toggle = 1;
                error -= width;
            }
        } else {
            row += row_step;
            toggle = 0;
        }
        let shade = if column == u {
            2
        } else if column + step == u2 {
            0
        } else {
            1
        };
        let shift = |p: [i32; 2]| [p[0] + ox, p[1] + oy];
        line(a, shift(grip_a), colors[shade]);
        line(shift(grip_a), shift(grip_b), colors[3 + shade]);
        line(shift(grip_b), shift(tip), colors[shade]);
        line(shift(tip), b, colors[6 + shade]);
        if advance != 0 {
            column += step;
        }
    }
}
fn javelin_point(radius: i32, angle: i32) -> (i32, i32) {
    use crate::presentation::{cos, sin};
    let offset = -cos(4096) * 512 / sin(4096);
    (
        (((((radius - offset) * cos(angle)) >> 14) + offset) * 346) >> 8,
        ((radius - offset) * sin(angle)) >> 14,
    )
}
fn javelin_ground_line(c: Camera, a: (i32, i32), b: (i32, i32), rgb: u32) {
    if let Some([a, b]) = crate::javelin::ground_segment(c, [a.0, a.1], [b.0, b.1]) {
        draw_line(a[0], a[1], b[0], b[1], 1., color(rgb));
    }
}
fn javelin_arc(c: Camera, radius: i32, rgb: u32) {
    let segments = 3 + radius / 2560;
    if segments <= 0 {
        return;
    }
    let mut previous = javelin_point(radius, -4096);
    for n in 1..=segments {
        let next = javelin_point(radius, -4096 + (8192 / segments) * n);
        javelin_ground_line(c, previous, next, rgb);
        previous = next;
    }
}
fn javelin_scene(art: &Art, g: &Game, c: Camera, time: i32) {
    let ci = 12934952;
    let cj = 14715962;
    let mut strips = vec![(-19712, 11725567), (-10752, 8104530), (-10740, 0xffffff)];
    for lane in 1..=6 {
        strips.push((-10752 + lane * 512 - 12, ci));
        strips.push((-10752 + lane * 512 + 12, 0xffffff));
    }
    for (n, rgb) in [10143323, 12374893, 10143323, 11914837, 9814862, 11783245]
        .into_iter()
        .enumerate()
    {
        strips.push((-5120 + n as i32 * 10240 / 5, rgb));
    }
    strips.push((i32::MAX, 0x3f3f3f));
    bands(c, &strips);
    // Original perspective mask separates the approach from the infield.
    let far = project(c, 173, 0, -7680);
    let center = project(c, 173, 0, 0);
    let bottom_x = if center.y != far.y {
        far.x + (center.x - far.x) * (319. - far.y) / (center.y - far.y)
    } else {
        far.x
    };
    let back = project(c, 173, 0, -512);
    let front = project(c, 173, 0, 512);
    for (a, b, rgb) in [
        (far, back, ci),
        (back, front, cj),
        (front, vec2(bottom_x, 319.), ci),
    ] {
        if a.x < 0. {
            continue;
        }
        draw_rectangle(
            0.,
            a.y,
            a.x.min(b.x).clamp(0., 240.),
            b.y - a.y + 1.,
            color(rgb),
        );
        if a.x > b.x {
            draw_triangle(vec2(b.x, a.y), a, b, color(rgb));
        } else {
            draw_triangle(vec2(a.x, b.y), b, a, color(rgb));
        }
    }
    for z in [-512, 512] {
        javelin_ground_line(c, (c.x - 1_000_000, z), (0, z), 0xffffff);
    }
    if let Some([a, b]) = crate::javelin::ground_segment(c, [0, -512], [0, 512]) {
        for n in 0..6 {
            draw_line(a[0] - n as f32, a[1], b[0] - n as f32, b[1], 1., WHITE);
        }
    }
    let (mut back, mut front) = ((0, -512), (0, 512));
    for meters in (10..=120).step_by(10) {
        let next = javelin_point(meters << 8, -4096);
        javelin_ground_line(c, back, next, 0xffffff);
        back = next;
        let next = (next.0, -next.1);
        javelin_ground_line(c, front, next, 0xffffff);
        front = next;
        if meters >= 30 {
            javelin_arc(c, meters << 8, 0xffffff);
        }
    }
    if let Some(record) = g.record_distance {
        javelin_arc(c, record * 256 / 100, 0xe15151);
    }
    if let Some(result) = g.result {
        javelin_arc(c, result * 256 / 100, 5346785);
    }
    for z in [-640, 544] {
        for (x, w, rgb) in [
            (-7696, 32, 0xffffff),
            (-7664, 2528, 16776502),
            (-5136, 32, 0xffffff),
            (-5104, 2528, 15834373),
            (-2576, 32, 0xffffff),
            (-2544, 2528, 13641520),
        ] {
            quad(c, x, x + w, z, z + 128, color(rgb));
        }
    }
    for (id, z) in [(493, -512), (492, 512)] {
        let p = project(c, 0, 0, z);
        art.bottom_center_sprite(id, p.x, p.y);
    }
    for (id, z) in [
        (scene::SKYLINE, -128000),
        (scene::DISTANT_BUILDINGS, -19712),
        (scene::NEAR_BUILDINGS, -14592),
        (scene::CROWD, -10752),
    ] {
        repeat(art, c, id, z, 0, time);
    }
    repeat_at(
        art,
        c,
        scene::STADIUM_ADVERTISING,
        -9728,
        0,
        time,
        project(c, 0, 0, -10752).y,
    );
    for meters in (50..=100).step_by(10) {
        let p = javelin_point(meters << 8, -4096);
        let p = project(c, p.0, 0, p.1);
        let id = 494 + (meters - 50) / 10;
        art.bottom_center_sprite(id as usize, p.x, p.y + art.sprite_height(id as usize) / 4.);
    }
    let official = project(c, 0, 0, -1280);
    art.scene_tween(
        scene::THROW_OFFICIAL,
        usize::from(g.discipline().javelin().foul),
        usize::from(g.discipline().javelin().foul),
        0,
        official.x,
        official.y,
    );
    let person = project(c, -1536, 0, -2048);
    art.bottom_center_sprite(690, person.x, person.y);
    for (id, z, spacing, end) in [
        (8198, -2048, 384, -7936),
        (8196, -768, 256, -8448),
        (scene::FOREGROUND_ADVERTISING, 1152, 256, -4608),
    ] {
        let anchor = project(c, end, 0, z);
        let period = art.scene_width(id) + spacing;
        let mut x = anchor.x as i32;
        if x > x % period + period {
            x = x % period + period;
        }
        while x > 0 {
            x -= period;
            art.scene_tween(id, 0, 0, 0, (x + spacing) as f32, anchor.y + 1.);
        }
    }
}
pub fn draw(art: &Art, frame: &crate::presentation::SceneFrame<'_>) {
    let g = frame.game;
    let c = frame.camera;
    let waiting_highlight = frame.waiting_highlight;
    let countdown = frame.countdown;
    let scenery_time = frame.scenery_time;
    let starter_time = frame.starter_time;
    let ci = 12934952u32;
    let cj = 14715962;
    let ck = 13666640;
    clear_background(color(11725567));
    if g.event() == Event::Javelin {
        javelin_scene(art, g, c, scenery_time);
    } else if g.event().race() {
        let mut strips = vec![(-128000, 11725567), (-17920, 10143323), (-15360, ck)];
        let greens = [10143323, 12374893, 10143323, 11914837, 9814862, 11783245];
        for (n, col) in greens.iter().enumerate() {
            strips.push(((-13056 * (5 - n as i32) - 2022 * n as i32) / 5, *col));
        }
        for n in 0..6 {
            strips.push(((-6 + n) * 333 + 12, 0xffffff));
            strips.push((
                (-6 + n + 1) * 333 - 12,
                if waiting_highlight && n == 3 { cj } else { ci },
            ));
        }
        strips.push((24, 0xffffff));
        strips.push((i32::MAX, 0x3f3f3f));
        bands(c, &strips);
        repeat(art, c, scene::DISTANT_BUILDINGS, -29440, 0, scenery_time);
        repeat(art, c, scene::NEAR_BUILDINGS, -24320, 0, scenery_time);
        repeat(art, c, scene::CROWD, -20480, 0, scenery_time);
        repeat_at(
            art,
            c,
            scene::STADIUM_ADVERTISING,
            -18432,
            0,
            scenery_time,
            project(c, 0, 0, -20480).y,
        );
        repeat(
            art,
            c,
            scene::STADIUM_EQUIPMENT,
            -14848,
            art.scene_width(scene::STADIUM_EQUIPMENT),
            scenery_time,
        );
        repeat(art, c, scene::TRACK_EQUIPMENT, -2664, 64, scenery_time);
        let finish = if g.event() == Event::Dash {
            25600
        } else {
            30720
        };
        let first = ((c.x - 1920).max(0) / 2560) * 2560;
        let finish_props = project(c, finish, 0, -2126);
        art.bottom_center_sprite(sprite::FINISH_CAMERA, finish_props.x, finish_props.y);
        art.bottom_center_sprite(sprite::FINISH_BELL, finish_props.x + 32., finish_props.y);
        for x in (first..first + 12800).step_by(2560) {
            if (g.event() == Event::Dash && x == 23040) || (g.event() == Event::Hurdles && x != 0) {
                continue;
            }
            line(
                c,
                (x, 0, -1998),
                (x, 0, 0),
                if x == 0 || x == finish { 3. } else { 1. },
                WHITE,
            )
        }
        line(c, (finish, 0, -1998), (finish, 0, 0), 3., WHITE);
        if c.x > 21120 {
            for lane in 0..6 {
                let z0 = (-6 + lane) * 333;
                for mark in 0..3 {
                    let x = finish - 1600 + mark * 416;
                    let a = project(c, x, 0, z0 + 82);
                    let b = project(c, x, 0, z0 + 333 - 82);
                    // Original d.c draws four one-pixel lines at a 2x2 offset.
                    for dx in 0..2 {
                        for dy in 0..2 {
                            draw_line(
                                a.x + dx as f32,
                                a.y + dy as f32,
                                b.x + dx as f32,
                                b.y + dy as f32,
                                1.,
                                WHITE,
                            );
                        }
                    }
                }
                let number = project(c, finish - 384, 0, z0 + 166);
                art.sprite(486 + lane as usize, number.x, number.y, true);
            }
        }
        if c.x < 1920 {
            for lane in 0..6 {
                let p = project(c, 384, 0, (-6 + lane) * 333 + 166);
                art.sprite(486 + lane as usize, p.x, p.y, true);
                let start = project(c, 0, 0, (-6 + lane) * 333 + 166);
                art.bottom_right_sprite(709, start.x - 6., start.y + 5.);
                art.bottom_right_sprite(710, start.x - 62. - (lane * 28 / 6) as f32, start.y + 5.);
            }
        }
        repeat(art, c, scene::FOREGROUND_ADVERTISING, 512, 0, scenery_time);
        if g.event() == Event::Hurdles {
            let h = art.scene_height(scene::HURDLE) as f32;
            for lane in 0..6 {
                for index in 0..7 {
                    let x = (10 + index * 15) * 256;
                    if x <= c.x - 1920 || x >= c.x + 1920 {
                        continue;
                    }
                    let back = project(c, x, 0, (-6 + lane) * 333 + 12);
                    let front = project(c, x, 0, (-5 + lane) * 333 - 12);
                    let fallen = lane == 3
                        && g.runner.position > ((10 + index * 15) << 16) - 25600
                        && g.discipline().hurdles().fallen & (1 << index) != 0;
                    let frame = usize::from(fallen);
                    art.scene_tween(scene::HURDLE, frame, frame, 0, back.x, back.y);
                    if fallen {
                        draw_line(
                            back.x + h - 1.,
                            back.y - 2.,
                            back.x + h - 8.,
                            back.y - 2.,
                            1.,
                            BLACK,
                        );
                        for offset in 2..=7 {
                            draw_line(
                                back.x + h - offset as f32,
                                back.y - 1.,
                                front.x + h - offset as f32,
                                front.y - 1.,
                                1.,
                                WHITE,
                            );
                        }
                        for offset in [1., 8.] {
                            draw_line(
                                back.x + h - offset,
                                back.y - 1.,
                                front.x + h - offset,
                                front.y - 1.,
                                1.,
                                BLACK,
                            );
                        }
                    } else {
                        draw_line(
                            back.x - 2.,
                            back.y - h + 1.,
                            back.x - 2.,
                            back.y - h + 8.,
                            1.,
                            BLACK,
                        );
                        for offset in 2..=7 {
                            draw_line(
                                back.x - 1.,
                                back.y - h + offset as f32,
                                front.x - 1.,
                                front.y - h + offset as f32,
                                1.,
                                WHITE,
                            );
                        }
                        for offset in [1., 8.] {
                            draw_line(
                                back.x - 1.,
                                back.y - h + offset,
                                front.x - 1.,
                                front.y - h + offset,
                                1.,
                                BLACK,
                            );
                        }
                    }
                    art.scene_tween(scene::HURDLE, frame, frame, 0, front.x, front.y);
                }
            }
        }
    } else {
        if g.event() == Event::PoleVault {
            let depth = |z| c.project_fixed(c.x, 0, z).1;
            let mut strips = vec![
                (0, 0),
                (depth(-8960).max(0), 11725567),
                (depth(-1536), 8104530),
            ];
            for z in (-1536..-768).step_by(200) {
                strips.push((depth(z), ci));
                strips.push((depth(z + 32), 13801852));
            }
            strips.push((depth(-768), ci));
            for z in (-743..-544).step_by(50) {
                strips.push((depth(z), 0x484848));
                strips.push((depth(z) + 16384, 0));
            }
            for (z, rgb) in [
                (-544, 0x484848),
                (-512, 0xffffff),
                (-256, ci),
                (256, cj),
                (512, ci),
                (544, 0xffffff),
            ] {
                strips.push((depth(z), rgb));
            }
            strips.push((depth(544) + 65536, 0));
            strips.push((320 << 16, 0x30a030));
            bands_fixed(&strips);
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
        }
        if matches!(g.event(), Event::LongJump | Event::PoleVault) {
            repeat(art, c, 8222, -128000, 0, scenery_time);
        }
        repeat(art, c, scene::DISTANT_BUILDINGS, -10496, 0, scenery_time);
        repeat(art, c, scene::NEAR_BUILDINGS, -5376, 0, scenery_time);
        let stadium_y = project(
            c,
            0,
            0,
            if g.event() == Event::PoleVault {
                -1792
            } else {
                -1536
            },
        )
        .y;
        repeat_at(art, c, 8192, -1536, 0, scenery_time, stadium_y - 20.);
        repeat_at(art, c, 8195, -1536, 0, scenery_time, stadium_y - 20.);
        repeat_at(art, c, 8197, -1280, 0, scenery_time, stadium_y);
        if g.event() == Event::LongJump {
            approach_equipment(art, c);
            let equipment = project(c, 2496, 0, -768);
            art.scene_tween(
                scene::LONG_JUMP_EQUIPMENT,
                0,
                0,
                0,
                equipment.x,
                equipment.y,
            );
        }
        if g.event() == Event::LongJump {
            // Original inset approach stripes and white/red takeoff board.
            line(c, (0, 0, -244), (0, 0, 244), 8., WHITE);
            let a = project(c, 0, 0, -244);
            let b = project(c, 0, 0, 244);
            for dx in 1..5 {
                draw_line(
                    a.x + dx as f32,
                    a.y,
                    b.x + dx as f32,
                    b.y,
                    1.,
                    color(0xe03030),
                );
            }
            line(c, (-4096, 0, 0), (-4156, 0, -60), 4., WHITE);
            line(c, (-4096, 0, 0), (-4156, 0, 60), 4., WHITE);
            for z in [-340, 256] {
                for (x, width, rgb) in [
                    (-4112, 32, 0xffffff),
                    (-4080, 1334, 16776502),
                    (-2746, 32, 0xffffff),
                    (-2714, 1333, 15834373),
                    (-1381, 32, 0xffffff),
                    (-1349, 1333, 13641520),
                ] {
                    quad(c, x, x + width, z, z + 84, color(rgb));
                }
            }
            for (id, z) in [(493, -320), (492, 320)] {
                let p = project(c, 0, 0, z);
                art.bottom_center_sprite(id, p.x, p.y);
            }
            if c.x > -1920 {
                let back_left = project(c, 416, 0, -400);
                let back_right = project(c, 5408, 0, -400);
                let front_left = project(c, 416, 0, 400);
                let front_right = project(c, 5408, 0, 400);
                let top = back_left.y as i32;
                let bottom = front_left.y as i32;
                if bottom > top {
                    let step = ((bottom - top + 1) << 8) / 20;
                    let mut row = top << 8;
                    let mut left = ((back_left.x as i32) << 8) + 128;
                    let mut right = ((back_right.x as i32) << 8) + 128;
                    let dl = (((front_left.x - back_left.x) as i32) << 8) / 20;
                    let dr = (((front_right.x - back_right.x) as i32) << 8) / 20;
                    let mut rgb = [
                        (16049328 >> 16 & 255) << 8,
                        (16049328 >> 8 & 255) << 8,
                        (16049328 & 255) << 8,
                    ];
                    let target = [13742479 >> 16 & 255, 13742479 >> 8 & 255, 13742479 & 255];
                    let delta = [
                        ((target[0] << 8) - rgb[0]) / 20,
                        ((target[1] << 8) - rgb[1]) / 20,
                        ((target[2] << 8) - rgb[2]) / 20,
                    ];
                    while row < (bottom + 1) << 8 {
                        draw_rectangle(
                            (left >> 8) as f32,
                            (row >> 8) as f32,
                            ((right >> 8) - (left >> 8) + 1) as f32,
                            ((step >> 8) + 1) as f32,
                            color(
                                ((rgb[0] << 8) & 0xff0000 | rgb[1] & 0xff00 | rgb[2] >> 8 & 255)
                                    as u32,
                            ),
                        );
                        row += step;
                        left += dl;
                        right += dr;
                        for i in 0..3 {
                            rgb[i] += delta[i];
                        }
                    }
                    let interp = |a: f32, b: f32, t: i32| {
                        (a as i32 * (bottom - top - t) + b as i32 * t) / (bottom - top)
                    };
                    let l0 = interp(back_left.x, front_left.x, -10);
                    let l1 = interp(back_left.x, front_left.x, bottom - top + 10);
                    let r0 = interp(back_right.x, front_right.x, -10);
                    let r1 = interp(back_right.x, front_right.x, bottom - top + 10);
                    let rear_left = if l0 < 120 { back_left.x as i32 } else { l0 };
                    let rear_right = if r0 < 120 { r0 } else { back_right.x as i32 };
                    let near_left = if l1 < 120 { front_left.x as i32 } else { l1 };
                    let near_right = if r1 < 120 { front_right.x as i32 } else { r1 };
                    draw_rectangle(
                        rear_left as f32,
                        (top - 10) as f32,
                        (rear_right - rear_left) as f32,
                        10.,
                        color(0x484848),
                    );
                    draw_rectangle(
                        near_left as f32,
                        bottom as f32,
                        (near_right - near_left) as f32,
                        10.,
                        color(0x484848),
                    );
                    for stripe in 1..5 {
                        let d = 10 * stripe / 5;
                        draw_line(
                            rear_left as f32,
                            (top - d) as f32,
                            rear_right as f32,
                            (top - d) as f32,
                            1.,
                            color(0x323232),
                        );
                        draw_line(
                            near_left as f32,
                            (bottom + d) as f32,
                            near_right as f32,
                            (bottom + d) as f32,
                            1.,
                            color(0x323232),
                        );
                    }
                    for offset in -1..7 {
                        draw_line(
                            (l0 - offset) as f32,
                            (top - 10) as f32,
                            (l1 - offset) as f32,
                            (bottom + 9) as f32,
                            1.,
                            color(0x484848),
                        );
                        draw_line(
                            (r0 + offset) as f32,
                            (top - 10) as f32,
                            (r1 + offset) as f32,
                            (bottom + 9) as f32,
                            1.,
                            color(0x484848),
                        );
                    }
                }
                let ruler_start = project(c, 416, 0, -528);
                let ruler_end = project(c, 4160, 0, -528);
                draw_rectangle(
                    ruler_start.x,
                    ruler_start.y - 29.,
                    ruler_end.x - ruler_start.x,
                    29.,
                    WHITE,
                );
                for meter in 1..10 {
                    let a = project(c, 416 * meter, 0, -528);
                    let b = project(c, 416 * (meter + 1), 0, -528);
                    if meter > 1 {
                        art.label(5, &meter.to_string(), a.x, ruler_start.y - 27., 1);
                    }
                    for tick in 0..10 {
                        let x = a.x + (b.x - a.x) * tick as f32 / 10.;
                        draw_rectangle(
                            x,
                            ruler_start.y - 9.,
                            ((b.x - a.x) / 10.).max(1.),
                            8.,
                            color(if tick % 2 == 0 { 11670053 } else { 16518171 }),
                        );
                    }
                }
            }
            if let Some(record) = g.record_distance {
                let x = record * 416 / 100;
                line(c, (x, 0, -400), (x, 0, 400), 5., color(16098699));
                let flag = project(c, x, 0, -400);
                art.bottom_center_sprite(483, flag.x, flag.y);
            }
            repeat(
                art,
                c,
                scene::FOREGROUND_ADVERTISING,
                896,
                art.scene_width(scene::FOREGROUND_ADVERTISING),
                scenery_time,
            );
            let official = project(c, 0, 0, -640);
            let foul = matches!(g.phase(), Phase::FoulRun) || (g.finished && g.result.is_none());
            art.scene_tween(
                scene::THROW_OFFICIAL,
                usize::from(foul),
                usize::from(foul),
                0,
                official.x,
                official.y,
            );
            if !matches!(g.phase(), Phase::Prepare | Phase::Run | Phase::Approach) {
                let takeoff = project(c, g.discipline().long_jump().takeoff_position >> 8, 0, 0);
                art.right_center_sprite(
                    if foul && scenery_time % 600 < 300 {
                        479
                    } else {
                        478
                    },
                    takeoff.x + 1.,
                    takeoff.y,
                );
            }
            if let Some(distance) = g.result {
                let mark = project(c, distance * 416 / 100, 0, 0);
                art.sprite(482, mark.x - 6., mark.y, true);
                art.sprite(462, mark.x - 6., mark.y - 3., false);
            }
        }
        if g.event() == Event::PoleVault {
            quad(c, -280, -20, -64, 64, color(0x484848));
            quad(c, -20, 20, -256, 256, WHITE);
            for z in [-340, 256] {
                for (x, w, rgb) in [
                    (-5136, 32, 0xffffff),
                    (-5104, 1675, 16776502),
                    (-3429, 32, 0xffffff),
                    (-3397, 1675, 15834373),
                    (-1722, 32, 0xffffff),
                    (-1690, 1674, 13641520),
                ] {
                    quad(c, x, x + w, z, z + 84, color(rgb));
                }
            }
            for z in [-256, 211] {
                quad(c, -30720, 0, z, z + 45, WHITE);
            }
            let bench = project(c, -5120, 0, -304);
            let width = art.scene_width(scene::BENCH);
            let height = art.scene_height(scene::BENCH);
            let mut bx = bench.x as i32;
            while bx > -width {
                art.scene_tween(
                    scene::BENCH,
                    0,
                    0,
                    0,
                    (bx - width) as f32,
                    bench.y - height as f32,
                );
                bx -= width + 256;
            }
            vault_mat(art, c, g);
        }
    }
    let lane = Camera::athlete_lane(g.event());
    let x = g.runner.position >> 8;
    let ground = project(c, x, 0, lane);
    let y = g.runner.height >> 8;
    let mut p = project(c, x, y, lane);
    if g.event() == Event::PoleVault
        && g.phase() == Phase::VaultPlant
        && g.discipline().vault().held
    {
        p.x -= 14.;
        p.y -= 84.;
    }
    if p.x > -120. && p.x < 360. && p.y > -160. && p.y < 480. {
        if g.event() == Event::PoleVault && x >= 200 {
            if g.runner.height > 76800 {
                let shadow = project(c, x, 250, 0);
                art.sprite(
                    518 + ((g.runner.position / 81920) & 3) as usize,
                    shadow.x - 9.,
                    shadow.y,
                    true,
                );
            }
        } else {
            art.sprite(
                518 + ((g.runner.position / 81920) & 3) as usize,
                ground.x - 9.,
                ground.y,
                true,
            );
        }
        if g.event() == Event::PoleVault && countdown.is_some() {
            art.animation_with_hook(
                Animation::new(3050, 9, scenery_time, 120, true),
                p.x,
                p.y,
                |bounds| {
                    vault_pole(c, g, Some(bounds));
                },
            );
        } else if let Some(t) = countdown.filter(|_| g.event().race()) {
            if g.event() == Event::Hurdles && starter_time.is_none() {
                art.animation(Animation::new(2378, 5, scenery_time, 200, true), p.x, p.y);
            } else {
                art.animation(Animation::ready(t), p.x, p.y)
            }
        } else {
            if (g.event() == Event::PoleVault && x > 200)
                || (g.event() == Event::LongJump
                    && matches!(
                        g.phase(),
                        Phase::Stretch | Phase::LandGood | Phase::LandBad | Phase::Done
                    ))
            {
                art.animation_ground_clip(
                    frame.animation,
                    p.x,
                    p.y,
                    if g.event() == Event::PoleVault {
                        project(c, x, 250, 0).y
                    } else {
                        p.y
                    },
                );
            } else if g.event() == Event::PoleVault && g.vault_phase() < 2 {
                art.animation_with_hook(frame.animation, p.x, p.y, |bounds| {
                    vault_pole(c, g, Some(bounds));
                });
            } else if g.event() == Event::Javelin && g.javelin_phase() < 6 {
                art.animation_with_hook(frame.animation, p.x, p.y, |[x, y, dx, dy]| {
                    javelin_rod(g, vec2(x, y + dy - 1.), vec2(dx, -dy), None);
                });
            } else {
                art.athlete(g, p.x, p.y)
            }
        }
    }
    if g.event() == Event::PoleVault {
        if g.vault_phase() >= 2 {
            vault_pole(c, g, None);
        }
        if x >= 200 && g.runner.height <= 76800 {
            let mark = project(c, x, 250, 0);
            let left = project(c, 200, 250, -256).x + 32.;
            let right = project(c, 1800, 250, -256).x - 32.;
            if mark.x >= left && mark.x <= right {
                art.sprite(
                    if g.finished && g.phase_elapsed_ms > 600 {
                        517
                    } else {
                        516
                    },
                    mark.x - if g.finished { 10. } else { 0. },
                    mark.y,
                    true,
                );
            }
        }
        vault_pillar(art, c, 256);
        let official = project(c, 500, 0, 256);
        art.sprite(691, official.x, official.y, true);
        art.sprite(693, official.x + 60., official.y + 10., true);
        art.sprite(693, official.x + 100., official.y, true);
    }
    if g.event().race() {
        let (a, b, fraction) = match starter_time {
            None => (0, 0, 0),
            Some(t) if t < 3000 => (1, 1, 0),
            Some(t) if t < 5000 => (2, 2, 0),
            Some(t) if t < 5032 => (3, 3, 0),
            Some(t) if t < 5432 => {
                let elapsed = t - 5032;
                let frame = 4 + (elapsed / 200) as usize;
                (frame, frame + 1, (elapsed % 200) * 65536 / 200)
            }
            Some(_) => (7, 7, 0),
        };
        let position = project(c, 512, 0, 512);
        art.scene_tween(scene::STARTER, a, b, fraction, position.x, position.y);
    }
    if g.event() == Event::Javelin && matches!(g.phase(), Phase::Flight | Phase::Done) {
        use crate::presentation::{cos, sin};
        let rod = g.discipline().javelin().projectile;
        let angle = (rod.angle >> 8) as i32;
        let center = project(c, (rod.x >> 8) as i32, (rod.y >> 8) as i32, 0);
        let shift = if g.phase() == Phase::Done { 2 } else { 4 };
        let tip = project(
            c,
            ((rod.x + ((cos(angle) as i64) << shift)) >> 8) as i32,
            ((rod.y + ((sin(angle) as i64) << shift)) >> 8) as i32,
            0,
        );
        let shadow = project(c, (rod.x >> 8) as i32, 0, 0);
        art.bottom_center_sprite(sprite::JAVELIN_SHADOW, shadow.x, shadow.y);
        let clip = if g.phase() == Phase::Done {
            Some(project(c, ((rod.x + ((cos(angle) as i64) << 2)) >> 8) as i32, 0, 0).y as i32)
        } else {
            None
        };
        javelin_rod(g, center, tip - center, clip);
    }
    if g.event() == Event::PoleVault {
        for (sprite, [x, y], _) in g.discipline().vault().timing.sprites() {
            art.sprite(sprite, x as f32, y as f32, true);
        }
        // Keep the timing cursor legible over the accumulated particle glow.
        if let Some(marker) = crate::vault::timing_marker(g) {
            art.transformed_sprite(
                sprite::TIMING_CURSOR,
                ((scenery_time >> 7) & 3) as u8,
                marker.position[0] as f32,
                marker.position[1] as f32,
            );
        }
    }
    if g.event() == Event::Javelin {
        for (sprite, [x, y], world) in g.discipline().javelin().effects.sprites() {
            let p = if world {
                project(c, x, y, 0)
            } else {
                vec2(x as f32, y as f32)
            };
            art.sprite(sprite, p.x, p.y, true);
        }
        if let Some([x, y]) = crate::javelin::charge_marker(g) {
            art.transformed_sprite(
                sprite::TIMING_CURSOR,
                ((scenery_time >> 7) & 3) as u8,
                (x >> 10) as f32,
                (y >> 10) as f32,
            );
        }
    }
}
