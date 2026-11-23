use super::text;
use crate::{
    application::{App, menu},
    rendering::art::Art,
    simulation::Event,
};
use macroquad::prelude::*;

const INK: Color = Color::from_rgba(27, 34, 43, 255);
const MUTED: Color = Color::from_rgba(104, 113, 125, 255);

pub(super) fn draw(art: &Art, app: &App) {
    super::header(art, "SELECT EVENT", None);
    for i in 0..=Event::COUNT {
        let x = 31. + i as f32 * 29.5;
        if i == app.selected {
            art.sprite(1, x - 31., 70., false);
        }
        let icon = if i == Event::COUNT {
            30
        } else if i == 5 {
            25
        } else {
            25 + i
        };
        art.scaled_sprite(icon, x, 97., if i == app.selected { 1. } else { 0.65 });
    }
    super::band(128., 29.);
    art.label(
        3,
        &format!("< {} >", super::name(app.selected)),
        120.,
        132.,
        1,
    );

    draw_rectangle(12., 172., 216., 48., Color::from_rgba(241, 243, 246, 255));
    let (caption, value) = if app.selected == Event::COUNT {
        ("TOURNAMENT", "5 EVENTS".into())
    } else {
        (
            "PERSONAL BEST",
            app.records[app.selected].map_or_else(
                || "NO RECORD".into(),
                |result| {
                    format!(
                        "{}.{:02} {}",
                        result / 100,
                        result % 100,
                        if Event::SELECTABLE[app.selected].race() {
                            "s"
                        } else {
                            "m"
                        }
                    )
                },
            ),
        )
    };
    text(art, caption, vec2(24., 190.), 11., MUTED, 88., 0.);
    text(art, &value, vec2(216., 181.), 27., INK, 100., 1.);
    if app.selected == 5 {
        text(art, "EXPERIMENTAL", vec2(120., 161.), 8., MUTED, 216., 0.5);
    }
    if app.save_error.is_some() {
        text(
            art,
            "Record could not be saved",
            vec2(120., 222.),
            8.,
            super::RED,
            216.,
            0.5,
        );
    }

    let [x, y, width, height] = menu::HANDICAP_PANEL;
    draw_rectangle(x, y, width, height, Color::from_rgba(248, 249, 251, 255));
    text(art, "HANDICAP", vec2(24., 239.), 12., INK, 120., 0.);
    text(
        art,
        &format!("{}%", app.speed_percent()),
        vec2(216., 235.),
        20.,
        INK,
        64.,
        1.,
    );
    let [left, right] = menu::HANDICAP_TRACK;
    let position = left + f32::from(app.speed_percent() - 50) / 50. * (right - left);
    draw_rectangle(
        left,
        263.,
        right - left,
        4.,
        Color::from_rgba(223, 227, 231, 255),
    );
    draw_rectangle(left, 263., position - left, 4., super::RED);
    for stop in 0..=5 {
        let x = left + stop as f32 / 5. * (right - left);
        draw_circle(
            x,
            265.,
            2.,
            if x <= position {
                super::RED
            } else {
                Color::from_rgba(195, 201, 208, 255)
            },
        );
    }
    draw_circle(position, 265., 7., WHITE);
    draw_circle(position, 265., 6., super::RED);
    draw_circle(position, 265., 3., WHITE);
    text(art, "50% · slower", vec2(24., 276.), 8., MUTED, 72., 0.);
    text(art, "100% · normal", vec2(216., 276.), 8., MUTED, 78., 1.);
    draw_line(116., 275., 116., 282., 1., MUTED);
    draw_triangle(vec2(113., 276.), vec2(119., 276.), vec2(116., 273.), MUTED);
    draw_line(124., 273., 124., 280., 1., MUTED);
    draw_triangle(vec2(121., 280.), vec2(127., 280.), vec2(124., 283.), MUTED);
    text(
        art,
        "R  Reset records",
        vec2(120., 292.),
        8.,
        MUTED,
        216.,
        0.5,
    );

    draw_rectangle(0., 303., 240., 17., INK);
    text(
        art,
        "SPACE",
        vec2(10., 308.),
        8.,
        Color::from_rgba(249, 149, 162, 255),
        44.,
        0.,
    );
    text(art, "SELECT", vec2(58., 307.), 10., WHITE, 96., 0.);
    text(
        art,
        "ESC  BACK",
        vec2(230., 308.),
        8.,
        Color::from_rgba(217, 222, 229, 255),
        61.,
        1.,
    );
}
