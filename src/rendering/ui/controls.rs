use crate::rendering::art::Art;
use macroquad::prelude::*;

fn rounded_rect(x: f32, y: f32, width: f32, height: f32, color: Color) {
    draw_rectangle(x + 3., y, width - 6., height, color);
    draw_rectangle(x, y + 3., width, height - 6., color);
    for (cx, cy) in [
        (x + 3., y + 3.),
        (x + width - 3., y + 3.),
        (x + 3., y + height - 3.),
        (x + width - 3., y + height - 3.),
    ] {
        draw_circle(cx, cy, 3., color);
    }
}

/// Replace phone-key labels at the UI boundary. Feedback sprites keep their
/// original positions and timing supplied by presentation::prompts.
pub(super) fn prompt(art: &Art, sprite: usize, x: f32, y: f32) {
    let (width, color) = match sprite {
        470 => (30., Color::from_rgba(43, 113, 184, 255)),
        471 => (30., Color::from_rgba(119, 157, 34, 255)),
        472 => (44., Color::from_rgba(221, 134, 32, 255)),
        473 => (44., Color::from_rgba(245, 172, 62, 255)),
        _ => {
            art.sprite(sprite, x, y, true);
            return;
        }
    };
    let x = if sprite >= 472 {
        x.clamp(width / 2. + 2., 240. - width / 2. - 2.)
    } else {
        x
    };
    rounded_rect(
        x - width / 2.,
        y - 14.,
        width,
        28.,
        Color::from_rgba(27, 34, 43, 255),
    );
    rounded_rect(x - width / 2. + 1., y - 13., width - 2., 23., color);
    if sprite >= 472 {
        super::text(art, "SPACE", vec2(x, y - 5.), 12., WHITE, width - 6., 0.5);
    } else {
        let direction = if sprite == 470 { -1. } else { 1. };
        let tip = x + direction * 7.;
        draw_line(x - direction * 7., y - 1., tip, y - 1., 2., WHITE);
        draw_line(tip, y - 1., tip - direction * 5., y - 6., 2., WHITE);
        draw_line(tip, y - 1., tip - direction * 5., y + 4., 2., WHITE);
    }
}
