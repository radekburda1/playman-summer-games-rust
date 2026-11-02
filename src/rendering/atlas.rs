//! Assemble custom assets without rewriting or resampling the original atlas.
use crate::assets::{artwork::original_sprite_rect, custom_character as custom};
use macroquad::prelude::{Image, ImageFormat};
fn decode(bytes: &[u8]) -> Image {
    Image::from_file_with_format(bytes, Some(ImageFormat::Png)).expect("embedded character image")
}
fn rgba(image: &Image, x: usize, y: usize) -> [u8; 4] {
    let p = (y * image.width as usize + x) * 4;
    image.bytes[p..p + 4].try_into().unwrap()
}
fn put(image: &mut Image, x: usize, y: usize, pixel: [u8; 4]) {
    let p = (y * image.width as usize + x) * 4;
    image.bytes[p..p + 4].copy_from_slice(&pixel);
}
fn bounds(image: &Image, cell: usize) -> (usize, usize, usize, usize) {
    let cw = image.width as usize / 4;
    bounds_between(image, cell * cw, (cell + 1) * cw)
}
fn bounds_between(image: &Image, start: usize, end: usize) -> (usize, usize, usize, usize) {
    let (mut left, mut top, mut right, mut bottom) = (end, image.height as usize, start, 0);
    for y in 0..image.height as usize {
        for x in start..end {
            if rgba(image, x, y)[3] > 32 {
                left = left.min(x);
                top = top.min(y);
                right = right.max(x + 1);
                bottom = bottom.max(y + 1);
            }
        }
    }
    (left, top, right - left, bottom - top)
}
/// Generated views are separated by transparent gutters, not exact quarter cells.
/// Find the nearest empty column so neighboring silhouettes cannot leak in.
fn torso_bounds(image: &Image, cell: usize) -> (usize, usize, usize, usize) {
    let width = image.width as usize;
    let cw = width / 4;
    let gutter = |boundary: usize| {
        let nominal = boundary * cw;
        (nominal - cw / 4..nominal + cw / 4)
            .min_by_key(|&x| {
                let count = (0..image.height as usize)
                    .filter(|&y| rgba(image, x, y)[3] > 32)
                    .count();
                (count, x.abs_diff(nominal))
            })
            .unwrap()
    };
    bounds_between(
        image,
        if cell == 0 { 0 } else { gutter(cell) },
        if cell == 3 { width } else { gutter(cell + 1) },
    )
}
pub fn image() -> Image {
    let original = decode(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/atlas.png"
    )));
    let heads = decode(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/custom/brother-heads.png"
    )));
    let torsos = decode(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/custom/brother-torsos.png"
    )));
    let kubak_heads = decode(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/custom/kubak-heads.png"
    )));
    let radek_heads = decode(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/custom/radek-heads.png"
    )));
    let rows = (custom::COUNT * 8).div_ceil(custom::ATLAS_WIDTH / custom::CELL);
    let height = custom::ORIGINAL_HEIGHT + rows * custom::CELL;
    let mut out = Image {
        bytes: vec![0; custom::ATLAS_WIDTH * height * 4],
        width: custom::ATLAS_WIDTH as u16,
        height: height as u16,
    };
    assert_eq!(original.width as usize, custom::ATLAS_WIDTH);
    assert_eq!(original.height as usize, custom::ORIGINAL_HEIGHT);
    out.bytes[..original.bytes.len()].copy_from_slice(&original.bytes);
    for index in 0..custom::COUNT {
        let (w, h) = custom::dimensions(index);
        let source = if index < 4 {
            bounds(&heads, index)
        } else if index < 6 {
            bounds(&heads, 2)
        } else if (27..31).contains(&index) {
            torso_bounds(&torsos, index - 27)
        } else if (31..35).contains(&index) {
            bounds(&kubak_heads, index - 31)
        } else if (35..37).contains(&index) {
            bounds(&kubak_heads, 2)
        } else if (37..41).contains(&index) {
            torso_bounds(&radek_heads, index - 37)
        } else if index >= 41 {
            torso_bounds(&radek_heads, 2)
        } else {
            let [x, y, w, h] = original_sprite_rect(custom::original_sprite(index)).unwrap();
            (x as usize, y as usize, w as usize, h as usize)
        };
        let input = if index < 6 {
            &heads
        } else if (27..31).contains(&index) {
            &torsos
        } else if index >= 37 {
            &radek_heads
        } else if index >= 31 {
            &kubak_heads
        } else {
            &original
        };
        for y in 0..h {
            for x in 0..w {
                let sx = source.0 + x * source.2 / w;
                let sy = source.1 + y * source.3 / h;
                let pixel = rgba(input, sx, sy);
                for transform in 0..8u8 {
                    let (tx, ty) = match transform {
                        0 => (x, y),
                        4 => (w - 1 - x, y),
                        5 => (x, h - 1 - y),
                        2 => (w - 1 - x, h - 1 - y),
                        6 => (y, x),
                        1 => (h - 1 - y, x),
                        3 => (y, w - 1 - x),
                        7 => (h - 1 - y, w - 1 - x),
                        _ => unreachable!(),
                    };
                    let [dx, dy, _, _] = custom::rect(custom::BASE + index, transform).unwrap();
                    put(&mut out, dx as usize + tx, dy as usize + ty, pixel);
                }
            }
        }
    }
    out
}
