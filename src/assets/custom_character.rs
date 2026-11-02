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
        27 => (48, 40),
        28 => (48, 36),
        29 | 30 => (52, 34),
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
                (w * if (27..31).contains(&index) { 2.6 } else { 1.45 }).ceil() as usize,
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

/// Align generated shoulder/waist landmarks with the original animation rig.
pub fn anchor_offset(id: usize, transform: u8) -> (f32, f32) {
    let (x, y) = match id.checked_sub(BASE) {
        Some(27) => (6., 2.),
        Some(28) => (3., 1.),
        _ => return (0., 0.),
    };
    match transform {
        0 => (x, y),
        4 => (-x, y),
        5 => (x, -y),
        2 => (-x, -y),
        6 => (y, x),
        1 => (-y, x),
        3 => (y, -x),
        7 => (-y, -x),
        _ => (0., 0.),
    }
}
