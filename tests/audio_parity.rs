use playman_rust::{
    application::{App, FrameInput, Screen},
    simulation::{Event, Game, Input, Phase, TICK},
};

fn original(scenario: &str) -> Vec<(i32, usize)> {
    include_str!("fixtures/audio/original-cues.csv")
        .lines()
        .skip(1)
        .filter_map(|row| {
            let c: Vec<_> = row.split(',').collect();
            (c[0] == scenario).then(|| (c[1].parse().unwrap(), c[2].parse().unwrap()))
        })
        .collect()
}

#[test]
fn dash_finish_uses_executed_original_resource() {
    let mut app = App::new([Some(0); 6]);
    app.screen = Screen::Play;
    app.game.runner.position = 0x640000 - 1000;
    app.game.runner.speed = 16000;
    let effects = app.update(TICK, FrameInput::default());
    assert!(app.screen == Screen::Finish);
    assert_eq!(
        effects
            .sounds
            .iter()
            .map(|s| s.resource_id())
            .collect::<Vec<_>>(),
        original("dash-finish")
            .iter()
            .map(|c| c.1)
            .collect::<Vec<_>>()
    );
}

#[test]
fn jump_player_cues_match_executed_original_ticks() {
    for name in ["jump-valid", "jump-early", "jump-foul"] {
        let mut game = Game::new(Event::LongJump, 1);
        game.set_phase(Phase::Approach).unwrap();
        game.runner.position = if name == "jump-foul" { 1 } else { -65536 };
        game.runner.speed = 28000;
        game.input(Input::ActionDown);
        let mut actual = Vec::new();
        for tick in 0..90 {
            if game.phase() == Phase::Air
                && if name == "jump-early" {
                    tick == 3
                } else {
                    game.runner.vertical_velocity <= 650
                }
            {
                game.input(Input::ActionDown);
            }
            game.step();
            actual.extend(
                game.sound_events
                    .drain(..)
                    .map(|cue| (tick, cue.resource_id())),
            );
        }
        assert_eq!(actual, original(name), "{name}");
    }
}

#[test]
fn countdown_cue_order_matches_with_at_most_one_display_tick_offset() {
    let mut app = App::new([None; 6]);
    app.start();
    app.update(
        0.,
        FrameInput {
            action: true,
            ..Default::default()
        },
    );
    let mut actual = Vec::new();
    for tick in 0..200 {
        let effects = app.update(TICK, FrameInput::default());
        actual.extend(
            effects
                .sounds
                .into_iter()
                .map(|cue| (tick, cue.resource_id())),
        );
    }
    let origin = actual[0].0;
    let expected = original("dash-countdown");
    assert_eq!(actual.len(), expected.len());
    for ((tick, cue), (original_tick, original_cue)) in actual.into_iter().zip(expected) {
        assert_eq!(cue, original_cue);
        assert!((tick - origin - original_tick).abs() <= 1);
    }
}

#[test]
fn good_results_request_original_celebration_once_after_runout() {
    for (result, record, scenario) in [
        (965, 900, "result-good"),
        (1282, 900, "result-poor"),
        (965, 1000, "result-good-record"),
    ] {
        let mut app = App::new([Some(record); 6]);
        app.game.result = Some(result);
        app.game.runner.speed = 10000;
        app.finish();
        assert!(app.update(0., FrameInput::default()).sounds.is_empty());
        app.game.runner.speed = -1;
        let effects = app.update(0., FrameInput::default());
        let expected: Vec<usize> = include_str!("fixtures/audio/result-cues.csv")
            .lines()
            .skip(1)
            .filter_map(|row| {
                let c: Vec<_> = row.split(',').collect();
                (c[0] == scenario).then(|| c[2].parse().unwrap())
            })
            .collect();
        assert_eq!(
            effects
                .sounds
                .iter()
                .map(|s| s.resource_id())
                .collect::<Vec<_>>(),
            expected
        );
        assert!(app.update(0., FrameInput::default()).sounds.is_empty());
    }
}
