use playman_rust::{Event, Game, Input};
#[test]
fn hurdle_races_match_executed_java() {
    for scenario in [
        "clean",
        "wrong",
        "incomplete",
        "boundary",
        "dive",
        "late-dive",
        "idle",
    ] {
        let csv = std::fs::read_to_string(format!(
            "{}/tests/fixtures/hurdles/{scenario}.csv",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        let mut game = Game::new(Event::Hurdles, 12345);
        for row in csv.lines().skip(1) {
            let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
            match v[1] {
                0 => game.input(Input::Left),
                1 => game.input(Input::Right),
                10 => game.input(Input::ActionDown),
                _ => {}
            }
            let was_finished = game.finished;
            if game.finished {
                game.step_finish();
            } else {
                game.step();
            }
            if game.finished && !was_finished {
                assert_eq!(
                    game.result,
                    Some((v[0] + 1) * 32 * 100 / 1024),
                    "{scenario} measured time"
                );
            }
            let actual = [
                game.runner.position,
                game.runner.speed,
                game.runner.sequence_cursor as i32,
                game.runner.mistakes,
                game.runner.previous_mistakes,
                game.discipline().hurdles().fallen as i32,
                game.runner.dive.unwrap_or(-1),
                game.runner.sequence[0] as i32,
                game.runner.sequence[1] as i32,
                game.runner.sequence[2] as i32,
            ];
            assert_eq!(actual, &v[2..12], "{scenario} tick {}", v[0]);
            let camera = playman_rust::presentation::Camera::for_game(&game);
            assert_eq!(
                [camera.x, camera.y, camera.z, camera.angle],
                &v[12..16],
                "{scenario} camera tick {}",
                v[0]
            );
        }
    }
}

#[test]
fn hurdle_input_lock_matches_original_boundaries() {
    for (window, accepted) in [(865279, true), (865280, false), (983039, false), (0, true)] {
        let mut g = Game::new(Event::Hurdles, 1);
        g.runner.position = 983040 + window - 327680;
        let key = if g.runner.sequence[0] == 0 {
            Input::Left
        } else {
            Input::Right
        };
        g.input(key);
        assert_eq!(
            g.runner.sequence_cursor,
            usize::from(accepted),
            "window {window}"
        );
    }
}
#[test]
fn mistake_count_selects_original_hurdle_pose_and_sound() {
    for count in 1..=3 {
        let mut g = Game::new(Event::Hurdles, 1);
        g.runner.position = 537599;
        g.runner.speed = 20000;
        g.runner.sequence_cursor = 3;
        g.runner.mistakes = count;
        g.step();
        assert_eq!(g.runner.previous_mistakes, count);
        assert_eq!(g.discipline().hurdles().fallen, 1);
        assert_eq!(
            g.sound_events
                .iter()
                .map(|cue| cue.resource_id())
                .collect::<Vec<_>>(),
            vec![if count > 1 { 22 } else { 18 }]
        );
        g.runner.position = 2500 << 8;
        assert_eq!(
            playman_rust::presentation::Animation::for_game(&g).offset,
            match count {
                1 => 2912,
                2 => 2958,
                _ => 3004,
            }
        );
    }
}
