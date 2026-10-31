//! Shared scene decoding for GPU rendering and reference-image regression tests.
include!("../assets/rects.rs");
include!("../assets/logical.rs");
#[derive(Clone, Debug)]
pub enum Command {
    Hook { bounds: [f32; 4] },
    Image { source: [f32; 4], dest: [f32; 2] },
    Rect { bounds: [f32; 4], rgb: u32 },
}
pub struct Artwork {
    scenes: Vec<Vec<i16>>,
}
impl Default for Artwork {
    fn default() -> Self {
        Self::new()
    }
}
impl Artwork {
    pub fn new() -> Self {
        Self {
            scenes: [
                include_bytes!("../assets/animation.bin").as_slice(),
                include_bytes!("../assets/scene1.bin").as_slice(),
                include_bytes!("../assets/scene2.bin").as_slice(),
            ]
            .iter()
            .map(|bytes| {
                bytes
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|b| i16::from_be_bytes([b[0], b[1]]))
                    .collect()
            })
            .collect(),
        }
    }
    pub fn width(&self, id: usize) -> i32 {
        let d = &self.scenes[id >> 13];
        let p = d[1 + (id & 8191)] as usize;
        d[p + 3] as i32
    }
    pub fn height(&self, id: usize) -> i32 {
        let d = &self.scenes[id >> 13];
        let p = d[1 + (id & 8191)] as usize;
        d[p + 4] as i32
    }
    pub fn sprite(
        &self,
        id: usize,
        transform: u8,
        x: f32,
        y: f32,
        center: bool,
    ) -> Option<Command> {
        let mut physical = *LOGICAL.get(id)?;
        if transform != 0 {
            physical = TRANSFORMS
                .iter()
                .find(|v| v.0 == physical && v.1 == transform)
                .map_or(physical, |v| v.2);
        }
        let rect = SPRITES[SPRITES.binary_search_by_key(&physical, |s| s.0).ok()?].1;
        Some(Command::Image {
            source: rect,
            dest: [
                if center {
                    x - (rect[2] as i32 / 2) as f32
                } else {
                    x
                },
                if center {
                    y - (rect[3] as i32 / 2) as f32
                } else {
                    y
                },
            ],
        })
    }
    pub fn frame(&self, id: usize, a: usize, b: usize, f: i32, x: f32, y: f32) -> Vec<Command> {
        let mut commands = Vec::new();
        self.decode(id, a, b, f, x, y, 0, &mut commands);
        commands
    }
    #[allow(clippy::too_many_arguments)]
    fn decode(
        &self,
        id: usize,
        mut frame: usize,
        mut next: usize,
        mut f: i32,
        x: f32,
        y: f32,
        depth: usize,
        out: &mut Vec<Command>,
    ) {
        if depth > 4 {
            return;
        }
        if f > 32767 {
            std::mem::swap(&mut frame, &mut next);
            f = 65536 - f;
        }
        let Some(data) = self.scenes.get(id >> 13) else {
            return;
        };
        let Some(&start) = data.get(1 + (id & 8191)) else {
            return;
        };
        let mut p = start as usize;
        let frames = data[p] as usize;
        if frame >= frames || next >= frames {
            return;
        }
        let ox = data[p + 1] as i32;
        let oy = data[p + 2] as i32;
        p += 5;
        while p < data.len() {
            let cmd = data[p] as u16;
            p += 1;
            if cmd == 0 {
                break;
            }
            let mask = if frames > 1 {
                let m = data[p] as u16;
                p += 1;
                m
            } else {
                0
            };
            let count = (cmd & 255) as usize;
            if count > 6 {
                return;
            }
            let mut v = [0i32; 6];
            let mut nearest = [0i32; 6];
            let mut other = [0i32; 6];
            for i in 0..count {
                let (a, b) = if mask & (1 << i) != 0 {
                    let a = data[p + frame] as i32;
                    let b = data[p + next] as i32;
                    p += frames;
                    (a, b)
                } else {
                    let a = data[p] as i32;
                    p += 1;
                    (a, a)
                };
                nearest[i] = a;
                other[i] = b;
                v[i] = a + (((b - a) * f + 32768) >> 16);
            }
            if cmd & 0x4000 != 0 {
                continue;
            }
            let px = x + (v[0] - ox) as f32;
            let py = y + (v[1] - oy) as f32;
            match (cmd & 0x3fff) >> 8 {
                0 => {
                    if nearest[2] >= 0
                        && let Some(c) = self.sprite(
                            (nearest[2] >> 3) as usize,
                            (nearest[2] & 7) as u8,
                            px,
                            py,
                            true,
                        )
                    {
                        out.push(c)
                    }
                }
                1 => {
                    let a = nearest[2] as u16 as u32;
                    let b = other[2] as u16 as u32;
                    let f = f as u32;
                    let rgb = (((((a & 0xf800) * (65536 - f) + (b & 0xf800) * f) >> 13) * 33)
                        & 0xff0000)
                        | (((((a & 0x7e0) * (65536 - f) + (b & 0x7e0) * f) >> 17) * 65) & 0xff00)
                        | ((((a & 31) * (65536 - f) + (b & 31) * f) * 33) >> 18);
                    out.push(Command::Rect {
                        bounds: [px, py, v[3] as f32, v[4] as f32],
                        rgb,
                    });
                }
                2 => {
                    if nearest[2] >= 0
                        && let Some(Command::Image {
                            source: [sx, sy, w, h],
                            ..
                        }) = self.sprite(
                            (nearest[2] >> 3) as usize,
                            (nearest[2] & 7) as u8,
                            0.,
                            0.,
                            false,
                        )
                    {
                        let left = px.max(0.);
                        let top = py.max(0.);
                        let right = (px + v[3] as f32).min(240.);
                        let bottom = (py + v[4] as f32).min(320.);
                        let mut yy = py + ((top - py) / h).floor() * h;
                        while yy < bottom {
                            let mut xx = px + ((left - px) / w).floor() * w;
                            while xx < right {
                                let x0 = xx.max(left);
                                let y0 = yy.max(top);
                                let ww = (xx + w).min(right) - x0;
                                let hh = (yy + h).min(bottom) - y0;
                                if ww > 0. && hh > 0. {
                                    out.push(Command::Image {
                                        source: [sx + x0 - xx, sy + y0 - yy, ww, hh],
                                        dest: [x0, y0],
                                    });
                                }
                                xx += w;
                            }
                            yy += h;
                        }
                    }
                }
                3 => out.push(Command::Hook {
                    bounds: [px, py, v[count - 2] as f32, v[count - 1] as f32],
                }),
                4 => {
                    let nested = v[2] as usize;
                    if let Some(d) = self.scenes.get(nested >> 13)
                        && let Some(&start) = d.get(1 + (nested & 8191))
                    {
                        let p = start as usize;
                        self.decode(
                            nested,
                            0,
                            0,
                            0,
                            px + d[p + 1] as f32,
                            py + d[p + 2] as f32,
                            depth + 1,
                            out,
                        )
                    }
                }
                _ => {}
            }
        }
    }
}
