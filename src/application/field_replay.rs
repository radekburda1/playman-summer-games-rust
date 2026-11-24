//! Isolated field-event playback. Keep inputs, not display-frame snapshots.
//! Java d.bi/bv/bJ rewinds each athlete, consumes their timestamped inputs,
//! and fast-forwards to a rank-staggered position near the jump/throw.
use super::gameplay;
use crate::{
    presentation::effects::TimingEffect,
    simulation::{Event, Game, Input, Phase},
};

#[derive(Clone)]
pub struct Recording {
    initial: Game,
    inputs: Vec<(usize, Input)>,
    ticks: usize,
    pub result: Option<i32>,
    pub attempt: usize,
    pub final_position: i32,
}
impl Recording {
    pub fn new(game: &Game, attempt: usize) -> Self {
        Self {
            initial: game.clone(),
            inputs: Vec::new(),
            ticks: 0,
            result: None,
            attempt,
            final_position: game.runner.position,
        }
    }
    pub fn input(&mut self, input: Input) {
        if self.ticks < 2000 {
            self.inputs.push((self.ticks, input));
        }
    }
    pub fn tick(&mut self) {
        self.ticks += 1;
    }
}

pub struct Athlete {
    pub player: usize,
    pub attempt: usize,
    pub game: Game,
    pub effects: TimingEffect,
    recording: Recording,
    tick: usize,
    cursor: usize,
    recovery: usize,
}
impl Athlete {
    /// This round's measured result, available before playback reaches landing.
    pub fn result(&self) -> Option<i32> {
        self.recording.result
    }
    fn step(&mut self) {
        if self.game.finished {
            if self.recovery < 125 {
                gameplay::step_finish(&mut self.game, &mut self.effects);
                self.recovery += 1;
            }
        } else {
            while let Some(&(tick, input)) = self.recording.inputs.get(self.cursor) {
                if tick > self.tick {
                    break;
                }
                self.cursor += 1;
                // Space during flight is a viewing shortcut, not part of the throw.
                if !(self.game.event() == Event::Javelin
                    && self.game.phase() == Phase::Flight
                    && matches!(input, Input::ActionDown))
                {
                    gameplay::input(&mut self.game, &mut self.effects, input);
                }
            }
            gameplay::step(&mut self.game, &mut self.effects);
            self.tick += 1;
        }
        if self.game.finished {
            self.game.finish_good = self.game.event().score(self.game.result) >= 750;
        }
        // Playback is silent and never feeds the live attempt's audio queue.
        self.game.sound_events.clear();
    }
    fn complete(&self) -> bool {
        self.game.finished && self.recovery >= 125
    }
}

pub struct FieldReplay {
    pub athletes: Vec<Athlete>,
    elapsed: usize,
    early_throw_offset: i32,
}
impl FieldReplay {
    pub fn new(recordings: &[Option<Recording>; 6], count: usize) -> Self {
        let mut order: Vec<_> = (0..count).filter(|&p| recordings[p].is_some()).collect();
        order.sort_by_key(|&p| std::cmp::Reverse(recordings[p].as_ref().unwrap().result));
        let athletes: Vec<Athlete> = order
            .into_iter()
            .enumerate()
            .map(|(rank, player)| {
                let recording = recordings[player].as_ref().unwrap().clone();
                let mut athlete = Athlete {
                    player,
                    attempt: recording.attempt,
                    game: recording.initial.clone(),
                    effects: TimingEffect::default(),
                    recording,
                    tick: 0,
                    cursor: 0,
                    recovery: 0,
                };
                let boundary = if athlete.game.event() == Event::LongJump {
                    -131072
                } else {
                    -524288
                } - 262144 * (count - 1 - rank) as i32;
                // Bound idle/foul lead-ins, and preserve the final second for everyone.
                while athlete.tick < athlete.recording.ticks.saturating_sub(40)
                    && athlete.game.runner.position < boundary
                    && !athlete.game.finished
                    && !(athlete.game.event() == Event::Javelin
                        && matches!(athlete.game.phase(), Phase::Throw | Phase::Flight))
                {
                    athlete.step();
                }
                athlete
            })
            .collect();
        let early_throw_offset = athletes
            .iter()
            .map(|a| a.recording.final_position >> 8)
            .max()
            .map_or(0, |x| (x + 1512).min(0));
        Self {
            athletes,
            elapsed: 0,
            early_throw_offset,
        }
    }
    pub fn frames<'a>(
        &'a self,
        players: &[super::Player],
        scenery_time: i32,
    ) -> Vec<(
        crate::presentation::SceneFrame<'a>,
        crate::assets::character::Appearance,
    )> {
        let camera = self.athletes.first().map(|a| {
            let mut camera = crate::presentation::Camera::field_summary(
                a.game.event(),
                a.recording.result,
                self.elapsed as i32 * 32,
            );
            if a.game.event() == Event::Javelin {
                camera.x += self.early_throw_offset;
            }
            if a.game.event() == Event::LongJump {
                // Keep the leading landing visible while the original pan catches up.
                while camera.x < 16384
                    && self.athletes.iter().any(|a| {
                        camera
                            .project(a.game.runner.position >> 8, a.game.runner.height >> 8, 0)
                            .0
                            > 160
                    })
                {
                    camera.x += 32;
                }
            }
            if a.game.event() == Event::PoleVault {
                while camera.x < 2048
                    && self.athletes.iter().any(|a| {
                        a.game.runner.height > 76800
                            && camera
                                .project(a.game.runner.position >> 8, a.game.runner.height >> 8, 0)
                                .0
                                > 160
                    })
                {
                    camera.x += 32;
                }
                // Custom heads and inverted vault poses need space below the header.
                while camera.y < 4096
                    && self.athletes.iter().any(|a| {
                        a.game.runner.height > 76800
                            && camera
                                .project(a.game.runner.position >> 8, a.game.runner.height >> 8, 0)
                                .1
                                < 130
                    })
                {
                    camera.y += 32;
                }
            }
            camera
        });
        self.athletes
            .iter()
            .enumerate()
            .map(|(index, athlete)| {
                let mut frame = crate::presentation::SceneFrame::new(
                    &athlete.game,
                    &athlete.effects,
                    false,
                    None,
                    scenery_time,
                    None,
                );
                if let Some(camera) = camera {
                    frame.camera = camera;
                    frame.athlete_depth = -80 + 160 * index as i32 / self.athletes.len() as i32;
                }
                (frame, players[athlete.player].appearance)
            })
            .collect()
    }
    pub fn step(&mut self) {
        if self.complete() {
            return;
        }
        self.elapsed += 1;
        for athlete in &mut self.athletes {
            athlete.step();
        }
    }
    pub fn complete(&self) -> bool {
        self.elapsed >= 2100 || self.athletes.iter().all(Athlete::complete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_jump_replays_takeoff_stretch_and_fouls_exactly() {
        for kind in 0..4 {
            let mut game = Game::new(Event::LongJump, 12345);
            game.set_phase(Phase::Approach).unwrap();
            game.runner.position = -65536;
            game.runner.speed = 28000;
            let mut effects = TimingEffect::default();
            let mut recording = Recording::new(&game, 0);
            for tick in 0..2000 {
                let action = tick == 0 && kind != 3
                    || game.phase() == Phase::Air
                        && match kind {
                            0 => game.runner.vertical_velocity <= 650,
                            1 => tick == 3,
                            2 => game.runner.vertical_velocity < -2000,
                            _ => false,
                        };
                if action {
                    recording.input(Input::ActionDown);
                    gameplay::input(&mut game, &mut effects, Input::ActionDown);
                }
                gameplay::step(&mut game, &mut effects);
                recording.tick();
                if game.finished {
                    break;
                }
            }
            assert!(game.finished);
            recording.result = game.result;
            let mut recordings = std::array::from_fn(|_| None);
            recordings[0] = Some(recording);
            let mut replay = FieldReplay::new(&recordings, 1);
            for _ in 0..2100 {
                if replay.athletes[0].game.finished {
                    break;
                }
                replay.step();
            }
            assert!(replay.athletes[0].game.finished);
            game.sound_events.clear();
            game.finish_good = game.event().score(game.result) >= 750;
            assert_eq!(
                format!("{:?}", replay.athletes[0].game),
                format!("{game:?}")
            );
            assert_eq!(
                format!("{:?}", replay.athletes[0].effects),
                format!("{effects:?}")
            );
        }
    }

    #[test]
    fn live_javelin_flight_skip_replays_the_whole_flight() {
        let mut game = Game::new(Event::Javelin, 12345);
        let mut effects = TimingEffect::default();
        let mut recording = Recording::new(&game, 0);
        for row in include_str!("../../tests/fixtures/javelin/skip.csv")
            .lines()
            .skip(1)
        {
            let key: i32 = row.split(',').nth(1).unwrap().parse().unwrap();
            let input = match key {
                0 => Some(Input::Left),
                1 => Some(Input::Right),
                10 | 12 => Some(Input::ActionDown),
                11 => Some(Input::ActionUp),
                _ => None,
            };
            if let Some(input) = input {
                recording.input(input);
                gameplay::input(&mut game, &mut effects, input);
            }
            gameplay::step(&mut game, &mut effects);
            recording.tick();
            if game.finished {
                break;
            }
        }
        assert!(game.finished);
        recording.result = game.result;
        let ticks = recording.ticks;
        let mut recordings = std::array::from_fn(|_| None);
        recordings[0] = Some(recording);
        let mut replay = FieldReplay::new(&recordings, 1);
        for _ in 0..2100 {
            if replay.athletes[0].game.finished {
                break;
            }
            replay.step();
        }
        assert!(replay.athletes[0].game.finished);
        assert!(replay.athletes[0].tick > ticks);
        game.sound_events.clear();
        game.finish_good = game.event().score(game.result) >= 750;
        assert_eq!(
            format!("{:?}", replay.athletes[0].game),
            format!("{game:?}")
        );
        assert_eq!(
            format!("{:?}", replay.athletes[0].effects),
            format!("{effects:?}")
        );
    }

    #[test]
    fn recorded_original_inputs_reproduce_mechanics_and_effects() {
        for (event, folder, scenarios) in [
            (
                Event::PoleVault,
                "pole-vault",
                &["clean", "early", "no-plant", "no-release"][..],
            ),
            (
                Event::Javelin,
                "javelin",
                &["valid", "early", "foul", "skip", "idle"][..],
            ),
        ] {
            for scenario in scenarios {
                let csv = std::fs::read_to_string(format!(
                    "{}/tests/fixtures/{folder}/{scenario}.csv",
                    env!("CARGO_MANIFEST_DIR")
                ))
                .unwrap();
                let mut game = Game::new(event, 12345);
                let mut effects = TimingEffect::default();
                let mut recording = Recording::new(&game, 1);
                for row in csv.lines().skip(1) {
                    let v: Vec<i32> = row.split(',').map(|v| v.parse().unwrap()).collect();
                    let input = match v[1] {
                        0 => Some(Input::Left),
                        1 => Some(Input::Right),
                        10 | 12 => Some(Input::ActionDown),
                        11 => Some(Input::ActionUp),
                        _ => None,
                    };
                    if let Some(input) = input {
                        recording.input(input);
                        // Skip is intentionally omitted from playback; validate natural flight.
                        if v[1] != 12 {
                            gameplay::input(&mut game, &mut effects, input);
                        }
                    }
                    gameplay::step(&mut game, &mut effects);
                    recording.tick();
                    if game.finished {
                        break;
                    }
                }
                while !game.finished && recording.ticks < 2000 {
                    gameplay::step(&mut game, &mut effects);
                    recording.tick();
                }
                assert!(game.finished, "{folder}/{scenario}");
                recording.result = game.result;
                recording.final_position = game.runner.position;
                let mut recordings = std::array::from_fn(|_| None);
                recordings[0] = Some(recording);
                let mut replay = FieldReplay::new(&recordings, 1);
                for _ in 0..2100 {
                    if replay.athletes[0].game.finished {
                        break;
                    }
                    replay.step();
                }
                assert!(replay.athletes[0].game.finished);
                let played = &replay.athletes[0];
                let mut expected = game.clone();
                expected.sound_events.clear();
                expected.finish_good = expected.event().score(expected.result) >= 750;
                assert_eq!(
                    format!("{:?}", played.game),
                    format!("{expected:?}"),
                    "{folder}/{scenario}"
                );
                assert_eq!(
                    format!("{:?}", played.effects),
                    format!("{effects:?}"),
                    "{folder}/{scenario}"
                );
                for _ in 0..200 {
                    replay.step();
                }
                assert!(replay.complete());
            }
        }
    }
}
