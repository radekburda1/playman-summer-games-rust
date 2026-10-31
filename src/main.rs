mod art;
mod sound;
mod ui;
mod world;
use art::Art;
use macroquad::prelude::*;
use playman_rust::{Event, Game, Input, TICK};
use std::path::PathBuf;
fn config() -> Conf {
    Conf {
        window_title: "Playman Summer Games 3 — Rust reconstruction".into(),
        window_width: 720,
        window_height: 960,
        high_dpi: true,
        ..Default::default()
    }
}
#[derive(Clone, Copy, PartialEq)]
enum Screen {
    Menu,
    Help,
    AwaitStart,
    Ready,
    Play,
    Finish,
    Results,
    Complete,
}
struct App {
    screen: Screen,
    selected: usize,
    game: Game,
    paused: bool,
    clock: f32,
    ready: f32,
    wait_time: f32,
    tournament: bool,
    total: i32,
    attempt: usize,
    best: Option<i32>,
    records: [Option<i32>; 5],
    save_error: Option<String>,
    seed: u64,
    pause_selection: usize,
    screen_age: f32,
    outgoing: Option<(Screen, f32)>,
}
impl App {
    fn new() -> Self {
        Self {
            screen: Screen::Menu,
            selected: 0,
            game: Game::new(Event::Dash, 25),
            paused: false,
            clock: 0.,
            ready: 0.,
            wait_time: 0.,
            tournament: false,
            total: 0,
            attempt: 0,
            best: None,
            records: load_records(),
            save_error: None,
            seed: 25,
            pause_selection: 0,
            screen_age: 0.,
            outgoing: None,
        }
    }
    fn start(&mut self) {
        self.seed = self.seed.wrapping_add(1009);
        self.game = Game::new(Event::ALL[self.selected], self.seed);
        self.outgoing = None;
        self.screen = Screen::AwaitStart;
        self.ready = 0.;
        self.wait_time = 0.;
        self.clock = 0.;
        self.paused = false;
    }
    fn finish(&mut self) {
        let e = self.game.event;
        let result = self.game.result;
        self.game.finish_record = better(e, result, self.records[self.selected]);
        self.game.finish_good = e.score(result) >= 750;
        if better(e, result, self.best) {
            self.best = result
        }
        if better(e, result, self.records[self.selected]) {
            self.records[self.selected] = result;
            self.save_error = save_records(&self.records).err().map(|e| e.to_string());
        }
        self.screen = if e.race() {
            Screen::Finish
        } else {
            Screen::Results
        };
        self.clock = 0.;
    }
    fn advance(&mut self) {
        if !self.game.event.race() && self.attempt < 2 {
            self.attempt += 1;
            self.start();
            return;
        }
        if self.tournament {
            self.total += self.game.event.score(self.best);
            if self.selected < 4 {
                self.selected += 1;
                self.attempt = 0;
                self.best = None;
                self.screen = Screen::Help
            } else {
                self.screen = Screen::Complete
            }
        } else {
            self.screen = Screen::Menu
        }
    }
}
fn better(event: Event, new: Option<i32>, old: Option<i32>) -> bool {
    match (new, old) {
        (Some(a), Some(b)) => {
            if event.race() {
                a < b
            } else {
                a > b
            }
        }
        (Some(_), None) => true,
        _ => false,
    }
}
fn record_path() -> PathBuf {
    // Store beside the executable; failures are surfaced in the result screen.
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("playman-records.txt")))
        .unwrap_or_else(|| PathBuf::from("playman-records.txt"))
}
fn load_records() -> [Option<i32>; 5] {
    let mut r = [None; 5];
    if let Ok(s) = std::fs::read_to_string(record_path()) {
        for (slot, line) in r.iter_mut().zip(s.lines()) {
            *slot = line.parse::<i32>().ok().filter(|n| *n >= 0)
        }
    }
    r
}
fn save_records(r: &[Option<i32>; 5]) -> std::io::Result<()> {
    let path = record_path();
    let temp = path.with_extension("tmp");
    std::fs::write(
        &temp,
        r.iter()
            .map(|r| r.map_or("-".into(), |v| v.to_string()))
            .collect::<Vec<_>>()
            .join("\n"),
    )?;
    std::fs::rename(temp, path)
}
fn draw_game(art: &Art, app: &App, time: f32) {
    let g = &app.game;
    world::draw(
        art,
        g,
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
            _ => Some(5000 + app.game.elapsed),
        },
    );
    ui::game_hud(art, app);
}

#[macroquad::main(config)]
async fn main() {
    let art = Art::new();
    let mut sounds = sound::Sounds::load().await;
    let target = render_target(240, 320);
    target.texture.set_filter(FilterMode::Nearest);
    let mut app = App::new();
    let smoke = std::env::args().any(|s| s == "--smoke-test");
    let mut frames = 0;
    loop {
        let dt = get_frame_time().min(0.125);
        if is_key_pressed(KeyCode::M) {
            sounds.muted = !sounds.muted;
        }
        app.screen_age += dt;
        if let Some((target, elapsed)) = app.outgoing {
            if elapsed + dt >= 0.4 {
                app.screen = target;
                if target == Screen::Ready {
                    sounds.play(23);
                }
                app.screen_age = 0.;
                app.outgoing = None;
            } else {
                app.outgoing = Some((target, elapsed + dt));
            }
        }
        let old_screen = app.screen;
        let old_ready = app.ready;
        let old_feedback = app.game.feedback_time;
        let confirm = app.outgoing.is_none()
            && (is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::Space)
                || is_key_pressed(KeyCode::Key5)
                || is_key_pressed(KeyCode::Kp5));
        let previous = is_key_pressed(KeyCode::Up)
            || is_key_pressed(KeyCode::Left)
            || is_key_pressed(KeyCode::A)
            || is_key_pressed(KeyCode::Key4)
            || is_key_pressed(KeyCode::Kp4);
        let next = is_key_pressed(KeyCode::Down)
            || is_key_pressed(KeyCode::Right)
            || is_key_pressed(KeyCode::D)
            || is_key_pressed(KeyCode::Key6)
            || is_key_pressed(KeyCode::Kp6);
        let esc = is_key_pressed(KeyCode::Escape);
        if esc {
            app.outgoing = None;
            app.screen = Screen::Menu;
            app.paused = false;
            app.clock = 0.;
        }
        match app.screen {
            Screen::Menu => {
                if previous {
                    app.selected = (app.selected + 5) % 6
                }
                if next {
                    app.selected = (app.selected + 1) % 6
                }
                if confirm {
                    app.tournament = app.selected == 5;
                    app.selected = if app.tournament { 0 } else { app.selected };
                    app.total = 0;
                    app.attempt = 0;
                    app.best = None;
                    app.screen = Screen::Help
                }
            }
            Screen::Help => {
                if confirm {
                    app.start()
                }
            }
            Screen::AwaitStart => {
                app.wait_time += dt;
                if confirm {
                    app.outgoing = Some((Screen::Ready, 0.));
                    app.ready = 0.;
                }
            }
            Screen::Ready => {
                app.ready += dt;
                if app.ready >= if app.game.event.race() { 5. } else { 3. } {
                    app.screen = Screen::Play;
                }
            }
            Screen::Play => {
                let was_paused = app.paused;
                if is_key_pressed(KeyCode::P) {
                    app.paused = !app.paused;
                    app.pause_selection = 0;
                    app.clock = 0.
                }
                if app.paused {
                    if previous {
                        app.pause_selection = (app.pause_selection + 3) % 4;
                    }
                    if next {
                        app.pause_selection = (app.pause_selection + 1) % 4;
                    }
                    if confirm {
                        match app.pause_selection {
                            0 => app.paused = false,
                            1 => app.start(),
                            2 => sounds.muted = !sounds.muted,
                            _ => {
                                app.screen = Screen::Menu;
                                app.paused = false;
                            }
                        }
                    }
                }
                if !app.paused && !was_paused && app.screen == Screen::Play {
                    if is_key_pressed(KeyCode::Left)
                        || is_key_pressed(KeyCode::A)
                        || is_key_pressed(KeyCode::Key4)
                        || is_key_pressed(KeyCode::Kp4)
                    {
                        app.game.input(Input::Left)
                    }
                    if is_key_pressed(KeyCode::Right)
                        || is_key_pressed(KeyCode::D)
                        || is_key_pressed(KeyCode::Key6)
                        || is_key_pressed(KeyCode::Kp6)
                    {
                        app.game.input(Input::Right)
                    }
                    if is_key_pressed(KeyCode::Space)
                        || is_key_pressed(KeyCode::Key5)
                        || is_key_pressed(KeyCode::Kp5)
                    {
                        app.game.input(Input::ActionDown)
                    }
                    if is_key_released(KeyCode::Space)
                        || is_key_released(KeyCode::Key5)
                        || is_key_released(KeyCode::Kp5)
                    {
                        app.game.input(Input::ActionUp)
                    }
                    app.clock += dt;
                    while app.clock >= TICK {
                        app.clock -= TICK;
                        app.game.step();
                        if app.game.finished {
                            app.finish();
                            break;
                        }
                    }
                }
            }
            Screen::Finish => {
                app.clock += dt;
                while app.clock >= TICK {
                    app.clock -= TICK;
                    app.game.step_finish();
                }
                if confirm && app.game.runner.speed < 4000 {
                    app.outgoing = Some((Screen::Results, 0.));
                }
            }
            Screen::Results => {
                app.game.phase_time += (dt * 1000.) as i32;
                if is_key_pressed(KeyCode::R) {
                    app.start();
                } else if confirm {
                    app.advance()
                }
            }
            Screen::Complete => {
                if confirm {
                    app.screen = Screen::Menu
                }
            }
        }
        if app.screen != old_screen {
            app.screen_age = 0.;
        }
        if app.screen == Screen::Ready && old_screen != Screen::Ready {
            sounds.play(23)
        }
        if app.screen == Screen::Ready && old_ready < 3. && app.ready >= 3. {
            sounds.play(24)
        }
        if app.screen == Screen::Play && old_screen == Screen::Ready {
            sounds.play(25)
        }
        if matches!(app.screen, Screen::Finish | Screen::Results) && old_screen == Screen::Play {
            sounds.play(if app.game.result.is_some() { 26 } else { 22 })
        }
        if app.game.feedback_time > old_feedback {
            match app.game.feedback.as_str() {
                "WRONG BUTTON" => sounds.play(19),
                "OUCH!" | "TOO EARLY!" => sounds.play(22),
                "STRETCH!" | "DIVE!" | "RELEASE!" => sounds.play(20),
                "CLEAR!" | "PLANT!" => sounds.play(18),
                _ => {}
            }
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
