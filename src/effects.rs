//! Deterministic visual effects with a separate RNG and fixed-step clock.
use crate::presentation::SINE;
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
    pub preset: usize,
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
    pub fn update_particles(&mut self) {
        self.particles.retain_mut(|p| {
            if self.clock >= p.born + p.lifetime {
                return false;
            }
            p.position[0] += p.velocity[0];
            p.position[1] += p.velocity[1];
            let table = preset(p.preset);
            if table.bounce_scale > 0 && p.position[1] < p.origin[1] {
                p.position[1] = p.origin[1];
                p.velocity[0] = (p.velocity[0] * table.bounce_scale) >> 8;
                p.velocity[1] = (-p.velocity[1] * table.bounce_scale) >> 8;
            }
            p.velocity[1] += (150 * table.gravity_scale) >> 8;
            true
        });
    }
    pub fn emit_preset(
        &mut self,
        id: usize,
        anchor: [i32; 2],
        range: Option<[i32; 2]>,
        scale: i32,
    ) {
        let table = preset(id);
        let [low, high] = range.unwrap_or([table.angle_start, table.angle_end]);
        for _ in 0..((table.count * scale) >> 10) {
            let angle = if low == high {
                low
            } else {
                low + self.random() % (high - low)
            };
            let speed = 50 + self.random() % 50;
            let velocity = [
                (SINE[(angle & 511) as usize] * speed) >> 10,
                (SINE[((angle + 128) & 511) as usize] * speed) >> 10,
            ]
            .map(|v| (v * table.speed_scale) >> 8);
            let extra = if table.lifetime_jitter_ms > 0 {
                self.random() % table.lifetime_jitter_ms
            } else {
                self.random() & 15
            };
            self.particles.push(Particle {
                position: anchor,
                velocity,
                origin: anchor,
                born: self.clock,
                lifetime: table.lifetime_ms + extra,
                preset: id,
            });
        }
    }
    pub fn step(&mut self, marker: Option<Marker>) {
        self.clock += 32;
        self.update_particles();
        if !self.emit {
            return;
        }
        let Some(marker) = marker.filter(|m| m.on_segment) else {
            return;
        };
        self.anchor = marker.position.map(|v| v << 10);
        self.emit_preset(
            1,
            self.anchor,
            Some([marker.direction - 64, marker.direction + 64]),
            1024,
        );
    }
    pub fn sprites(&self) -> impl Iterator<Item = (usize, [i32; 2], bool)> + '_ {
        self.particles.iter().map(|p| {
            let age = (((self.clock - p.born) << 8) / p.lifetime).min(256);
            let table = preset(p.preset);
            let follow = if table.follow_curve > 0 {
                (table.follow_scale * age) >> 8
            } else if table.follow_curve < 0 {
                (table.follow_scale * (256 - age)) >> 8
            } else {
                table.follow_scale
            };
            let point = [0, 1].map(|i| {
                (p.position[i] + ((self.anchor[i] - p.origin[i]).wrapping_mul(follow) >> 14)) >> 10
            });
            let sprite = if table.sprite_family == 4 {
                466 + ((age * 10) >> 8).min(2)
            } else {
                463 + ((age * 2) >> 8)
            };
            (sprite as usize, point, table.world_space != 0)
        })
    }
}
struct ParticlePreset {
    sprite_family: i32,
    count: i32,
    speed_scale: i32,
    lifetime_ms: i32,
    gravity_scale: i32,
    lifetime_jitter_ms: i32,
    angle_start: i32,
    angle_end: i32,
    follow_scale: i32,
    follow_curve: i32,
    world_space: i32,
    bounce_scale: i32,
}
fn preset(id: usize) -> ParticlePreset {
    let data = include_bytes!("../assets/tables.bin");
    // Resource records are sixteen big-endian i16 values at byte offset 3568.
    let value = |index: usize| {
        let offset = 3568 + id * 32 + index * 2;
        i16::from_be_bytes([data[offset], data[offset + 1]]) as i32
    };
    ParticlePreset {
        sprite_family: value(0),
        count: value(1),
        speed_scale: value(4),
        lifetime_ms: value(6),
        gravity_scale: value(8),
        lifetime_jitter_ms: value(9),
        angle_start: value(10),
        angle_end: value(11),
        follow_scale: value(12),
        follow_curve: value(13),
        world_space: value(14),
        bounce_scale: value(15),
    }
}
