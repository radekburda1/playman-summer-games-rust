//! Application flow. Simulation and record decisions require no window or filesystem.
use crate::audio::SoundCue;
use crate::{Event, Game, Input, TICK, records::better};
#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Menu,
    Help,
    AwaitStart,
    Ready,
    Play,
    Finish,
    Results,
    Complete,
}
pub struct App {
    pub screen: Screen,
    pub selected: usize,
    pub game: Game,
    pub paused: bool,
    pub clock: f32,
    pub ready: f32,
    pub wait_time: f32,
    pub tournament: bool,
    pub total: i32,
    pub attempt: usize,
    pub best: Option<i32>,
    pub records: [Option<i32>; 5],
    pub records_dirty: bool,
    pub save_error: Option<String>,
    pub seed: u64,
    pub pause_selection: usize,
    pub screen_age: f32,
    pub outgoing: Option<(Screen, f32)>,
    pub advance_after_transition: bool,
}
impl App {
    pub fn new(records: [Option<i32>; 5]) -> Self {
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
            records,
            records_dirty: false,
            save_error: None,
            seed: 25,
            pause_selection: 0,
            screen_age: 0.,
            outgoing: None,
            advance_after_transition: false,
        }
    }
    pub fn start(&mut self) {
        self.seed = self.seed.wrapping_add(1009);
        self.game = Game::new(Event::ALL[self.selected], self.seed);
        self.game.record_distance = self.records[self.selected];
        self.outgoing = None;
        self.screen = Screen::AwaitStart;
        self.ready = 0.;
        self.wait_time = 0.;
        self.clock = 0.;
        self.paused = false;
    }
    pub fn finish(&mut self) {
        let e = self.game.event();
        let result = self.game.result;
        self.game.finish_record = better(e, result, self.records[self.selected]);
        self.game.finish_good = e.score(result) >= 750;
        if better(e, result, self.best) {
            self.best = result
        }
        if better(e, result, self.records[self.selected]) {
            self.records[self.selected] = result;
            self.records_dirty = true;
        }
        self.screen = Screen::Finish;
        self.clock = 0.;
    }
    pub fn advance(&mut self) {
        if !self.game.event().race() && self.attempt < 2 {
            self.attempt += 1;
            self.start();
            return;
        }
        if self.tournament {
            self.total += self.game.event().score(self.best);
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
#[cfg(test)]
mod flow_tests {
    use super::*;
    #[test]
    fn field_events_have_three_attempts_and_preserve_best() {
        for selected in [1, 3, 4] {
            let mut app = App::new([None; 5]);
            app.selected = selected;
            app.game = Game::new(Event::ALL[selected], 1);
            app.best = Some(720);
            for attempt in 1..=2 {
                app.advance();
                assert_eq!(app.attempt, attempt);
                assert!(app.screen == Screen::AwaitStart);
                assert_eq!(app.best, Some(720));
            }
            app.advance();
            assert!(app.screen == Screen::Menu);
        }
    }
}

/// Edge-triggered platform input, sampled once per display frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameInput {
    pub enter: bool,
    pub action: bool,
    pub action_released: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub escape: bool,
    pub pause: bool,
    pub retry: bool,
}
#[derive(Default)]
pub struct FrameEffects {
    pub sounds: Vec<crate::audio::SoundCue>,
    pub toggle_mute: bool,
    pub sounds_after_music: Vec<SoundCue>,
}
impl App {
    /// Advance display-time flow and dispatch exact 32ms simulation ticks.
    pub fn update(&mut self, dt: f32, input: FrameInput) -> FrameEffects {
        let mut effects = FrameEffects::default();
        self.screen_age += dt;
        if let Some((target, elapsed)) = self.outgoing {
            if elapsed + dt >= 0.4 {
                if self.advance_after_transition {
                    self.advance_after_transition = false;
                    self.advance();
                } else {
                    self.screen = target;
                }
                if target == Screen::Ready && self.game.event().race() {
                    effects.sounds.push(SoundCue::Ready);
                }
                self.screen_age = 0.;
                self.outgoing = None;
            } else {
                self.outgoing = Some((target, elapsed + dt));
            }
        }
        let old_screen = self.screen;
        let old_ready = self.ready;
        let old_feedback = self.game.feedback_time;
        let confirm = self.outgoing.is_none() && (input.enter || input.action);
        let previous = input.up || input.left;
        let next = input.down || input.right;
        let esc = input.escape;
        if esc {
            self.outgoing = None;
            self.advance_after_transition = false;
            self.screen = Screen::Menu;
            self.paused = false;
            self.clock = 0.;
        }
        match self.screen {
            Screen::Menu => {
                if previous {
                    self.selected = (self.selected + 5) % 6
                }
                if next {
                    self.selected = (self.selected + 1) % 6
                }
                if confirm {
                    self.tournament = self.selected == 5;
                    self.selected = if self.tournament { 0 } else { self.selected };
                    self.total = 0;
                    self.attempt = 0;
                    self.best = None;
                    self.screen = Screen::Help
                }
            }
            Screen::Help => {
                if confirm {
                    self.start()
                }
            }
            Screen::AwaitStart => {
                self.wait_time += dt;
                if confirm {
                    self.outgoing = Some((Screen::Ready, 0.));
                    self.ready = 0.;
                }
            }
            Screen::Ready => {
                self.ready += dt;
                if self.ready >= if self.game.event().race() { 5. } else { 0. } {
                    self.screen = Screen::Play;
                }
            }
            Screen::Play => {
                let was_paused = self.paused;
                if input.pause {
                    self.paused = !self.paused;
                    self.pause_selection = 0;
                    self.clock = 0.
                }
                if self.paused {
                    if previous {
                        self.pause_selection = (self.pause_selection + 3) % 4;
                    }
                    if next {
                        self.pause_selection = (self.pause_selection + 1) % 4;
                    }
                    if confirm {
                        match self.pause_selection {
                            0 => self.paused = false,
                            1 => self.start(),
                            2 => effects.toggle_mute = !effects.toggle_mute,
                            _ => {
                                self.screen = Screen::Menu;
                                self.paused = false;
                            }
                        }
                    }
                }
                if !self.paused && !was_paused && self.screen == Screen::Play {
                    if input.left {
                        self.game.input(Input::Left)
                    }
                    if input.right {
                        self.game.input(Input::Right)
                    }
                    if input.action {
                        self.game.input(Input::ActionDown)
                    }
                    if input.action_released {
                        self.game.input(Input::ActionUp)
                    }
                    self.clock += dt;
                    while self.clock >= TICK {
                        self.clock -= TICK;
                        self.game.step();
                        if self.game.finished {
                            self.finish();
                            break;
                        }
                    }
                }
            }
            Screen::Finish => {
                self.clock += dt;
                while self.clock >= TICK {
                    self.clock -= TICK;
                    self.game.step_finish();
                }
                if confirm && (!self.game.event().race() || self.game.runner.speed < 4000) {
                    self.advance_after_transition = !self.game.event().race();
                    self.outgoing = Some((Screen::Results, 0.));
                }
            }
            Screen::Results => {
                self.game.phase_elapsed_ms += (dt * 1000.) as i32;
                if input.retry {
                    self.start();
                } else if confirm {
                    self.advance()
                }
            }
            Screen::Complete => {
                if confirm {
                    self.screen = Screen::Menu
                }
            }
        }
        if self.screen != old_screen {
            self.screen_age = 0.;
        }
        if self.game.event().race() && self.screen == Screen::Ready && old_screen != Screen::Ready {
            effects.sounds.push(SoundCue::Ready)
        }
        if self.screen == Screen::Ready && old_ready < 3. && self.ready >= 3. {
            effects.sounds.push(SoundCue::Set)
        }
        if self.game.event().race() && self.screen == Screen::Play && old_screen == Screen::Ready {
            effects.sounds.push(SoundCue::Gun)
        }
        if self.game.event() == Event::Dash
            && matches!(self.screen, Screen::Finish | Screen::Results)
            && old_screen == Screen::Play
        {
            effects.sounds.push(if self.game.result.is_some() {
                SoundCue::Landing
            } else {
                SoundCue::Failure
            })
        }
        if self.screen == Screen::Finish && old_screen != Screen::Finish && self.game.finish_record
        {
            effects.sounds.push(SoundCue::Record);
        }
        effects
            .sounds_after_music
            .append(&mut self.game.sound_events);
        if self.game.event().race() && self.game.feedback_time > old_feedback {
            let cue = match self.game.feedback.as_str() {
                "WRONG BUTTON" => Some(SoundCue::WrongInput),
                "OUCH!" | "TOO EARLY!" => Some(SoundCue::Failure),
                "STRETCH!" | "DIVE!" | "RELEASE!" => Some(SoundCue::Action),
                "CLEAR!" | "PLANT!" => Some(SoundCue::Takeoff),
                _ => None,
            };
            effects.sounds_after_music.extend(cue);
        }
        effects
    }
}
