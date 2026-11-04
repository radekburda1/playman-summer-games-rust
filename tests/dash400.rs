use playman_rust::{
    application::{App, FrameInput, Screen, records},
    presentation::{Camera, prompts},
    simulation::{Event, Game, Input, TICK},
};

#[test]
fn longer_dash_shares_runner_mechanics_but_keeps_prompts_past_100m() {
    let mut short = Game::new(Event::Dash, 1034);
    let mut long = Game::new(Event::Dash400, 1034);
    for tick in 0..250 {
        if tick % 3 == 0 {
            let key = if short.runner.expected_key == 0 {
                Input::Left
            } else {
                Input::Right
            };
            short.input(key);
            long.input(key);
        }
        short.step();
        long.step();
        assert_eq!(short.runner.position, long.runner.position);
        assert_eq!(short.runner.speed, long.runner.speed);
        assert_eq!(short.runner.expected_key, long.runner.expected_key);
        if short.runner.position >= short.event().dive_start() {
            break;
        }
    }
    long.runner.position = 100 << 16;
    long.input(Input::ActionDown);
    assert!(long.runner.dive.is_none());
    assert!(!prompts::running(&long).is_empty());
    long.runner.position = 380 << 16;
    long.input(Input::ActionDown);
    assert!(long.runner.dive.is_some());
    assert!(prompts::running(&long).is_empty());
    long.runner.position = (400 << 16) - 1;
    long.runner.speed = 0;
    long.step();
    assert!(!long.finished);
    long.runner.speed = 20000;
    long.step();
    assert!(long.finished && long.result.is_some());
    let result = long.result;
    let elapsed = long.elapsed_ms;
    for _ in 0..100 {
        long.step_finish();
    }
    assert_eq!(long.result, result);
    assert_eq!(long.elapsed_ms, elapsed);
    assert!(long.runner.position > 400 << 16);
    assert!(long.runner.speed < 4000);
    let (x, _) = Camera::for_game(&long).project(long.runner.position >> 8, 0, -833);
    assert!((0..240).contains(&x));
}

#[test]
fn legacy_records_keep_their_slots_and_400m_has_an_independent_record() {
    assert_eq!(
        records::decode("1000\n600\n1400\n500\n7000"),
        [
            Some(1000),
            Some(600),
            Some(1400),
            Some(500),
            Some(7000),
            None
        ]
    );
    let records = [Some(1000), None, None, None, None, Some(4000)];
    assert_eq!(records::decode(&records::encode(&records)), records);
    assert!(records::better(Event::Dash400, Some(3900), Some(4000)));
    assert_eq!(
        Event::Dash400.score(Some(4000)),
        Event::Dash.score(Some(1000))
    );
}

#[test]
fn multiplayer_400m_finishes_rotates_and_replays_at_the_extended_finish() {
    let mut app = App::new([None; Event::COUNT]);
    app.multiplayer = true;
    app.player_count = 2;
    app.selected = 5;
    app.start();
    for player in 0..2 {
        app.screen = Screen::Play;
        for tick in 0..7500 {
            let key = app.game.runner.expected_key;
            app.update(
                TICK,
                FrameInput {
                    left: tick % 3 == 0 && key == 0,
                    right: tick % 3 == 0 && key == 1,
                    action: app.game.runner.position >= (390 << 16),
                    ..Default::default()
                },
            );
            if app.screen == Screen::Finish {
                break;
            }
        }
        assert!(app.game.finished && app.game.result.is_some());
        assert!(app.game.runner.position >= 400 << 16);
        let elapsed = app.game.elapsed_ms;
        for _ in 0..250 {
            app.update(TICK, FrameInput::default());
        }
        assert_eq!(app.game.elapsed_ms, elapsed);
        assert!(app.game.runner.position >= 400 << 16);
        assert!(app.game.runner.position < 420 << 16);
        assert!(app.game.runner.speed < 4000);
        app.advance();
        assert!(app.players[player].results[5].is_some());
        if player == 0 {
            assert_eq!(app.current_player, 1);
            assert_eq!(app.race_athletes().len(), 2);
        }
    }
    assert!(app.screen == Screen::Standings);
    assert_eq!(app.records[0], None);
    assert!(app.records[5].is_some());
    assert_eq!(app.standings().len(), 2);
}

#[test]
fn event_menu_selects_400m_and_keeps_the_original_tournament() {
    let mut app = App::new([None; Event::COUNT]);
    app.screen = Screen::Menu;
    app.update(
        0.,
        FrameInput {
            left: true,
            ..Default::default()
        },
    );
    assert_eq!(app.selected, Event::COUNT);
    app.update(
        0.,
        FrameInput {
            enter: true,
            ..Default::default()
        },
    );
    assert!(app.tournament);
    assert_eq!(app.selected, 0);
    app.screen = Screen::Menu;
    app.selected = 5;
    app.update(
        0.,
        FrameInput {
            enter: true,
            ..Default::default()
        },
    );
    assert!(!app.tournament);
    app.start();
    assert_eq!(app.game.event(), Event::Dash400);
}
