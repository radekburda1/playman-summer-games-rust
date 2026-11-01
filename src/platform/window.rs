use super::audio as sound;
use crate::{
    application::{App, FrameInput, Screen},
    platform::records::{load_records, save_records},
    simulation::Event,
    simulation::Game,
};
use crate::{rendering::art::Art, rendering::ui, rendering::world};
use macroquad::prelude::*;
fn frame_input() -> FrameInput {
    let pressed = |keys: &[KeyCode]| keys.iter().any(|key| is_key_pressed(*key));
    let action_keys = [KeyCode::Space, KeyCode::Key5, KeyCode::Kp5];
    FrameInput {
        enter: pressed(&[KeyCode::Enter]),
        action: pressed(&action_keys),
        action_released: action_keys.iter().any(|key| is_key_released(*key)),
        left: pressed(&[KeyCode::Left, KeyCode::A, KeyCode::Key4, KeyCode::Kp4]),
        right: pressed(&[KeyCode::Right, KeyCode::D, KeyCode::Key6, KeyCode::Kp6]),
        up: pressed(&[KeyCode::Up]),
        down: pressed(&[KeyCode::Down]),
        escape: pressed(&[KeyCode::Escape]),
        pause: pressed(&[KeyCode::P]),
        retry: pressed(&[KeyCode::R]),
    }
}
pub fn config() -> Conf {
    Conf {
        window_title: "Playman Summer Games 3 — Rust reconstruction".into(),
        window_width: 720,
        window_height: 960,
        high_dpi: true,
        ..Default::default()
    }
}
fn draw_game(art: &Art, app: &App, time: f32) {
    let g = &app.game;
    world::draw(
        art,
        &crate::presentation::SceneFrame::new(
            g,
            &app.effects,
            app.screen == Screen::AwaitStart && app.wait_time % 0.6 < 0.3,
            if matches!(app.screen, Screen::AwaitStart | Screen::Ready) {
                Some((app.ready * 1000.) as i32)
            } else {
                None
            },
            (time * 1000.) as i32,
            match app.screen {
                Screen::AwaitStart => None,
                Screen::Ready => Some((app.ready * 1000.) as i32),
                _ => Some(5000 + app.game.elapsed_ms),
            },
        ),
    );
    ui::game_hud(art, app);
}

pub async fn run() {
    let art = Art::new();
    let mut sounds = sound::Sounds::load().await;
    let target = render_target(240, 320);
    target.texture.set_filter(FilterMode::Nearest);
    let mut app = App::new(load_records());
    let smoke = std::env::args().any(|s| s == "--smoke-test");
    let mut frames = 0;
    loop {
        let dt = get_frame_time().min(0.125);
        if is_key_pressed(KeyCode::M) {
            sounds.muted = !sounds.muted;
        }
        let effects = app.update(dt, frame_input());
        if effects.toggle_mute {
            sounds.muted = !sounds.muted;
        }
        for cue in effects.sounds {
            sounds.play(cue);
        }
        if app.records_dirty {
            app.save_error = save_records(&app.records).err().map(|e| e.to_string());
            app.records_dirty = false;
        }
        sounds
            .update_music(1 + (app.seed as usize / 1009) % 3)
            .await;
        for cue in effects.sounds_after_music {
            sounds.play(cue);
        }
        if smoke {
            // Exercise every event renderer and menu/result states without input.
            let phase = (frames / 5) % 8;
            if phase < 5 {
                app.selected = phase;
                app.game = Game::new(Event::ALL[phase], 25);
                app.game.runner.speed = 20000;
                app.screen = Screen::Play;
                app.paused = true;
            } else {
                app.screen = match phase {
                    5 => Screen::Menu,
                    6 => Screen::Help,
                    _ => Screen::Results,
                };
                app.selected = 0;
            }
        }
        set_camera(&Camera2D {
            render_target: Some(target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0., 0., 240., 320.))
        });
        if matches!(
            app.screen,
            Screen::AwaitStart | Screen::Ready | Screen::Play | Screen::Finish | Screen::Results
        ) {
            draw_game(&art, &app, get_time() as f32)
        } else {
            clear_background(WHITE)
        }
        ui::transition_wash(&art, &app);
        ui::draw(&art, &app, sounds.muted);
        set_default_camera();
        clear_background(BLACK);
        let scale = (screen_width() / 240.).min(screen_height() / 320.);
        draw_texture_ex(
            &target.texture,
            (screen_width() - 240. * scale) / 2.,
            (screen_height() - 320. * scale) / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(240. * scale, 320. * scale)),
                flip_y: true,
                ..Default::default()
            },
        );
        frames += 1;
        if smoke && frames >= 40 {
            get_screen_data().export_png("smoke-test.png");
            break;
        }
        next_frame().await;
    }
}
