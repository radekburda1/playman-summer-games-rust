use crate::{App, Screen, art::Art};
use macroquad::prelude::*;
use playman_rust::{Event, Phase, bitmap_font::BitmapFont, presentation::Camera, prompts};
const RED: Color = Color::new(1., 0.035, 0.12, 1.);
// Original d.c easing curve, evaluated at 8-bit progress.
fn slide(app: &App) -> f32 {
    let remaining = if let Some((_, elapsed)) = app.outgoing {
        elapsed / 0.4
    } else {
        1. - (app.screen_age / 0.4).min(1.)
    };
    let t = (remaining.clamp(0., 1.) * 256.) as i32;
    let curved = (4 * t * t / 256 - t) / 3;
    (240 * curved / 256) as f32
}
pub fn transition_wash(art: &Art, app: &App) {
    // Java state 10 washes only on entry (bb < 400), never on start confirmation.
    if app.screen == Screen::AwaitStart && app.outgoing.is_some() {
        return;
    }
    if !matches!(
        app.screen,
        Screen::AwaitStart | Screen::Finish | Screen::Results
    ) {
        return;
    }
    let progress = if let Some((_, elapsed)) = app.outgoing {
        1. - elapsed / 0.4
    } else {
        (app.screen_age / 0.4).min(1.)
    };
    if progress >= 1. {
        return;
    }
    let sprite = 109 + (progress.clamp(0., 1.) * 9.) as usize;
    for y in (0..320).step_by(24) {
        for x in (0..240).step_by(24) {
            art.sprite(sprite, x as f32, y as f32, false);
        }
    }
}
fn event_header(art: &Art, app: &App, title: &str) {
    let offset = slide(app);
    draw_rectangle(0., 0., 240., 36., WHITE);
    draw_line(-offset, 36., 240. - offset, 36., 2., RED);
    art.sprite(1, -offset, 0., false);
    art.scaled_sprite(25 + event_index(app.game.event), 33. - offset, 25.5, 0.82);
    art.label(4, title, 230. + offset, 7., 2);
}
fn small(text: &str, x: f32, y: f32, color: Color) {
    draw_text(text, x, y, 14., color);
}
fn band(y: f32, h: f32) {
    draw_triangle(vec2(12., y), vec2(240., y), vec2(240., y + h), RED);
    draw_triangle(vec2(12., y), vec2(240., y + h), vec2(0., y + h), RED);
}
fn header(art: &Art, title: &str, icon: Option<usize>) {
    clear_background(WHITE);
    draw_line(0., 36., 240., 36., 2., RED);
    if let Some(id) = icon {
        art.sprite(1, 0., 0., false);
        art.scaled_sprite(id, 33., 25.5, 0.82);
    } else {
        art.sprite(0, 0., 0., false);
    }
    art.label(4, title, 230., 7., 2);
}
fn footer(left: &str, right: &str) {
    draw_rectangle(0., 303., 240., 17., BLACK);
    small(left, 5., 316., RED);
    let w = measure_text(right, None, 14, 1.).width;
    small(right, 235. - w, 316., RED);
}
fn event_index(event: Event) -> usize {
    Event::ALL.iter().position(|e| *e == event).unwrap_or(0)
}
fn name(selected: usize) -> String {
    if selected == 5 {
        "TOURNAMENT".into()
    } else {
        Event::ALL[selected].name().to_uppercase()
    }
}
fn result(event: Event, value: Option<i32>) -> String {
    value.map_or("FOUL".into(), |n| {
        format!(
            "{}.{:02}{}",
            n / 100,
            n % 100,
            if event.race() { "s" } else { "m" }
        )
    })
}
fn score(art: &Art, event: Event, value: Option<i32>, y: f32) {
    if value.is_some() {
        art.label(6, &result(event, value), 120., y, 1)
    } else {
        art.label(3, "NO RECORD", 120., y + 3., 1)
    }
}
fn wrap(art: &Art, text: &str, x: f32, mut y: f32, width: i32) {
    let font = BitmapFont::get(3);
    let mut line = String::new();
    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.into()
        } else {
            format!("{line} {word}")
        };
        if font.width(&candidate) > width && !line.is_empty() {
            art.label(3, &line, x, y, 0);
            y += 25.;
            line = word.into()
        } else {
            line = candidate
        }
    }
    art.label(3, &line, x, y, 0);
}
fn menu(art: &Art, app: &App) {
    header(art, "SELECT EVENT", None);
    for i in 0..6 {
        let x = 34. + i as f32 * 34.;
        if i == app.selected {
            art.sprite(1, x - 31., 70., false)
        }
        art.scaled_sprite(25 + i, x, 97., if i == app.selected { 1. } else { 0.65 });
    }
    band(128., 29.);
    art.label(3, &format!("< {} >", name(app.selected)), 120., 132., 1);
    if app.selected < 5 {
        art.label(3, "PERSONAL BEST", 120., 185., 1);
        for (x, y, w, h) in [
            (15., 214., 210., 1.),
            (15., 263., 210., 1.),
            (15., 214., 1., 50.),
            (224., 214., 1., 50.),
        ] {
            draw_rectangle(x, y, w, h, BLACK);
        }
        score(
            art,
            Event::ALL[app.selected],
            app.records[app.selected],
            223.,
        );
    } else {
        art.label(3, "ALL FIVE EVENTS", 120., 189., 1);
        art.label(3, "ONE TOURNAMENT", 120., 223., 1);
    }
    art.label(3, "4 / 6 TO CHANGE", 120., 272., 1);
    footer("Select", "Back");
}
fn pause(art: &Art, app: &App, muted: bool) {
    header(art, "PAUSED", None);
    let items = [
        "RESUME GAME",
        "RETRY EVENT",
        if muted { "SOUND: OFF" } else { "SOUND: ON" },
        "MAIN MENU",
    ];
    for (i, item) in items.iter().enumerate() {
        let y = 65. + i as f32 * 29.;
        if i == app.pause_selection {
            band(y - 3., 28.)
        }
        draw_triangle(
            vec2(27., y + 7.),
            vec2(27., y + 17.),
            vec2(35., y + 12.),
            if i == app.pause_selection {
                WHITE
            } else {
                GRAY
            },
        );
        art.label(3, item, 48., y, 0);
    }
    art.label(3, "P TO RESUME", 120., 271., 1);
    footer("Select", "Menu");
}
pub fn game_hud(art: &Art, app: &App) {
    let g = &app.game;
    draw_rectangle(0., 0., 240., 36., WHITE);
    draw_line(0., 36., 240., 36., 1., RED);
    art.scaled_sprite(25 + event_index(g.event), 21., 20., 0.72);
    art.label(3, "PLAYER 1", 42., 5., 0);
    if g.event.race() {
        let n = g.elapsed * 100 / 1024;
        art.label(5, &format!("{}.{:02}", n / 100, n % 100), 235., 9., 2)
    } else {
        art.label(3, &format!("{}/3", app.attempt + 1), 235., 6., 2)
    }
    if g.feedback_time > 0 {
        art.label(3, &g.feedback, 120., 53., 1)
    }
    if app.screen == Screen::Play && !app.paused {
        for p in prompts::running(g) {
            art.sprite(p.sprite, p.x as f32, p.y as f32, true)
        }
        let dive_zone = g.event.race()
            && g.runner.pos
                >= if g.event == Event::Dash {
                    0x500000
                } else {
                    0x640000
                };
        let action = (g.event == Event::PoleVault && g.phase == Phase::VaultPlant && !g.vault.held)
            || dive_zone
            || matches!(g.phase, Phase::Approach | Phase::Charge)
            || (g.phase == Phase::Air && g.runner.vertical <= 650);
        if g.event == Event::LongJump && matches!(g.phase, Phase::BadStretch | Phase::Stretch) {
            let (x, y) = Camera::for_game(g).project(g.runner.pos >> 8, g.runner.height >> 8, 0);
            if g.phase == Phase::BadStretch {
                art.sprite(477, (x - 14) as f32, (y - 107) as f32, true);
            } else if g.phase_time < 200 {
                art.sprite(
                    474 + (g.phase_time * 3 / 200) as usize,
                    (x - 14) as f32,
                    (y - 107) as f32,
                    true,
                );
            }
        }
        if action && g.runner.dive.is_none() {
            let (x, y) = Camera::for_game(g).project(
                g.runner.pos >> 8,
                g.runner.height >> 8,
                Camera::athlete_lane(g.event),
            );
            art.sprite(
                if (g.phase == Phase::Charge || g.event == Event::LongJump && g.phase == Phase::Air)
                    && g.elapsed % 200 < 100
                {
                    473
                } else {
                    472
                },
                (x + if g.event == Event::PoleVault { 61 } else { -14 }) as f32,
                (y - 124) as f32,
                true,
            );
        }
    }
}
pub fn draw(art: &Art, app: &App, muted: bool) {
    match app.screen {
        Screen::Menu => menu(art, app),
        Screen::Help => {
            header(art, "INSTRUCTIONS", Some(25 + app.selected));
            band(55., 29.);
            art.label(3, &name(app.selected), 120., 59., 1);
            wrap(art, Event::ALL[app.selected].instructions(), 16., 100., 208);
            for (i, id) in [470, 471, 472].into_iter().enumerate() {
                art.sprite(id, 49. + i as f32 * 71., 264., true)
            }
            footer("Start", "Back");
        }
        Screen::AwaitStart => {
            event_header(art, app, "NEXT UP");
            let offset = slide(app);
            draw_rectangle(-offset, 54., 240., 27., Color::new(0.08, 0.13, 0.2, 0.75));
            art.label(3, "PLAYER 1", 120. - offset, 57., 1);
            let mut camera = Camera::for_game(&app.game);
            if app.game.event.race() {
                camera.x = 0;
            }
            let (x, y) = camera.project(
                app.game.runner.pos >> 8,
                app.game.runner.height >> 8,
                Camera::athlete_lane(app.game.event),
            );
            if app.outgoing.is_none() && app.screen_age >= 0.4 && app.wait_time % 0.6 < 0.3 {
                art.label(3, "Start", (x + 32) as f32, (y - 132) as f32, 1);
                art.sprite(472, (x + 32) as f32, (y - 92) as f32, true);
            }
            if app.outgoing.is_none() {
                footer("5 / Space: Start", "Esc: Menu");
            }
        }
        Screen::Ready => art.label(
            3,
            if app.ready < 3. {
                "READY"
            } else if app.ready < 4.8 {
                "GET SET"
            } else {
                "GO!"
            },
            120.,
            104.,
            1,
        ),
        Screen::Play => {
            if app.paused {
                pause(art, app, muted)
            }
        }
        Screen::Finish => {
            event_header(art, app, "FINISH");
            let offset = slide(app);
            draw_rectangle(-offset, 54., 240., 27., Color::new(0.08, 0.13, 0.2, 0.75));
            art.label(3, "PLAYER 1", 120. - offset, 57., 1);
            let time = result(app.game.event, app.game.result);
            let font = if app.game.result.is_some() { 6 } else { 3 };
            let width = BitmapFont::get(font).width(&time) as f32;
            art.result_panel(120. + offset - (width as i32 / 2) as f32, 90., width);
            art.label(font, &time, 120. + offset, 87., 1);
            if app.game.event == Event::PoleVault || app.game.runner.speed < 4000 {
                art.label(
                    3,
                    app.game.event.finish_message(app.game.result),
                    120. + offset,
                    127.,
                    1,
                );
                if app.game.finish_record && app.screen_age > 1.2 {
                    let enter = ((app.screen_age - 1.2) / 0.4).clamp(0., 1.);
                    let t = ((1. - enter) * 256.) as i32;
                    let offset = if app.outgoing.is_some() {
                        -slide(app)
                    } else {
                        (240 * ((4 * t * t / 256 - t) / 3) / 256) as f32
                    };
                    let width = BitmapFont::get(4).width("HIGH SCORE!") as f32 + 24.;
                    art.rounded_panel(120. + offset - (width as i32 / 2) as f32, 122., width, 37.);
                    art.label(4, "HIGH SCORE!", 120. + offset, 126., 1);
                }
                if app.outgoing.is_none() {
                    footer("OK", "Menu");
                }
            }
        }
        Screen::Results => {
            header(art, "RESULTS", Some(25 + event_index(app.game.event)));
            band(58., 29.);
            art.label(3, &app.game.event.name().to_uppercase(), 120., 62., 1);
            if app.game.result.is_some() {
                score(art, app.game.event, app.game.result, 109.)
            } else {
                art.label(4, "FOUL", 120., 109., 1)
            }
            art.label(
                3,
                &format!("{} POINTS", app.game.event.score(app.game.result)),
                120.,
                152.,
                1,
            );
            art.label(4, "BEST", 120., 195., 1);
            score(art, app.game.event, app.best, 229.);
            art.label(3, "R: RETRY", 120., 270., 1);
            if app.save_error.is_some() {
                small("Record could not be saved", 30., 299., RED);
            }
            footer("Next", "Menu");
        }
        Screen::Complete => {
            header(art, "RESULTS", Some(30));
            art.label(4, "TOURNAMENT", 120., 82., 1);
            art.label(3, "COMPLETE!", 120., 119., 1);
            art.label(6, &app.total.to_string(), 120., 181., 1);
            art.label(3, "POINTS", 120., 222., 1);
            footer("Menu", "");
        }
    }
}
