use playman_rust::{
    application::{App, FrameInput, Screen},
    presentation::{Animation, Camera},
    simulation::{Event, TICK},
};
fn input(app: &App, tick: usize) -> FrameInput {
    if !tick.is_multiple_of(3) {
        return FrameInput::default();
    }
    let key = if app.game.event() == Event::Hurdles {
        app.game.runner.sequence[app.game.runner.sequence_cursor.min(2)]
    } else {
        app.game.runner.expected_key
    };
    FrameInput {
        left: key == 0,
        right: key == 1,
        action: app.game.runner.position > 0x600000,
        ..Default::default()
    }
}
#[test]
fn earlier_races_replay_exact_positions_dives_and_hurdles_without_changing_live_game() {
    for selected in [0, 2] {
        let mut app = App::new([Some(1); 5]);
        app.multiplayer = true;
        app.player_count = 2;
        app.selected = selected;
        app.start();
        app.screen = Screen::Play;
        let mut expected = vec![(app.game.runner.position, Animation::for_game(&app.game), 0)];
        for tick in 0..1900 {
            let mut keys = input(&app, tick);
            if selected == 2 && tick == 0 {
                std::mem::swap(&mut keys.left, &mut keys.right);
            }
            app.update(TICK, keys);
            expected.push((
                app.game.runner.position,
                Animation::for_game(&app.game),
                app.game.discipline().as_hurdles().map_or(0, |h| h.fallen),
            ));
            if app.screen == Screen::Finish {
                break;
            }
        }
        assert!(app.game.finished);
        assert!(app.game.runner.dive.is_some());
        if selected == 2 {
            assert!(expected.iter().any(|sample| sample.2 != 0));
        }
        for _ in 0..200 {
            app.update(TICK, FrameInput::default());
            if app.game.runner.speed < 4000 {
                break;
            }
        }
        app.advance();
        assert_eq!(app.race_athletes().len(), 2);
        assert_eq!(app.race_athletes()[0].position, 0);
        assert_eq!(app.race_athletes()[0].lane, -1166);
        assert_eq!(app.race_athletes()[1].lane, -833);
        app.screen = Screen::Play;
        let before = app.race_athletes()[0].position;
        app.paused = true;
        app.update(0.1, FrameInput::default());
        assert_eq!(app.race_athletes()[0].position, before);
        app.paused = false;
        let mut solo = App::new([Some(1); 5]);
        solo.seed = app.seed - 1009;
        solo.selected = selected;
        solo.start();
        solo.screen = Screen::Play;
        for (tick, &(position, animation, fallen)) in expected.iter().enumerate().skip(1) {
            let keys = input(&app, tick);
            let a = app.update(TICK, keys);
            let b = solo.update(TICK, keys);
            let ghost = &app.race_athletes()[0];
            assert_eq!(ghost.position, position, "event {selected}, tick {tick}");
            assert_eq!(ghost.animation, animation);
            assert_eq!(ghost.fallen, fallen);
            assert_eq!(app.game.runner.position, solo.game.runner.position);
            assert_eq!(a.sounds, b.sounds);
            assert_eq!(a.sounds_after_music, b.sounds_after_music);
        }
    }
}
#[test]
fn lanes_fit_six_players() {
    let mut app = App::new([None; 5]);
    app.multiplayer = true;
    app.player_count = 6;
    for player in 0..6 {
        app.current_player = player;
        let lanes: Vec<_> = (0..=player).map(|i| app.race_lane(i)).collect();
        assert!(lanes.windows(2).all(|w| w[1] - w[0] == 333));
        assert!(lanes.iter().all(|&z| (-1832..=-167).contains(&z)));
        let camera = Camera::race_at(0);
        assert!(lanes.iter().all(|&z| {
            let (_, y) = camera.project(0, 0, z);
            (100..300).contains(&y)
        }));
    }
}
#[test]
fn two_player_reactions_follow_placement_even_for_two_good_times() {
    use playman_rust::assets::resources::animation;
    assert_eq!(
        Animation::race_reaction(0, 2, 4000).offset,
        animation::CELEBRATION_IDLE
    );
    assert_eq!(
        Animation::race_reaction(1, 2, 4000).offset,
        animation::FOUL_IDLE
    );
}

#[test]
fn stopped_replay_extrapolates_reaction_without_retaining_idle_frames() {
    use playman_rust::{
        application::race_replay::{RaceReplay, RaceSample},
        simulation::Game,
    };
    let game = Game::new(Event::Dash, 1);
    let replay = RaceReplay {
        samples: vec![RaceSample::capture(&game)],
        result: Some(1000),
        crossing: None,
    };
    assert_eq!(replay.sample(100).unwrap().1, 3200);
    assert_eq!(replay.samples.len(), 1);
}

#[test]
fn finish_crossing_keeps_active_and_replayed_runners_in_view_without_teleporting() {
    for selected in [0, 2] {
        let mut app = App::new([Some(1); 5]);
        app.multiplayer = true;
        app.player_count = 2;
        app.selected = selected;
        app.start();
        for player in 0..2 {
            app.screen = Screen::Play;
            let mut finish_ticks = 0;
            for tick in 0..2100 {
                let keys = if app.game.finished {
                    FrameInput::default()
                } else {
                    input(&app, tick)
                };
                app.update(TICK, keys);
                let athletes = app.race_athletes();
                let active = athletes.last().unwrap();
                // The old finish staging teleported the camera to 29440 and
                // later the athletes to parked positions. Preserve world motion.
                assert_eq!(active.position, app.game.runner.position);
                let camera = Camera::for_game(&app.game);
                let (x, y) = camera.project(active.position >> 8, active.height >> 8, active.lane);
                assert!(
                    (0..240).contains(&x),
                    "active player off screen at {tick}: {x}"
                );
                assert!((36..303).contains(&y));
                if app.screen == Screen::Finish {
                    finish_ticks += 1;
                    for athlete in &athletes {
                        let (x, _) = camera.project(
                            athlete.position >> 8,
                            athlete.height >> 8,
                            athlete.lane,
                        );
                        assert!(
                            (-60..300).contains(&x),
                            "runner disappears after crossing: {x}"
                        );
                    }
                    if finish_ticks > 100 {
                        break;
                    }
                }
            }
            assert!(finish_ticks > 100);
            if player == 0 {
                app.advance();
            }
        }
    }
}

#[test]
fn original_runout_coasts_then_brakes_to_separate_targets_without_changing_result() {
    use playman_rust::{application::race_replay::stop_target, simulation::Game};
    for event in [Event::Dash, Event::Hurdles] {
        for dive in [false, true] {
            let mut stops = Vec::new();
            for active in [false, true] {
                let mut game = Game::new(event, 1);
                game.finished = true;
                game.result = Some(950);
                game.elapsed_ms = 9728;
                game.runner.position = if event == Event::Dash {
                    0x640000
                } else {
                    0x780000
                };
                game.runner.speed = 20000;
                if dive {
                    game.runner.dive = Some(game.runner.position - 327680);
                }
                let target = stop_target(1034, 0, active, event == Event::Hurdles);
                let start = game.runner.position;
                game.step_race_finish_to(target);
                assert_eq!(
                    game.runner.position - start,
                    if dive { 23900 } else { 20000 }
                );
                assert_eq!(game.runner.speed, 20000); // Coast before braking distance.
                let mut braked = false;
                for _ in 0..250 {
                    let previous = game.runner.position;
                    let speed = game.runner.speed;
                    game.step_race_finish_to(target);
                    assert!((0..=23900).contains(&(game.runner.position - previous)));
                    assert!(game.runner.position <= target);
                    braked |= game.runner.speed < speed;
                    assert_eq!(game.result, Some(950));
                    assert_eq!(game.elapsed_ms, 9728);
                    if game.runner.speed < 0 {
                        break;
                    }
                }
                assert!(braked && game.runner.speed < 0);
                stops.push(game.runner.position);
            }
            assert!((stops[0] - stops[1]).abs() > 100000);
        }
    }
}
