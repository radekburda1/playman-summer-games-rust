//! Fixed-step mechanics recovered from the supplied Java ME executable.
//! Simulation is independent of the renderer. One step is exactly 32 ms.
pub mod artwork;
pub mod bitmap_font;
pub mod javelin;
pub mod presentation;
pub mod prompts;
pub mod vault;
pub mod vault_effect;
pub const TICK: f32 = 0.032;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    Dash,
    LongJump,
    Hurdles,
    PoleVault,
    Javelin,
}
impl Event {
    pub const ALL: [Self; 5] = [
        Self::Dash,
        Self::LongJump,
        Self::Hurdles,
        Self::PoleVault,
        Self::Javelin,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Self::Dash => "100m Dash",
            Self::LongJump => "Long Jump",
            Self::Hurdles => "110m Hurdles",
            Self::PoleVault => "Pole Vault",
            Self::Javelin => "Javelin",
        }
    }
    pub fn race(self) -> bool {
        matches!(self, Self::Dash | Self::Hurdles)
    }
    pub fn instructions(self) -> &'static str {
        match self {
            Self::Dash => "Follow the blue 4 / green 6 prompts. Press 5 near the finish to dive.",
            Self::LongJump => {
                "Follow 4 / 6. Press 5 before the board, then 5 at the apex to stretch."
            }
            Self::Hurdles => {
                "Complete each three-button 4 / 6 sequence. Hurdle jumps are automatic."
            }
            Self::PoleVault => {
                "Follow 4 / 6. Hold 5 as the pole tip reaches the box; release as it straightens."
            }
            Self::Javelin => {
                "Follow 4 / 6. Hold 5 to charge; release when the marker reaches the tip."
            }
        }
    }
    pub fn finish_message(self, result: Option<i32>) -> &'static str {
        let points = self.score(result).max(0);
        [
            "AWESOME!",
            "GREAT!",
            "NICE!",
            "ALMOST THERE",
            "NOT GOOD ENOUGH",
            "OUCH",
        ][[900, 750, 600, 450, 300]
            .iter()
            .position(|threshold| points >= *threshold)
            .unwrap_or(5)]
    }
    pub fn score(self, result: Option<i32>) -> i32 {
        let Some(n) = result else { return 0 };
        let lerp = |a: i32, b: i32, x: i32, d: i32| (a * (d - x) + b * x) / d;
        match self {
            Self::Dash => {
                if n < 1120 {
                    lerp(600, 1000, 1120 - n, 146)
                } else {
                    lerp(600, 0, n - 1120, 380)
                }
            }
            Self::Hurdles => {
                if n < 1450 {
                    lerp(600, 1000, 1450 - n, 162)
                } else {
                    lerp(600, 0, n - 1450, 270)
                }
            }
            Self::LongJump => {
                if n > 550 {
                    lerp(600, 1000, n - 550, 345)
                } else {
                    lerp(600, 0, 550 - n, 250)
                }
            }
            Self::PoleVault => {
                if n > 500 {
                    lerp(600, 1000, n - 500, 114)
                } else {
                    lerp(600, 0, 500 - n, 300)
                }
            }
            Self::Javelin => {
                if n > 6500 {
                    lerp(600, 1000, n - 6500, 3348)
                } else {
                    lerp(600, 0, 6500 - n, 4500)
                }
            }
        }
        .max(0)
    }
    pub fn result(self, n: Option<i32>) -> String {
        n.map_or("FOUL".into(), |v| {
            format!(
                "{}.{:02} {}",
                v / 100,
                v % 100,
                if self.race() { "s" } else { "m" }
            )
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Prepare,
    LandGood,
    LandBad,
    FoulRun,
    Run,
    Approach,
    Air,
    Stretch,
    BadStretch,
    Charge,
    Throw,
    Flight,
    VaultPlant,
    VaultBend,
    VaultStraighten,
    VaultFall,
    VaultRelease,
    Done,
}
#[derive(Clone, Copy, Debug)]
pub enum Input {
    Left,
    Right,
    ActionDown,
    ActionUp,
}
#[derive(Clone, Debug)]
pub struct Runner {
    pub pos: i32,
    pub speed: i32,
    pub height: i32,
    pub vertical: i32,
    pub expected: usize,
    previous: usize,
    rng: u64,
    ages: [i32; 10],
    pub sequence: [usize; 3],
    pub sequence_index: usize,
    pub mistakes: i32,
    pub previous_mistakes: i32,
    pub tick: i32,
    last_press: i32,
    burst: i32,
    pub dive: Option<i32>,
}
impl Runner {
    pub fn new(seed: u64, pos: i32) -> Self {
        Self {
            pos,
            speed: -1,
            height: 0,
            vertical: 0,
            expected: ((seed >> 6) & 1) as usize,
            previous: ((seed >> 4) & 1) as usize,
            rng: (seed ^ 0x5DEECE66D) & ((1 << 48) - 1),
            ages: [22; 10],
            sequence: [
                ((seed >> 4) & 1) as usize,
                ((seed >> 6) & 1) as usize,
                ((seed >> 7) & 1) as usize,
            ],
            sequence_index: 0,
            mistakes: 0,
            previous_mistakes: 0,
            tick: 0,
            last_press: 0,
            burst: 0,
            dive: None,
        }
    }
    fn randomize(&mut self) {
        self.rng = (self.rng.wrapping_mul(25214903917) + 11) & ((1 << 48) - 1);
    }
    pub fn next_sequence(&mut self) {
        self.randomize();
        self.sequence = [
            ((self.rng >> 4) & 1) as usize,
            ((self.rng >> 6) & 1) as usize,
            ((self.rng >> 7) & 1) as usize,
        ];
        self.sequence_index = 0;
        self.penalty(50 * self.mistakes / 3);
        self.speed = 24000 - 8000 * (self.ages.iter().sum::<i32>() - 80) / 140;
        self.previous_mistakes = self.mistakes;
        self.mistakes = 0;
    }
    fn penalty(&mut self, mut amount: i32) {
        for a in self.ages.iter_mut().rev() {
            let n = (22 - *a).min(amount);
            *a += n;
            amount -= n;
            if amount <= 0 {
                break;
            }
        }
    }
    pub fn press(&mut self, key: usize, hurdles: bool) -> bool {
        if self.speed < 0 {
            self.speed = 0
        }
        if self.tick - self.last_press <= 4 {
            self.burst = (self.burst + 1).min(12)
        } else {
            self.burst -= 1
        }
        self.last_press = self.tick;
        if self.dive.is_some() || hurdles && self.sequence_index >= 3 {
            return false;
        }
        let correct = key
            == if hurdles {
                self.sequence[self.sequence_index]
            } else {
                self.expected
            };
        if correct {
            self.ages.rotate_right(1);
            self.ages[0] = 0;
        } else if hurdles {
            self.mistakes += 1
        } else {
            self.penalty(50)
        }
        if hurdles {
            self.sequence_index += 1
        } else if correct {
            if self.expected == self.previous {
                self.expected = 1 - self.expected
            } else {
                self.previous = self.expected;
                self.randomize();
                self.expected = ((self.rng >> 5) & 1) as usize
            }
        }
        correct
    }
    pub fn run(&mut self, hurdles: bool) {
        if self.burst >= 6 && self.tick - self.last_press > (self.burst - 5) * 4 {
            self.burst = 0
        }
        if self.speed >= 0 {
            let mut budget = if self.burst >= 6 { 30 } else { 10 };
            while budget > 0 {
                if self.ages[0] < 22 {
                    self.ages[0] += 1;
                    budget -= 10
                } else {
                    self.ages.rotate_right(1)
                }
                budget -= 1;
            }
            let sum: i32 = self.ages.iter().sum();
            let base = if hurdles { 4 } else { 8 };
            self.speed = 24000 - 8000 * (sum - 10 * base) / (10 * (22 - base));
            self.move_by(0);
        }
    }
    pub fn move_by(&mut self, gravity: i32) {
        if self.speed >= 0 {
            if let Some(start) = self.dive {
                self.pos += if self.pos - start < 409600 {
                    3900
                } else {
                    -3900
                }
            }
            self.pos += self.speed;
            self.vertical -= gravity;
            self.height += self.vertical;
        }
    }
}
#[derive(Clone, Debug)]
pub struct Game {
    pub event: Event,
    pub runner: Runner,
    pub phase: Phase,
    pub elapsed: i32,
    pub phase_time: i32,
    pub result: Option<i32>,
    pub finished: bool,
    pub finish_good: bool,
    pub finish_record: bool,
    pub takeoff_pos: i32,
    pub sound_events: Vec<usize>,
    pub record_distance: Option<i32>,
    pub landing_good: bool,
    pub feedback: String,
    pub feedback_time: i32,
    pub projectile: [i64; 6],
    pub javelin: javelin::Javelin,
    pub pole: [i32; 12],
    pub vault: vault::Vault,
    pub hurdle_index: usize,
    pub tripped: u32,
    pub peak: i32,
    pub seed: u64,
    pub run_feedback: Option<prompts::RunFeedback>,
}
impl Game {
    pub fn new(event: Event, seed: u64) -> Self {
        let pos = match event {
            Event::LongJump => -3889152,
            Event::Javelin => -4587520,
            Event::PoleVault => -3932160,
            _ => 0,
        };
        let mut g = Self {
            event,
            runner: Runner::new(seed, pos),
            phase: if matches!(event, Event::LongJump | Event::PoleVault | Event::Javelin) {
                Phase::Prepare
            } else {
                Phase::Run
            },
            elapsed: 0,
            phase_time: 0,
            result: None,
            finished: false,
            finish_good: false,
            finish_record: false,
            takeoff_pos: 0,
            sound_events: Vec::new(),
            record_distance: None,
            landing_good: false,
            feedback: String::new(),
            feedback_time: 0,
            projectile: [0; 6],
            javelin: javelin::Javelin::default(),
            pole: [0; 12],
            vault: vault::Vault::default(),
            hurdle_index: 0,
            tripped: 0,
            peak: 0,
            seed,
            run_feedback: None,
        };
        if event == Event::PoleVault {
            let r = g.runner.pos;
            let s = 51200;
            let v = r + sin(55 << 8) * 18;
            let w = s + sin(183 << 8) * 18;
            // r,s,t,u,v,w,x,y,C,B,E,D are the decompiled pole/body model.
            g.pole = [r, s, 0, 0, v, w, 0, 0, w, -5120 + ((v - r) >> 8), 0, 0];
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
        self.phase = Phase::Done;
        self.phase_time = 0;
    }
    pub fn input(&mut self, input: Input) {
        if self.finished {
            return;
        }
        if self.event == Event::Javelin
            && self.phase == Phase::Flight
            && matches!(input, Input::ActionDown)
        {
            while !self.finished {
                self.step();
            }
            return;
        }
        match input {
            Input::Left | Input::Right => {
                if matches!(self.event, Event::LongJump | Event::Javelin)
                    && self.phase == Phase::Prepare
                {
                    self.phase = Phase::Run;
                    self.phase_time = 0;
                    self.runner.speed = 0;
                }
                let hurdle_window = (self.runner.pos + 327680) % 983040;
                let permitted = self.phase == Phase::Run
                    && (self.event != Event::Dash || self.runner.pos < 0x500000)
                    && (self.event != Event::Hurdles || (0..865280).contains(&hurdle_window));
                if permitted {
                    let key = usize::from(matches!(input, Input::Right));
                    let slot = self.runner.sequence_index;
                    let show =
                        self.runner.dive.is_none() && (self.event != Event::Hurdles || slot < 3);
                    let good = self.runner.press(key, self.event == Event::Hurdles);
                    if show {
                        self.run_feedback = Some(prompts::RunFeedback {
                            key,
                            correct: good,
                            at_ms: self.elapsed,
                            slot,
                        });
                    }
                    if matches!(
                        self.event,
                        Event::LongJump | Event::Hurdles | Event::PoleVault | Event::Javelin
                    ) {
                        if !good
                            && matches!(
                                self.event,
                                Event::LongJump | Event::PoleVault | Event::Javelin
                            )
                        {
                            self.sound_events.push(19);
                        }
                    } else {
                        self.message(if good { "NICE!" } else { "WRONG BUTTON" });
                    }
                }
            }
            Input::ActionDown => match self.event {
                Event::Dash | Event::Hurdles => {
                    let min = if self.event == Event::Dash {
                        0x500000
                    } else {
                        0x640000
                    };
                    if self.runner.pos >= min && self.runner.dive.is_none() {
                        self.runner.dive = Some(self.runner.pos);
                        if self.event == Event::Hurdles {
                            self.sound_events.push(20);
                        } else {
                            self.message("DIVE!");
                        }
                    }
                }
                Event::LongJump => {
                    if self.phase == Phase::Approach {
                        self.takeoff_pos = self.runner.pos;
                        if self.runner.pos > 0 {
                            self.runner.speed = 16000 + (self.runner.speed - 16000) / 2;
                            self.phase = Phase::FoulRun;
                            self.phase_time = 0;
                            self.sound_events.push(22);
                            self.message("FOUL");
                            return;
                        }
                        self.phase = Phase::Air;
                        self.sound_events.push(18);
                        self.phase_time = 0;
                        self.runner.vertical = (4000 + self.runner.speed * 300 / 1000).min(13000);
                        self.runner.speed = 16000 + (self.runner.speed - 16000) / 2;
                    } else if self.phase == Phase::Air {
                        if self.runner.vertical - 650 <= 0 {
                            self.runner.vertical += (8000 - self.runner.vertical.abs() / 4).max(0);
                            self.phase = Phase::Stretch;
                            self.phase_time = 0;
                            self.sound_events.push(20);
                            self.message("STRETCH!");
                        } else {
                            self.phase = Phase::BadStretch;
                            self.phase_time = 0;
                            self.sound_events.push(22);
                            self.message("TOO EARLY!")
                        }
                    }
                }
                Event::Javelin => {
                    if self.phase == Phase::Approach {
                        self.phase = Phase::Charge;
                        self.phase_time = 0;
                        self.sound_events.push(18);
                    }
                }
                Event::PoleVault => self.vault_down(),
            },
            Input::ActionUp => match self.event {
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
        if self.event == Event::Javelin {
            self.phase_time += 32;
            self.runner.tick += 1;
            let previous = self.phase;
            self.javelin_step();
            self.javelin_effects_step(previous);
            return;
        }
        if self.event == Event::PoleVault {
            self.phase_time += 32;
            self.runner.tick += 1;
            self.vault_step();
            let marker = vault_effect::marker(self);
            self.vault.timing.step(marker);
            return;
        }
        if self.event == Event::LongJump {
            self.phase_time += 32;
            return;
        }
        if self.event == Event::Hurdles && self.runner.speed >= 0 {
            let adjustment = self.runner.dive.map_or(0, |start| {
                if self.runner.pos - start < 409600 {
                    3900
                } else {
                    -3900
                }
            });
            let ticks = (self.runner.speed - 4000 + adjustment) / 630;
            let distance = ticks * (self.runner.speed + adjustment) - ticks * ticks * 630 / 2;
            if 0x870000 > self.runner.pos + distance {
                self.runner.pos += self.runner.speed + adjustment;
                return;
            }
        }
        if self.runner.speed >= 4000 {
            self.runner.speed -= 630;
            if self.runner.speed < 4000 {
                self.runner.speed = -1;
            } else {
                let dive_adjustment = self.runner.dive.map_or(0, |start| {
                    if self.runner.pos - start < 409600 {
                        3900
                    } else {
                        -3900
                    }
                });
                self.runner.pos += self.runner.speed + dive_adjustment;
            }
        } else {
            self.phase_time += 32;
        }
        if self.event == Event::Hurdles && self.runner.pos > 0x870000 {
            self.runner.pos = 0x870000;
            self.runner.speed = -1;
        }
    }
    pub fn step(&mut self) {
        if self.finished {
            return;
        }
        self.elapsed += 32;
        self.phase_time += 32;
        self.runner.tick += 1;
        self.feedback_time -= 32;
        if self.elapsed > 60000 {
            self.finish(None);
            return;
        }
        match self.event {
            Event::Dash | Event::Hurdles => self.race_step(),
            Event::LongJump => self.jump_step(),
            Event::Javelin => {
                let previous = self.phase;
                self.javelin_step();
                self.javelin_effects_step(previous);
            }
            Event::PoleVault => {
                self.vault_step();
                let marker = vault_effect::marker(self);
                self.vault.timing.step(marker);
            }
        }
        self.peak = self.peak.max(self.runner.height);
    }
    fn race_step(&mut self) {
        let r = &mut self.runner;
        if self.event == Event::Dash {
            if r.pos < 0x500000 {
                r.run(false)
            } else {
                r.move_by(0)
            }
            if r.pos >= 0x640000 {
                self.finish(Some(self.elapsed * 100 / 1024))
            }
        } else {
            let old = r.pos;
            if r.speed < 0 && self.elapsed > 5000 {
                r.speed = 0;
            }
            if r.sequence_index < 3 {
                r.run(true)
            } else {
                r.move_by(0)
            }
            let offset = -117760;
            if (old + 327680 - offset) / 983040 < (r.pos + 327680 - offset) / 983040
                && self.hurdle_index < 7
            {
                if r.sequence_index < 3 && r.mistakes == 0 {
                    r.mistakes = 1
                }
                let missed = r.mistakes > 0;
                if missed {
                    self.tripped |= 1 << self.hurdle_index
                }
                r.next_sequence();
                self.hurdle_index += 1;
                if self.hurdle_index >= 7 {
                    r.sequence_index = 3
                }
                self.sound_events
                    .push(if r.previous_mistakes > 1 { 22 } else { 18 });
            }
            if self.runner.pos >= 0x780000 {
                self.sound_events.push(13);
                self.finish(Some(self.elapsed * 100 / 1024))
            }
        }
    }
    fn jump_step(&mut self) {
        match self.phase {
            Phase::Prepare => {
                if self.phase_time >= 1200 {
                    self.phase = Phase::Run;
                    self.phase_time = 0;
                    self.runner.speed = 0;
                }
            }
            Phase::Run => {
                self.runner.run(false);
                if self.runner.pos >= -1048576 {
                    self.phase = Phase::Approach;
                    self.phase_time = 0;
                }
            }
            Phase::Approach => {
                self.runner.move_by(0);
                if self.runner.pos >= 131072 {
                    self.phase = Phase::FoulRun;
                    self.phase_time = 0;
                    self.sound_events.push(22);
                    self.message("FOUL");
                }
            }
            Phase::FoulRun => {
                self.runner.move_by(0);
                if self.phase_time > 2000 {
                    self.finish(None);
                }
            }
            Phase::Air | Phase::Stretch | Phase::BadStretch => {
                if self.phase == Phase::BadStretch
                    || self.phase == Phase::Air && self.runner.vertical <= 0
                {
                    self.runner.speed = (self.runner.speed - 600).max(0)
                }
                self.runner.move_by(650);
                if self.runner.height < 0 {
                    self.sound_events.push(
                        if self.phase == Phase::Stretch && self.runner.pos >= 0 {
                            26
                        } else {
                            22
                        },
                    );
                    self.runner.height = 0;
                    self.runner.vertical = 0;
                    if self.runner.pos < 0 {
                        self.phase = Phase::FoulRun;
                        self.result = None;
                    } else {
                        self.result = Some(self.runner.pos * 100 / 106496);
                        self.landing_good = self.phase == Phase::Stretch;
                        self.phase = if self.landing_good {
                            Phase::LandGood
                        } else {
                            Phase::LandBad
                        };
                    }
                    self.runner.speed =
                        self.runner
                            .speed
                            .max(if self.landing_good { 20000 } else { 16000 });
                    self.phase_time = 0;
                }
            }
            Phase::LandGood | Phase::LandBad => {
                self.runner.speed = (self.runner.speed - 1400).max(0);
                self.runner.move_by(0);
                let bump = 1080..1200;
                if bump.contains(&self.phase_time) {
                    self.runner.pos += 4000;
                }
                if self.phase_time > 2000 {
                    self.finish(self.result);
                }
            }
            _ => {}
        }
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
        assert_eq!(g.runner.pos, 0);
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
                a.press(a.expected, false);
                b.press(1 - b.expected, false);
            }
            a.run(false);
            b.run(false)
        }
        assert!(a.pos > b.pos);
    }
    #[test]
    fn dive_boost_changes_sign_after_threshold() {
        let mut r = Runner::new(1, 0);
        r.speed = 16000;
        r.dive = Some(0);
        r.move_by(0);
        assert_eq!(r.pos, 19900);
        r.pos = 409600;
        r.move_by(0);
        assert_eq!(r.pos, 421700)
    }
    #[test]
    fn late_jump_is_foul() {
        let mut g = Game::new(Event::LongJump, 1);
        g.phase = Phase::Approach;
        g.runner.pos = 1;
        g.input(Input::ActionDown);
        assert_eq!(g.phase, Phase::FoulRun);
        assert_eq!(g.result, None)
    }
    #[test]
    fn apex_stretch_adds_original_boost() {
        let mut g = Game::new(Event::LongJump, 1);
        g.phase = Phase::Air;
        g.runner.vertical = -100;
        g.input(Input::ActionDown);
        assert_eq!(g.phase, Phase::Stretch);
        assert_eq!(g.runner.vertical, 7875)
    }
    #[test]
    fn all_events_terminate_with_automated_controls() {
        for event in Event::ALL {
            let mut g = Game::new(event, 17);
            for t in 0..2000 {
                if t % 7 == 0 {
                    g.input(
                        if if event == Event::Hurdles {
                            g.runner.sequence[g.runner.sequence_index.min(2)]
                        } else {
                            g.runner.expected
                        } == 0
                        {
                            Input::Left
                        } else {
                            Input::Right
                        },
                    );
                }
                match event {
                    Event::LongJump if g.phase == Phase::Approach && g.runner.pos > -50000 => {
                        g.input(Input::ActionDown)
                    }
                    Event::LongJump if g.phase == Phase::Air && g.runner.vertical <= 650 => {
                        g.input(Input::ActionDown)
                    }
                    Event::Javelin if g.phase == Phase::Approach => g.input(Input::ActionDown),
                    Event::Javelin if g.phase == Phase::Charge && g.phase_time >= 1088 => {
                        g.input(Input::ActionUp)
                    }
                    Event::PoleVault
                        if g.phase == Phase::VaultPlant && g.pole[4] + g.pole[6] >= -100000 =>
                    {
                        g.input(Input::ActionDown)
                    }
                    Event::PoleVault if g.phase == Phase::VaultStraighten && g.pole[11] >= 4800 => {
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
