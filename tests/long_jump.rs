use playman_rust::{Event, Game, Input, Phase};
#[test]
fn flight_and_landing_match_executed_java_ticks() {
    for (scenario, csv) in [
        ("foul", include_str!("fixtures/long-jump/foul.csv")),
        ("valid", include_str!("fixtures/long-jump/valid.csv")),
        ("early", include_str!("fixtures/long-jump/early.csv")),
        ("late", include_str!("fixtures/long-jump/late.csv")),
    ] {
        let mut game = Game::new(Event::LongJump, 1);
        game.phase = Phase::Approach;
        game.runner.pos = -65536;
        game.runner.speed = 28000;
        if scenario != "foul" {
            game.input(Input::ActionDown);
        }
        for row in csv
            .lines()
            .skip(1)
            .take(if scenario == "foul" { 60 } else { 100 })
        {
            let cells: Vec<_> = row.split(',').collect();
            let tick: i32 = cells[0].parse().unwrap();
            if game.phase == Phase::Air
                && ((scenario == "valid" && game.runner.vertical <= 650)
                    || (scenario == "early" && tick == 3)
                    || (scenario == "late" && game.runner.vertical < -2000))
            {
                game.input(Input::ActionDown);
            }
            game.step();
            if tick % 8 == 0 {
                let path = format!(
                    "{}/tests/fixtures/long-jump/{scenario}-{tick}.json",
                    env!("CARGO_MANIFEST_DIR")
                );
                let json = std::fs::read_to_string(path).unwrap();
                let value = |key: &str| -> i32 {
                    json.split(&format!("\"{key}\":"))
                        .nth(1)
                        .unwrap()
                        .split([',', '}'])
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap()
                };
                let camera = playman_rust::presentation::Camera::for_game(&game);
                assert_eq!(
                    [camera.x, camera.y, camera.z, camera.angle],
                    [
                        value("camera_x"),
                        value("camera_y"),
                        value("camera_z"),
                        value("camera_angle")
                    ],
                    "{scenario} camera tick {tick}"
                );
            }
            for (column, value) in [
                (3, game.runner.pos),
                (4, game.runner.speed),
                (5, game.runner.height),
                (6, game.runner.vertical),
            ] {
                assert_eq!(
                    value,
                    cells[column].parse::<i32>().unwrap(),
                    "{scenario} tick {tick} column {column}"
                );
            }
        }
    }
}

#[test]
fn preparation_can_be_skipped_by_first_running_input() {
    let mut game = Game::new(Event::LongJump, 1);
    let start = game.runner.pos;
    assert_eq!(game.phase, Phase::Prepare);
    for _ in 0..10 {
        game.step();
    }
    assert_eq!(game.runner.pos, start);
    game.input(Input::Left);
    assert_eq!(game.phase, Phase::Run);
}
#[test]
fn board_boundary_and_jump_sound_match_original() {
    for position in [-1, 0, 1] {
        let mut game = Game::new(Event::LongJump, 1);
        game.phase = Phase::Approach;
        game.runner.pos = position;
        game.runner.speed = 28000;
        game.input(Input::ActionDown);
        assert_eq!(
            game.phase,
            if position > 0 {
                Phase::FoulRun
            } else {
                Phase::Air
            }
        );
        assert_eq!(game.sound_events, vec![if position > 0 { 22 } else { 18 }]);
    }
}
