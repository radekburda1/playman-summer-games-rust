use crate::{App, Screen, art::Art};
use macroquad::prelude::*;
use playman_rust::{Event, Phase, bitmap_font::BitmapFont, presentation::Camera, prompts};
const RED: Color = Color::new(1., 0.035, 0.12, 1.);
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
        art.scaled_sprite(id, 26., 23., 0.82);
    } else {
        art.sprite(0, 0., 0., false);
    }
    art.label(4, title, 236., 7., 2);
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
        draw_rectangle_lines(15., 214., 210., 50., 1., BLACK);
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
        let action = dive_zone
            || matches!(
                g.phase,
                Phase::Approach | Phase::Charge | Phase::VaultPlant | Phase::VaultBend
            )
            || (g.phase == Phase::Air && g.runner.vertical <= 650);
        if action && g.runner.dive.is_none() {
            let (x, y) = Camera::for_game(g).project(
                g.runner.pos >> 8,
                g.runner.height >> 8,
                Camera::athlete_lane(g.event),
            );
            art.sprite(
                if g.phase == Phase::Charge && g.elapsed % 200 < 100 {
                    473
                } else {
                    472
                },
                (x - 14) as f32,
                (y - 124) as f32,
                true,
            );
        }
    }
    footer("P: Pause", "Esc: Menu");
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
