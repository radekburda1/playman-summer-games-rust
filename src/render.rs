//! Shared drawing primitives: GPU in the game, software in the comparison runner.
#![allow(dead_code)]
use macroquad::prelude::*;
use std::cell::RefCell;
struct Surface {
    frame: Image,
    atlas: Image,
}
thread_local! { static SOFTWARE: RefCell<Option<Surface>> = const { RefCell::new(None) }; }
pub fn begin_software() {
    SOFTWARE.with(|slot| {
        *slot.borrow_mut() = Some(Surface {
            frame: Image::gen_image_color(240, 320, WHITE),
            atlas: Image::from_file_with_format(
                include_bytes!("../assets/atlas.png"),
                Some(ImageFormat::Png),
            )
            .expect("embedded atlas"),
        })
    });
}
pub fn software_frame() -> Image {
    SOFTWARE.with(|slot| {
        slot.borrow()
            .as_ref()
            .expect("software surface")
            .frame
            .clone()
    })
}
pub fn save_software(path: &str) {
    SOFTWARE.with(|slot| {
        let slot = slot.borrow();
        let frame = &slot.as_ref().expect("software surface").frame;
        let mut flipped = frame.clone();
        for y in 0..320 {
            flipped.bytes[y * 960..(y + 1) * 960]
                .copy_from_slice(&frame.bytes[(319 - y) * 960..(320 - y) * 960]);
        }
        flipped.export_png(path);
    });
}

fn pixel(frame: &mut Image, x: i32, y: i32, rgba: [u8; 4]) {
    if !(0..240).contains(&x) || !(0..320).contains(&y) {
        return;
    }
    let at = (y as usize * 240 + x as usize) * 4;
    let alpha = rgba[3] as u32;
    for (channel, component) in rgba.iter().enumerate().take(3) {
        frame.bytes[at + channel] = if alpha == 255 {
            *component
        } else if alpha == 0 {
            frame.bytes[at + channel]
        } else {
            ((*component as u32 * alpha + frame.bytes[at + channel] as u32 * (255 - alpha)) >> 8)
                as u8
        };
    }
    frame.bytes[at + 3] = 255;
}
fn rgba(c: Color) -> [u8; 4] {
    [
        (c.r * 255.) as u8,
        (c.g * 255.) as u8,
        (c.b * 255.) as u8,
        (c.a * 255.) as u8,
    ]
}
pub fn clear_background(c: Color) {
    if !SOFTWARE.with(|slot| {
        let mut s = slot.borrow_mut();
        if let Some(s) = s.as_mut() {
            let p = rgba(c);
            for px in s.frame.bytes.as_chunks_mut::<4>().0 {
                px.copy_from_slice(&p);
            }
            true
        } else {
            false
        }
    }) {
        macroquad::prelude::clear_background(c);
    }
}
pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, c: Color) {
    if !SOFTWARE.with(|slot| {
        let mut s = slot.borrow_mut();
        if let Some(s) = s.as_mut() {
            for yy in (y as i32).max(0)..((y + h) as i32).min(320) {
                for xx in (x as i32).max(0)..((x + w) as i32).min(240) {
                    pixel(&mut s.frame, xx, yy, rgba(c));
                }
            }
            true
        } else {
            false
        }
    }) {
        macroquad::prelude::draw_rectangle(x, y, w, h, c);
    }
}
pub fn draw_line(x0: f32, y0: f32, x1: f32, y1: f32, width: f32, c: Color) {
    if !SOFTWARE.with(|slot| {
        let mut s = slot.borrow_mut();
        if let Some(s) = s.as_mut() {
            let (mut x, mut y) = (x0 as i32, y0 as i32);
            let (tx, ty) = (x1 as i32, y1 as i32);
            let dx = (tx - x).abs();
            let dy = -(ty - y).abs();
            let sx = if x < tx { 1 } else { -1 };
            let sy = if y < ty { 1 } else { -1 };
            let mut error = dx + dy;
            let radius = (width as i32 - 1).max(0) / 2;
            for _ in 0..10000 {
                for oy in -radius..=radius {
                    for ox in -radius..=radius {
                        pixel(&mut s.frame, x + ox, y + oy, rgba(c));
                    }
                }
                if x == tx && y == ty {
                    break;
                }
                let e = error * 2;
                if e >= dy {
                    error += dy;
                    x += sx;
                }
                if e <= dx {
                    error += dx;
                    y += sy;
                }
            }
            true
        } else {
            false
        }
    }) {
        macroquad::prelude::draw_line(x0, y0, x1, y1, width, c);
    }
}
pub fn draw_triangle(a: Vec2, b: Vec2, c: Vec2, color: Color) {
    if !SOFTWARE.with(|slot| {
        let mut s = slot.borrow_mut();
        if let Some(s) = s.as_mut() {
            let cross =
                |p: Vec2, q: Vec2, r: Vec2| (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
            for y in (a.y.min(b.y).min(c.y) as i32).max(0)..=(a.y.max(b.y).max(c.y) as i32).min(319)
            {
                for x in
                    (a.x.min(b.x).min(c.x) as i32).max(0)..=(a.x.max(b.x).max(c.x) as i32).min(239)
                {
                    let p = vec2(x as f32, y as f32);
                    let e = [cross(a, b, p), cross(b, c, p), cross(c, a, p)];
                    if e.iter().all(|v| *v >= 0.) || e.iter().all(|v| *v <= 0.) {
                        pixel(&mut s.frame, x, y, rgba(color));
                    }
                }
            }
            true
        } else {
            false
        }
    }) {
        macroquad::prelude::draw_triangle(a, b, c, color);
    }
}
pub fn blit(source: [f32; 4], dest: [f32; 2], scale: f32) {
    SOFTWARE.with(|slot| {
        let mut slot = slot.borrow_mut();
        let s = slot.as_mut().expect("software surface");
        let [sx, sy, w, h] = source;
        let [dx, dy] = dest;
        for y in 0..(h * scale) as i32 {
            for x in 0..(w * scale) as i32 {
                let ix = sx as usize + (x as f32 / scale) as usize;
                let iy = sy as usize + (y as f32 / scale) as usize;
                let at = (iy * s.atlas.width as usize + ix) * 4;
                let p = s.atlas.bytes[at..at + 4].try_into().unwrap();
                pixel(&mut s.frame, dx as i32 + x, dy as i32 + y, p);
            }
        }
    });
}
