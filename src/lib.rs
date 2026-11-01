//! Fixed-step mechanics recovered from the supplied Java ME executable.
//! Simulation is independent of the renderer. One step is exactly 32 ms.
use crate::audio::SoundCue;
pub mod artwork;
pub mod bitmap_font;
pub mod effects;
pub mod javelin;
pub mod long_jump;
pub mod presentation;
pub mod prompts;
pub mod race;
pub mod vault;
mod vault_marker;
pub const TICK: f32 = 0.032;
pub use event::{Event, Phase};
mod event;
#[derive(Clone, Copy, Debug)]
pub enum Input {
    Left,
    Right,
    ActionDown,
    ActionUp,
}
pub use runner::Runner;
mod runner;
#[derive(Clone, Debug)]
pub struct Game {
    discipline: discipline::Discipline,
    pub runner: Runner,
    pub elapsed_ms: i32,
    pub phase_elapsed_ms: i32,
    pub result: Option<i32>,
    pub finished: bool,
    pub finish_good: bool,
    pub finish_record: bool,
    pub sound_events: Vec<crate::audio::SoundCue>,
    pub record_distance: Option<i32>,
    pub feedback: String,
    pub feedback_time: i32,
    pub peak_height: i32,
    pub seed: u64,
    pub run_feedback: Option<prompts::RunFeedback>,
}
impl Game {
    pub fn event(&self) -> Event {
        self.discipline.event()
    }
    pub fn discipline(&self) -> &discipline::Discipline {
        &self.discipline
    }
    pub fn phase(&self) -> Phase {
        self.discipline.phase()
    }
    /// Explicit scenario setup for reference tools. Invalid event/phase pairs leave state unchanged.
    pub fn set_phase(&mut self, phase: Phase) -> Result<(), discipline::InvalidPhase> {
        self.discipline.set_phase(phase)
    }
    fn transition(&mut self, phase: Phase) {
        self.set_phase(phase)
            .expect("event transition must belong to this discipline");
    }
    pub fn new(event: Event, seed: u64) -> Self {
        let pos = match event {
            Event::LongJump => -3889152,
            Event::Javelin => -4587520,
            Event::PoleVault => -3932160,
            _ => 0,
        };
        let mut g = Self {
            discipline: discipline::Discipline::new(event),
            runner: Runner::new(seed, pos),
            elapsed_ms: 0,
            phase_elapsed_ms: 0,
            result: None,
            finished: false,
            finish_good: false,
            finish_record: false,
            sound_events: Vec::new(),
            record_distance: None,
            feedback: String::new(),
            feedback_time: 0,
            peak_height: 0,
            seed,
            run_feedback: None,
        };
        if event == Event::PoleVault {
            let r = g.runner.position;
            let s = 51200;
            let v = r + sin(55 << 8) * 18;
            let w = s + sin(183 << 8) * 18;
            // Initial endpoint geometry uses the original integer sine approximation.
            g.discipline.vault_mut().pole = vault::Pole {
                grip_x: r,
                grip_y: s,
                tip_x: v,
                tip_y: w,
                initial_tip_height: w,
                planting_reference: -5120 + ((v - r) >> 8),
                ..Default::default()
            };
        }
        g
    }
    fn message(&mut self, s: &str) {
        self.feedback = s.into();
        self.feedback_time = 1100;
    }
    fn finish(&mut self, r: Option<i32>) {
        self.result = r;
        self.finished = true;
        self.transition(Phase::Done);
        self.phase_elapsed_ms = 0;
    }
    pub fn input(&mut self, input: Input) {
        if self.finished {
            return;
        }
        if self.event() == Event::Javelin
            && self.phase() == Phase::Flight
            && matches!(input, Input::ActionDown)
        {
            while !self.finished {
                self.step();
            }
            return;
        }
        match input {
            Input::Left | Input::Right => {
                if matches!(self.event(), Event::LongJump | Event::Javelin)
                    && self.phase() == Phase::Prepare
                {
                    self.transition(Phase::Run);
                    self.phase_elapsed_ms = 0;
                    self.runner.speed = 0;
                }
                let hurdle_window = (self.runner.position + 327680) % 983040;
                let permitted = self.phase() == Phase::Run
                    && (self.event() != Event::Dash || self.runner.position < 0x500000)
                    && (self.event() != Event::Hurdles || (0..865280).contains(&hurdle_window));
                if permitted {
                    let key = usize::from(matches!(input, Input::Right));
                    let slot = self.runner.sequence_cursor;
                    let show =
                        self.runner.dive.is_none() && (self.event() != Event::Hurdles || slot < 3);
                    let good = self.runner.press(key, self.event() == Event::Hurdles);
                    if show {
                        self.run_feedback = Some(prompts::RunFeedback {
                            key,
                            correct: good,
                            at_ms: self.elapsed_ms,
                            slot,
                        });
                    }
                    if matches!(
                        self.event(),
                        Event::LongJump | Event::Hurdles | Event::PoleVault | Event::Javelin
                    ) {
                        if !good
                            && matches!(
                                self.event(),
                                Event::LongJump | Event::PoleVault | Event::Javelin
                            )
                        {
                            self.sound_events.push(SoundCue::WrongInput);
                        }
                    } else {
                        self.message(if good { "NICE!" } else { "WRONG BUTTON" });
                    }
                }
            }
            Input::ActionDown => match self.event() {
                Event::Dash | Event::Hurdles => self.race_action(),
                Event::LongJump => self.jump_action(),
                Event::Javelin => self.javelin_action(),
                Event::PoleVault => self.vault_down(),
            },
            Input::ActionUp => match self.event() {
                Event::Javelin => self.javelin_release(),
                Event::PoleVault => self.vault_up(),
                _ => {}
            },
        }
    }
    /// Post-finish movement leaves the measured result and race clock untouched.
    pub fn step_finish(&mut self) {
        if !self.finished {
            return;
        }
        if self.event() == Event::Javelin {
            self.phase_elapsed_ms += 32;
            self.runner.tick += 1;
            let previous = self.phase();
            self.javelin_step();
            self.javelin_effects_step(previous);
            return;
        }
        if self.event() == Event::PoleVault {
            self.phase_elapsed_ms += 32;
            self.runner.tick += 1;
            self.vault_step();
            let marker = vault::timing_marker(self);
            self.discipline.vault_mut().timing.step(marker);
            return;
        }
        if self.event() == Event::LongJump {
            self.phase_elapsed_ms += 32;
            return;
        }
        if self.event() == Event::Hurdles && self.runner.speed >= 0 {
            let adjustment = self.runner.dive.map_or(0, |start| {
                if self.runner.position - start < 409600 {
                    3900
                } else {
                    -3900
                }
            });
            let ticks = (self.runner.speed - 4000 + adjustment) / 630;
            let distance = ticks * (self.runner.speed + adjustment) - ticks * ticks * 630 / 2;
            if 0x870000 > self.runner.position + distance {
                self.runner.position += self.runner.speed + adjustment;
                return;
            }
        }
        if self.runner.speed >= 4000 {
            self.runner.speed -= 630;
            if self.runner.speed < 4000 {
                self.runner.speed = -1;
            } else {
                let dive_adjustment = self.runner.dive.map_or(0, |start| {
                    if self.runner.position - start < 409600 {
                        3900
                    } else {
                        -3900
                    }
                });
                self.runner.position += self.runner.speed + dive_adjustment;
            }
        } else {
            self.phase_elapsed_ms += 32;
        }
        if self.event() == Event::Hurdles && self.runner.position > 0x870000 {
            self.runner.position = 0x870000;
            self.runner.speed = -1;
        }
    }
    pub fn step(&mut self) {
        if self.finished {
            return;
        }
        self.elapsed_ms += 32;
        self.phase_elapsed_ms += 32;
        self.runner.tick += 1;
        self.feedback_time -= 32;
        if self.elapsed_ms > 60000 {
            self.finish(None);
            return;
        }
        match self.event() {
            Event::Dash | Event::Hurdles => self.race_step(),
            Event::LongJump => self.jump_step(),
            Event::Javelin => {
                let previous = self.phase();
                self.javelin_step();
                self.javelin_effects_step(previous);
            }
            Event::PoleVault => {
                self.vault_step();
                let marker = vault::timing_marker(self);
                self.discipline.vault_mut().timing.step(marker);
            }
        }
        self.peak_height = self.peak_height.max(self.runner.height);
    }
}
use presentation::sin;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn idle_race_starts_only_on_input_and_times_out() {
        let mut g = Game::new(Event::Dash, 1);
        g.step();
        assert_eq!(g.runner.position, 0);
        for _ in 0..1900 {
            g.step()
        }
        assert!(g.finished);
        assert_eq!(g.result, None)
    }
    #[test]
    fn correct_running_beats_wrong_running() {
        let mut a = Runner::new(4, 0);
        let mut b = a.clone();
        for i in 0..300 {
            a.tick = i;
            b.tick = i;
            if i % 7 == 0 {
                a.press(a.expected_key, false);
                b.press(1 - b.expected_key, false);
            }
            a.run(false);
            b.run(false)
        }
        assert!(a.position > b.position);
    }
    #[test]
    fn dive_boost_changes_sign_after_threshold() {
        let mut r = Runner::new(1, 0);
        r.speed = 16000;
        r.dive = Some(0);
        r.move_by(0);
        assert_eq!(r.position, 19900);
        r.position = 409600;
        r.move_by(0);
        assert_eq!(r.position, 421700)
    }
    #[test]
    fn late_jump_is_foul() {
        let mut g = Game::new(Event::LongJump, 1);
        g.set_phase(Phase::Approach).unwrap();
        g.runner.position = 1;
        g.input(Input::ActionDown);
        assert_eq!(g.phase(), Phase::FoulRun);
        assert_eq!(g.result, None)
    }
    #[test]
    fn apex_stretch_adds_original_boost() {
        let mut g = Game::new(Event::LongJump, 1);
        g.set_phase(Phase::Air).unwrap();
        g.runner.vertical_velocity = -100;
        g.input(Input::ActionDown);
        assert_eq!(g.phase(), Phase::Stretch);
        assert_eq!(g.runner.vertical_velocity, 7875)
    }
    #[test]
    fn all_events_terminate_with_automated_controls() {
        for event in Event::ALL {
            let mut g = Game::new(event, 17);
            for t in 0..2000 {
                if t % 7 == 0 {
                    g.input(
                        if if event == Event::Hurdles {
                            g.runner.sequence[g.runner.sequence_cursor.min(2)]
                        } else {
                            g.runner.expected_key
                        } == 0
                        {
                            Input::Left
                        } else {
                            Input::Right
                        },
                    );
                }
                match event {
                    Event::LongJump
                        if g.phase() == Phase::Approach && g.runner.position > -50000 =>
                    {
                        g.input(Input::ActionDown)
                    }
                    Event::LongJump
                        if g.phase() == Phase::Air && g.runner.vertical_velocity <= 650 =>
                    {
                        g.input(Input::ActionDown)
                    }
                    Event::Javelin if g.phase() == Phase::Approach => g.input(Input::ActionDown),
                    Event::Javelin if g.phase() == Phase::Charge && g.phase_elapsed_ms >= 1088 => {
                        g.input(Input::ActionUp)
                    }
                    Event::PoleVault
                        if g.phase() == Phase::VaultPlant
                            && g.discipline.vault_mut().pole.tip_x
                                + g.discipline.vault_mut().pole.tip_velocity_x
                                >= -100000 =>
                    {
                        g.input(Input::ActionDown)
                    }
                    Event::PoleVault
                        if g.phase() == Phase::VaultStraighten
                            && g.discipline.vault_mut().pole.length >= 4800 =>
                    {
                        g.input(Input::ActionUp)
                    }
                    _ => {}
                }
                g.step();
                if g.finished {
                    break;
                }
            }
            assert!(g.finished, "{event:?}");
            eprintln!("{event:?}: {:?}", g.result);
            assert!(
                g.result.is_some(),
                "automated {event:?} should yield a valid result"
            );
        }
    }
}

pub mod application;
pub mod records;

pub mod discipline;

pub mod audio;

// Runtime rendering adapters; none are called by deterministic simulation.
pub mod art;
pub mod render;
pub mod ui;
pub mod world;

pub mod resources;
