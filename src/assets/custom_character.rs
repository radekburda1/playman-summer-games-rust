//! Virtual sprites appended to the immutable original atlas at load time.
pub const BASE: usize = 4096;
pub const COUNT: usize = 43;
pub const KUBAK: usize = BASE + 31;
pub const RADEK: usize = BASE + 37;
pub const CELL: usize = 96;
pub const ATLAS_WIDTH: usize = 2048;
pub const ORIGINAL_HEIGHT: usize = 1249;
pub fn dimensions(index: usize) -> (usize, usize) {
    match index {
        0 => (38, 36),
        1 => (34, 38),
        2 => (30, 38),
        3 => (28, 28),
        4 => (25, 25),
        5 => (50, 50),
        27..=30 => (96, 96),
        37 => (34, 34),
        38 => (31, 35),
        39 => (29, 35),
        40 => (28, 30),
        41 => (25, 25),
        42 => (50, 50),
        31 => (38, 34),
        32 => (33, 35),
        33 => (28, 35),
        34 => (26, 28),
        35 => (25, 25),
        36 => (50, 50),
        _ => {
            let sprite = original_sprite(index);
            let [_, _, w, h] =
                super::artwork::original_sprite_rect(sprite).expect("custom body source");
            (
                // Hip/upper-leg pieces contain shorts. Widening these creates a
                // protruding black wedge during the celebration jump.
                (w * if index == 16 || (23..27).contains(&index) {
                    1.0
                } else {
                    1.45
                })
                .ceil() as usize,
                h as usize,
            )
        }
    }
}
pub fn original_sprite(index: usize) -> usize {
    if index >= 27 {
        612 + index - 27
    } else {
        let table = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/tables.bin"));
        let offset = 346 + (index - 6) * 4;
        i32::from_be_bytes(table[offset..offset + 4].try_into().unwrap()) as usize
    }
}
pub fn rect(id: usize, transform: u8) -> Option<[f32; 4]> {
    let index = id.checked_sub(BASE).filter(|i| *i < COUNT)?;
    let slot = index * 8 + transform as usize;
    let (mut w, mut h) = dimensions(index);
    if matches!(transform, 1 | 3 | 6 | 7) {
        std::mem::swap(&mut w, &mut h);
    }
    Some([
        ((slot % (ATLAS_WIDTH / CELL)) * CELL) as f32,
        (ORIGINAL_HEIGHT + slot / (ATLAS_WIDTH / CELL) * CELL) as f32,
        w as f32,
        h as f32,
    ])
}

/// Torso registration is baked into the padded sprites, including transforms.
pub fn anchor_offset(_id: usize, _transform: u8) -> (f32, f32) {
    (0., 0.)
}

/// Shoulder socket and waistband center in the original torso's centered space.
pub fn torso_joints(slot: usize) -> ((f32, f32), (f32, f32)) {
    [
        ((-6., -10.), (1., 15.)),
        ((3., -9.), (-3., 12.)),
        ((10., -7.), (-8., 9.)),
        ((12., -1.), (-12., 4.)),
    ][slot]
}

/// Fit the shoulder and bottom waistband directly to the shoulder/leg joints.
pub fn torso_sample(slot: usize, x: f32, y: f32, w: f32, h: f32) -> (f32, f32) {
    let (shoulder, hip) = torso_joints(slot);
    let (tx, ty) = (hip.0 - shoulder.0, hip.1 - shoulder.1);
    let (dx, dy) = (x - shoulder.0, y - shoulder.1);
    let length_squared = tx * tx + ty * ty;
    let along = (dx * tx + dy * ty) / length_squared;
    let across = (dx * ty - dy * tx) / length_squared;
    let across = across / if across < 0. { 1.0 } else { 1.5 };
    let (sx, sy) = (w * 0.18, h * 0.17);
    let (vx, vy) = (w * (0.47 - 0.18), h * (0.94 - 0.17));
    (sx + along * vx + across * vy, sy + along * vy - across * vx)
}
