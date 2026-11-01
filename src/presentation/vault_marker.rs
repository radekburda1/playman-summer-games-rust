//! Pure pole timing geometry, evaluated during fixed-step effects updates.
use crate::{
    presentation::Camera,
    presentation::effects::Marker,
    simulation::Game,
    simulation::events::vault::{direction, length},
    simulation::math::SINE,
};
/// Original curved-rod marker placement, including integer interpolation and bQ.
pub fn timing_marker(game: &Game) -> Option<Marker> {
    let phase = game.vault_phase();
    if !(2..5).contains(&phase) {
        return None;
    }
    let c = Camera::for_game(game);
    let p = game.discipline().vault().pole;
    let a = c.project(p.grip_x >> 8, p.grip_y >> 8, 0);
    let b = c.project(p.tip_x >> 8, p.tip_y >> 8, 0);
    let dx = (b.0 - a.0) << 10;
    let dy = (b.1 - a.1) << 10;
    let n = if phase < 3 { 184 } else { length(dx, dy) >> 10 };
    if n <= 0 {
        return Some(Marker {
            position: [a.0, a.1],
            direction: 0,
            on_segment: false,
        });
    }
    let (vx, vy) = (dx / n, dy / n);
    let bend = ((184 - n) >> 1).max(0);
    let mut x = a.0 << 10;
    let mut y = a.1 << 10;
    let mut previous = [a.0, a.1];
    let progress = game.discipline().vault().timing_progress;
    for i in 0..=16 {
        let offset = (bend * SINE[(256 * i / 16) as usize]) >> 14;
        let current = [(x + offset * vy) >> 10, (y - offset * vx) >> 10];
        if i == progress >> 10 {
            let next_offset = (bend * SINE[(256 * (i + 1) / 16) as usize]) >> 14;
            let mut target = [
                (x + dx / 16 + next_offset * vy) >> 10,
                (y + dy / 16 - next_offset * vx) >> 10,
            ];
            let mut fraction = progress & 1023;
            let mut angle = 512
                - direction(
                    (current[0] - previous[0]) << 10,
                    (current[1] - previous[1]) << 10,
                );
            if phase >= 3 {
                target = previous;
                fraction = 1023 - fraction;
                angle += 256;
            }
            return Some(Marker {
                position: [0, 1]
                    .map(|j| current[j] + (((target[j] - current[j]) * fraction) >> 10)),
                direction: angle + 512,
                on_segment: true,
            });
        }
        previous = current;
        x += dx / 16;
        y += dy / 16;
    }
    Some(Marker {
        position: [a.0, a.1],
        direction: 0,
        on_segment: false,
    })
}

/// Java d.j hook -> d.i direction approximation -> carried pole endpoints.
pub fn carried_endpoints([x, y, dx, dy]: [f32; 4]) -> [(i32, i32); 2] {
    let (dx, dy) = (dx as i32, -(dy as i32));
    let angle = direction(dx, dy);
    let start = (x as i32, (y as i32) - dy);
    let sine = crate::simulation::math::SINE;
    [
        start,
        (
            start.0 + ((sine[(angle & 511) as usize] * 184) >> 14),
            start.1 - ((sine[((128 + angle) & 511) as usize] * 184) >> 14),
        ),
    ]
}
