use super::text;
use crate::{
    application::{App, Player, Screen},
    rendering::art::Art,
};
use macroquad::prelude::*;

const INK: Color = Color::from_rgba(27, 34, 43, 255);
const MUTED: Color = Color::from_rgba(104, 113, 125, 255);
const GOLD: Color = Color::from_rgba(246, 198, 74, 255);
const ROW: Color = Color::from_rgba(241, 243, 246, 255);
const ALT_ROW: Color = Color::from_rgba(248, 249, 251, 255);

fn rank(art: &Art, place: usize, position: Vec2, winner: bool) {
    draw_circle(
        position.x,
        position.y,
        8.,
        if winner {
            GOLD
        } else {
            Color::from_rgba(229, 233, 238, 255)
        },
    );
    text(
        art,
        &(place + 1).to_string(),
        vec2(position.x, position.y - 5.),
        11.,
        INK,
        14.,
        0.5,
    );
}

fn value(app: &App, player: &Player) -> String {
    let tournament = app.tournament && app.screen != Screen::RoundResults;
    if tournament {
        if player.total >= 1000 {
            format!("{},{:03}", player.total / 1000, player.total % 1000)
        } else {
            player.total.to_string()
        }
    } else {
        player.results[app.selected].map_or_else(
            || "FOUL".into(),
            |result| {
                format!(
                    "{}.{:02} {}",
                    result / 100,
                    result % 100,
                    if app.game.event().race() { "s" } else { "m" }
                )
            },
        )
    }
}

pub(super) fn draw(art: &Art, app: &App) {
    let round = app.screen == Screen::RoundResults;
    let tournament = app.tournament && !round;
    super::header(art, if tournament { "STANDINGS" } else { "RESULTS" }, None);
    let title = if tournament {
        "TOURNAMENT".into()
    } else {
        app.game.event().name().to_uppercase()
    };
    text(art, &title, vec2(12., 55.), 12., INK, 113., 0.);
    let context = if round {
        format!("ROUND {} / 3", app.attempt + 1)
    } else if tournament {
        format!("AFTER {}", app.game.event().name()).to_uppercase()
    } else {
        format!("{} PLAYERS", app.player_count)
    };
    text(art, &context, vec2(228., 57.), 8., MUTED, 94., 1.);

    let order = app.standings();
    let leader = &app.players[order[0]];
    let has_winner = tournament || leader.results[app.selected].is_some();
    if app.player_count == 2 {
        let color = if has_winner { WHITE } else { INK };
        draw_rectangle(
            8.,
            83.,
            224.,
            94.,
            if has_winner { super::RED } else { ROW },
        );
        art.scaled_sprite(leader.appearance.face.portrait(true), 47., 126., 1.06);
        let caption = if !has_winner {
            "NO VALID RESULT"
        } else if round && app.attempt < 2 {
            "BEST SO FAR"
        } else if tournament && app.selected < 4 {
            "LEADING"
        } else {
            "1ST PLACE"
        };
        text(art, caption, vec2(87., 96.), 9., color, 116., 0.);
        text(art, &leader.name, vec2(87., 109.), 18., color, 137., 0.);
        text(
            art,
            &value(app, leader),
            vec2(87., 132.),
            28.,
            color,
            137.,
            0.,
        );
        rank(art, 0, vec2(216., 97.), has_winner);

        let second = &app.players[order[1]];
        draw_rectangle(8., 185., 224., 53., ROW);
        rank(art, 1, vec2(21., 211.), false);
        art.scaled_sprite(second.appearance.face.portrait(true), 49., 211., 0.62);
        text(art, &second.name, vec2(75., 199.), 14., INK, 149., 0.);
        text(
            art,
            &value(app, second),
            vec2(224., 216.),
            18.,
            INK,
            149.,
            1.,
        );
    } else {
        text(art, "#", vec2(21., 74.), 9., MUTED, 16., 0.5);
        text(art, "PLAYER", vec2(64., 74.), 9., MUTED, 90., 0.);
        text(
            art,
            if tournament {
                "TOTAL PTS"
            } else if app.game.event().race() {
                "TIME"
            } else {
                "DISTANCE"
            },
            vec2(224., 74.),
            9.,
            MUTED,
            66.,
            1.,
        );
        for (place, index) in order.into_iter().enumerate() {
            let player = &app.players[index];
            let y = 86. + place as f32 * 34.;
            let winner = place == 0 && has_winner;
            let color = if winner { WHITE } else { INK };
            let background = if winner {
                super::RED
            } else if place % 2 == 1 {
                ROW
            } else {
                ALT_ROW
            };
            draw_rectangle(8., y, 224., 31., background);
            rank(art, place, vec2(21., y + 15.), winner);
            art.scaled_sprite(player.appearance.face.portrait(false), 45., y + 15., 1.);
            text(art, &player.name, vec2(64., y + 10.), 13., color, 90., 0.);
            text(
                art,
                &value(app, player),
                vec2(224., y + 8.),
                16.,
                color,
                64.,
                1.,
            );
        }
    }
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
    text(
        art,
        if round && app.attempt < 2 {
            "NEXT ROUND"
        } else if round && app.tournament {
            "STANDINGS"
        } else if tournament && app.selected < 4 {
            "NEXT EVENT"
        } else {
            "CONTINUE"
        },
        vec2(58., 307.),
        10.,
        WHITE,
        96.,
        0.,
    );
    text(
        art,
        "ESC  MENU",
        vec2(230., 308.),
        8.,
        Color::from_rgba(217, 222, 229, 255),
        61.,
        1.,
    );
}
