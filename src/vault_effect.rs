//! Pole timing glow: original particle preset 1 at b0 byte offset 3600.
use crate::{
    Game,
    presentation::{Camera, SINE},
    vault::{direction, length},
};
#[derive(Clone, Copy, Debug)]
pub struct Marker {
    pub position: [i32; 2],
    pub direction: i32,
    pub on_segment: bool,
}
#[derive(Clone, Debug)]
pub struct Particle {
    pub position: [i32; 2],
    pub velocity: [i32; 2],
    pub origin: [i32; 2],
    pub born: i32,
    pub lifetime: i32,
}
#[derive(Clone, Debug)]
pub struct TimingEffect {
    pub progress: i32,
    pub emit: bool,
    pub particles: Vec<Particle>,
    pub anchor: [i32; 2],
    pub clock: i32,
    rng: u64,
}
impl Default for TimingEffect {
    fn default() -> Self {
        Self {
            progress: -1,
            emit: false,
            particles: Vec::new(),
            anchor: [0; 2],
            clock: -32,
            rng: (54321 ^ 25214903917) & ((1 << 48) - 1),
        }
    }
}
impl TimingEffect {
    fn random(&mut self) -> i32 {
        self.rng = (self.rng.wrapping_mul(25214903917) + 11) & ((1 << 48) - 1);
        ((self.rng >> 16) as i32).wrapping_abs()
    }
    pub fn step(&mut self, marker: Option<Marker>) {
        self.clock += 32;
        self.particles.retain_mut(|p| {
            if self.clock >= p.born + p.lifetime {
                return false;
            }
            p.position[0] += p.velocity[0];
            p.position[1] += p.velocity[1];
            true
        });
        if !self.emit {
            return;
        }
        let Some(marker) = marker.filter(|m| m.on_segment) else {
            return;
        };
        self.anchor = marker.position.map(|v| v << 10);
        for _ in 0..6 {
            let angle = marker.direction - 64 + self.random() % 128;
            let speed = 50 + self.random() % 50;
            let velocity = [
                (SINE[(angle & 511) as usize] * speed) >> 10,
                (SINE[((angle + 128) & 511) as usize] * speed) >> 10,
            ]
            .map(|v| (v * 320) >> 8);
            let lifetime = 350 + self.random() % 100;
            self.particles.push(Particle {
                position: self.anchor,
                velocity,
                origin: self.anchor,
                born: self.clock,
                lifetime,
            });
        }
    }
    pub fn sprites(&self) -> impl Iterator<Item = (usize, [i32; 2])> + '_ {
        self.particles.iter().map(|p| {
            let age = ((self.clock - p.born) << 8) / p.lifetime;
            let point = [0, 1].map(|i| (p.position[i] + self.anchor[i] - p.origin[i]) >> 10);
            (463 + ((age.min(256) * 2) >> 8) as usize, point)
        })
    }
}
/// Original curved-rod marker placement, including integer interpolation and bQ.
pub fn marker(game: &Game) -> Option<Marker> {
    let phase = game.vault_phase();
    if !(2..5).contains(&phase) {
        return None;
    }
    let c = Camera::for_game(game);
    let p = game.pole;
    let a = c.project(p[0] >> 8, p[1] >> 8, 0);
    let b = c.project(p[4] >> 8, p[5] >> 8, 0);
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
    let progress = game.vault.timing.progress;
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
