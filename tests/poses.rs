use macroquad::prelude::{Image, ImageFormat};
use playman_rust::assets::artwork::{Artwork, Command};
#[test]
fn scene_decoder_matches_original_rendered_poses() {
    let atlas = Image::from_file_with_format(
        include_bytes!("../assets/atlas.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();
    let artwork = Artwork::new();
    let mut cases: Vec<_> = [
        0, 4, 53, 110, 118, 126, 135, 150, 189, 199, 210, 240, 278, 300, 317,
    ]
    .into_iter()
    .map(|f| (f, f, 0, format!("{f:03}")))
    .collect();
    for (a, b, f) in [
        (4, 5, 16384),
        (4, 5, 32768),
        (4, 5, 49152),
        (125, 126, 49152),
    ] {
        cases.push((a, b, f, format!("interp-{a}-{b}-{f}")));
    }
    for (frame, next, fraction, name) in cases {
        let expected = Image::from_file_with_format(
            &std::fs::read(format!(
                "{}/tests/fixtures/poses/{name}.png",
                env!("CARGO_MANIFEST_DIR")
            ))
            .unwrap(),
            Some(ImageFormat::Png),
        )
        .unwrap();
        let mut actual = vec![255u8; 240 * 320 * 4];
        for command in artwork.frame(0, frame, next, fraction, 120., 220.) {
            match command {
                Command::Hook { .. } => {}
                Command::Image {
                    source: [sx, sy, w, h],
                    dest: [x, y],
                } => {
                    for iy in 0..h as i32 {
                        for ix in 0..w as i32 {
                            let dx = x as i32 + ix;
                            let dy = y as i32 + iy;
                            if !(0..240).contains(&dx) || !(0..320).contains(&dy) {
                                continue;
                            }
                            let src = ((sy as usize + iy as usize) * atlas.width as usize
                                + sx as usize
                                + ix as usize)
                                * 4;
                            let dst = (dy as usize * 240 + dx as usize) * 4;
                            let a = atlas.bytes[src + 3] as u32;
                            for c in 0..3 {
                                // Match FreeJ2ME's software SRC_OVER approximation for fixture comparison.
                                if a == 255 {
                                    actual[dst + c] = atlas.bytes[src + c];
                                } else if a != 0 {
                                    actual[dst + c] = ((atlas.bytes[src + c] as u32 * a
                                        + actual[dst + c] as u32 * (255 - a))
                                        >> 8)
                                        as u8;
                                }
                            }
                        }
                    }
                }
                Command::Rect {
                    bounds: [x, y, w, h],
                    rgb,
                } => {
                    for yy in (y as i32).max(0)..((y + h) as i32).min(320) {
                        for xx in (x as i32).max(0)..((x + w) as i32).min(240) {
                            let d = (yy as usize * 240 + xx as usize) * 4;
                            actual[d] = (rgb >> 16) as u8;
                            actual[d + 1] = (rgb >> 8) as u8;
                            actual[d + 2] = rgb as u8;
                        }
                    }
                }
            }
        }
        let changed = actual
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.bytes.as_chunks::<4>().0.iter())
            .filter(|(a, b)| a[..3].iter().zip(&b[..3]).any(|(a, b)| a.abs_diff(*b) > 4))
            .count();
        if changed > 0
            && let Ok(path) = std::env::var("PLAYMAN_COMPARE_OUTPUT")
        {
            Image {
                bytes: actual.clone(),
                width: 240,
                height: 320,
            }
            .export_png(&path);
        }
        assert_eq!(
            changed, 0,
            "pose {name}: {changed} pixels differ from original beyond 4-level alpha quantization tolerance"
        );
    }
}
