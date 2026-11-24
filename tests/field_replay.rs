use playman_rust::{
    application::{App, FrameInput, Screen},
    simulation::{Event, Phase, TICK},
};

fn perform(app: &mut App, foul: bool) {
    app.screen = Screen::Play;
    for tick in 0..2000 {
        let g = &app.game;
        let mut input = FrameInput::default();
        if tick % 7 == 0 {
            input.left = g.runner.expected_key == 0;
            input.right = !input.left;
        }
        if !foul {
            match g.event() {
                Event::LongJump if g.phase() == Phase::Approach && g.runner.position > -50000 => {
                    input.action = true
                }
                Event::LongJump if g.phase() == Phase::Air && g.runner.vertical_velocity <= 650 => {
                    input.action = true
                }
                Event::Javelin if g.phase() == Phase::Approach => input.action = true,
                Event::Javelin if g.phase() == Phase::Charge && g.phase_elapsed_ms >= 1088 => {
                    input.action_released = true
                }
                Event::PoleVault
                    if g.phase() == Phase::VaultPlant
                        && g.discipline().as_vault().unwrap().pole.tip_x
                            + g.discipline().as_vault().unwrap().pole.tip_velocity_x
                            >= -100000 =>
                {
                    input.action = true
                }
                Event::PoleVault
                    if g.phase() == Phase::VaultStraighten
                        && g.discipline().as_vault().unwrap().pole.length >= 4800 =>
                {
                    input.action_released = true
                }
                _ => {}
            }
        }
        app.update(TICK, input);
        if app.screen == Screen::Finish {
            return;
        }
    }
    panic!("attempt did not finish");
}

fn confirm(app: &mut App) {
    app.update(
        0.,
        FrameInput {
            action: true,
            ..Default::default()
        },
    );
}

#[test]
fn each_round_replays_current_attempts_but_results_keep_bests_and_points_award_once() {
    for count in [2, 6] {
        for selected in [1, 3, 4] {
            let mut app = App::new([None; 6]);
            app.multiplayer = true;
            app.player_count = count;
            app.selected = selected;
            app.start();
            let mut bests = vec![None; count];
            for round in 0..3 {
                let mut current = vec![None; count];
                for player in 0..count {
                    assert_eq!(app.current_player, player);
                    assert_eq!(app.attempt, round);
                    perform(&mut app, round == 1);
                    current[player] = app.game.result;
                    if playman_rust::application::records::better(
                        app.game.event(),
                        current[player],
                        bests[player],
                    ) {
                        bests[player] = current[player];
                    }
                    app.advance();
                }
                assert!(app.screen == Screen::FieldReplay);
                assert!(bests.iter().all(Option::is_some));
                let records = app.records;
                let seed = app.seed;
                let live = format!("{:?}", app.game);
                let totals: Vec<_> = app.players.iter().map(|p| p.total).collect();
                for (player, best) in bests.iter().enumerate() {
                    assert_eq!(app.players[player].results[selected], *best);
                    assert_eq!(
                        totals[player],
                        if round == 2 {
                            app.game.event().score(*best)
                        } else {
                            0
                        }
                    );
                }
                let initial: Vec<_> = app
                    .field_replay
                    .as_ref()
                    .unwrap()
                    .athletes
                    .iter()
                    .map(|a| (a.player, format!("{:?}", a.game)))
                    .collect();
                for _ in 0..2200 {
                    app.update(TICK, FrameInput::default());
                }
                let replay = app.field_replay.as_ref().unwrap();
                assert_eq!(replay.athletes.len(), count);
                assert!(replay.complete());
                for a in &replay.athletes {
                    assert_eq!(a.result(), current[a.player]);
                    assert_eq!(a.game.result, current[a.player]);
                    assert_eq!(a.attempt, round);
                }
                app.update(
                    0.,
                    FrameInput {
                        retry: true,
                        ..Default::default()
                    },
                );
                assert_eq!(
                    initial,
                    app.field_replay
                        .as_ref()
                        .unwrap()
                        .athletes
                        .iter()
                        .map(|a| (a.player, format!("{:?}", a.game)))
                        .collect::<Vec<_>>()
                );
                assert_eq!(app.records, records);
                assert_eq!(app.seed, seed);
                assert_eq!(format!("{:?}", app.game), live);
                assert_eq!(
                    app.players.iter().map(|p| p.total).collect::<Vec<_>>(),
                    totals
                );
                confirm(&mut app);
                assert!(app.screen == Screen::RoundResults);
                assert_eq!(
                    app.players.iter().map(|p| p.total).collect::<Vec<_>>(),
                    totals
                );
                confirm(&mut app);
                if round < 2 {
                    assert!(app.screen == Screen::AwaitStart);
                    assert_eq!(app.current_player, 0);
                    assert_eq!(app.attempt, round + 1);
                    assert!(app.field_replay.is_none());
                } else {
                    assert!(app.screen == Screen::Menu);
                }
            }
        }
    }
}

// Reach the first round replay through real attempts; no menu skipping between rounds.
fn finish_round(app: &mut App, foul: bool) {
    for _ in 0..app.player_count {
        perform(app, foul);
        app.advance();
    }
    assert!(app.screen == Screen::FieldReplay);
}

#[test]
fn all_fouls_still_replay_everyone_and_restart_clears_old_event() {
    let mut app = App::new([None; 6]);
    app.multiplayer = true;
    app.selected = 1;
    app.start();
    finish_round(&mut app, true);
    assert!(app.screen == Screen::FieldReplay);
    assert_eq!(app.field_replay.as_ref().unwrap().athletes.len(), 2);
    for _ in 0..2200 {
        app.update(TICK, FrameInput::default());
    }
    assert!(app.field_replay.as_ref().unwrap().complete());
    assert!(
        app.field_replay
            .as_ref()
            .unwrap()
            .athletes
            .iter()
            .all(|a| a.game.result.is_none())
    );
    app.update(
        0.,
        FrameInput {
            escape: true,
            ..Default::default()
        },
    );
    assert!(app.screen == Screen::Menu);
    app.update(
        0.,
        FrameInput {
            action: true,
            ..Default::default()
        },
    );
    app.update(
        0.,
        FrameInput {
            action: true,
            ..Default::default()
        },
    );
    assert!(app.screen == Screen::AwaitStart);
    assert!(app.field_replay.is_none());
}

#[test]
fn early_javelin_throw_stays_visible_in_the_shared_camera() {
    let mut app = App::new([None; 6]);
    app.multiplayer = true;
    app.selected = 4;
    app.start();
    finish_round(&mut app, false);
    for _ in 0..36 {
        app.update(TICK, FrameInput::default());
    }
    for (frame, _) in app.field_replay.as_ref().unwrap().frames(&app.players, 0) {
        let (x, _) = frame.camera.project(frame.game.runner.position >> 8, 0, 0);
        assert!((0..240).contains(&x), "early release outside summary: {x}");
    }
}

#[test]
fn shared_replay_rendering_is_repeatable_and_does_not_advance_any_athlete() {
    use playman_rust::rendering::{art::Art, backend, world};
    let art = Art::new_software();
    for selected in [1, 3, 4] {
        let mut app = App::new([None; 6]);
        app.multiplayer = true;
        app.selected = selected;
        app.start();
        finish_round(&mut app, false);
        for _ in 0..50 {
            app.update(TICK, FrameInput::default());
        }
        let replay = app.field_replay.as_ref().unwrap();
        let state = || {
            replay
                .athletes
                .iter()
                .map(|a| format!("{:?}{:?}", a.game, a.effects))
                .collect::<Vec<_>>()
        };
        let before = state();
        let frames = replay.frames(&app.players, 1600);
        world::draw_field_replay(&art, &frames);
        let first = backend::software_frame().bytes;
        world::draw_field_replay(&art, &frames);
        assert_eq!(first, backend::software_frame().bytes);
        assert_eq!(before, state());
    }
}

#[test]
fn long_jump_summary_keeps_leading_landings_inside_the_screen() {
    let mut app = App::new([None; 6]);
    app.multiplayer = true;
    app.selected = 1;
    app.start();
    finish_round(&mut app, false);
    for _ in 0..120 {
        app.update(TICK, FrameInput::default());
        for (frame, _) in app.field_replay.as_ref().unwrap().frames(&app.players, 0) {
            let (x, _) = frame.camera.project(
                frame.game.runner.position >> 8,
                frame.game.runner.height >> 8,
                0,
            );
            assert!(x <= 160, "landing outside shared view: {x}");
        }
    }
}
