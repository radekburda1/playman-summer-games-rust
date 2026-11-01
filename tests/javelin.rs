use playman_rust::{Event, Game, Input, presentation::Camera};
#[test]
fn javelin_matches_original_states_and_camera() {
    for scenario in [
        "valid",
        "early",
        "late",
        "foul",
        "missed",
        "wrong-run",
        "idle",
        "skip",
    ] {
        let csv = std::fs::read_to_string(format!(
            "{}/tests/fixtures/javelin/{scenario}.csv",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        let mut game = Game::new(Event::Javelin, 12345);
        for row in csv.lines().skip(1) {
            let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
            match v[1] {
                0 => game.input(Input::Left),
                1 => game.input(Input::Right),
                10 => game.input(Input::ActionDown),
                11 => game.input(Input::ActionUp),
                12 => game.input(Input::ActionDown),
                _ => {}
            }
            if game.finished {
                game.step_finish();
            } else {
                game.step();
            }
            let mut actual = vec![
                game.javelin_phase(),
                game.phase_elapsed_ms,
                game.runner.position,
                game.runner.speed,
                game.runner.height,
                game.runner.vertical_velocity,
                game.runner.expected_key as i32,
            ];
            actual.extend(
                game.discipline()
                    .javelin()
                    .projectile
                    .reference_values()
                    .map(|p| p as i32),
            );
            actual.extend([
                game.discipline().javelin().charge_angle,
                game.discipline().javelin().release_position,
                game.discipline().javelin().initial_vertical,
                game.discipline().javelin().landing,
                i32::from(game.discipline().javelin().foul),
            ]);
            let c = Camera::for_game(&game);
            actual.extend([c.x, c.y, c.z, c.angle]);
            if v[2] == 7 || v[2] == 8 {
                assert!(game.finished);
                assert_eq!(
                    game.result,
                    if v[19] != 0 {
                        None
                    } else {
                        Some((i64::from(v[18]) * 100 / 88576) as i32)
                    }
                );
            }
            assert_eq!(actual, &v[2..], "{scenario} tick {}", v[0]);
        }
    }
}

#[test]
fn javelin_particles_match_original() {
    use std::collections::BTreeMap;
    let root = env!("CARGO_MANIFEST_DIR");
    let rows =
        std::fs::read_to_string(format!("{root}/tests/fixtures/javelin/particles.csv")).unwrap();
    let mut expected = BTreeMap::<i32, Vec<Vec<i32>>>::new();
    for row in rows.lines().skip(1) {
        let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
        expected.entry(v[0]).or_default().push(v[1..].to_vec());
    }
    let input =
        std::fs::read_to_string(format!("{root}/tests/fixtures/javelin/valid.csv")).unwrap();
    let mut game = Game::new(Event::Javelin, 12345);
    for row in input.lines().skip(1) {
        let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
        match v[1] {
            0 => game.input(Input::Left),
            1 => game.input(Input::Right),
            10 => game.input(Input::ActionDown),
            11 => game.input(Input::ActionUp),
            _ => {}
        }
        if game.finished {
            game.step_finish();
        } else {
            game.step();
        }
        let actual: Vec<Vec<i32>> = game
            .discipline()
            .javelin()
            .effects
            .particles
            .iter()
            .map(|p| {
                vec![
                    p.preset as i32,
                    p.position[0],
                    p.position[1],
                    p.velocity[0],
                    p.velocity[1],
                    p.origin[0],
                    p.origin[1],
                    p.born,
                    p.lifetime,
                ]
            })
            .collect();
        if let Some(reference) = expected.get(&v[0]) {
            assert_eq!(&actual, reference, "particle tick {}", v[0]);
        }
    }
}

#[test]
fn field_markings_never_wrap_above_the_ground_horizon() {
    use playman_rust::{
        javelin::ground_segment,
        presentation::{cos, sin},
    };
    let csv = include_str!("fixtures/javelin/late.csv");
    let mut clipped = 0;
    let mut visible = 0;
    for row in csv.lines().skip(1) {
        let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
        let camera = Camera::new(v[20], v[21], v[22], v[23]);
        let horizon = 162. + 376. * f64::from(sin(-camera.angle)) / f64::from(cos(-camera.angle));
        let offset = -cos(4096) * 512 / sin(4096);
        for meters in (30..=120).step_by(10) {
            let point = |angle| {
                let r = (meters << 8) - offset;
                [
                    ((((r * cos(angle)) >> 14) + offset) * 346) >> 8,
                    (r * sin(angle)) >> 14,
                ]
            };
            for angle in (-4096..4096).step_by(512) {
                match ground_segment(camera, point(angle), point(angle + 512)) {
                    None => clipped += 1,
                    Some(points) => {
                        for [x, y] in points {
                            visible += 1;
                            assert!(
                                (-0.001..=239.001).contains(&x) && (-0.001..=319.001).contains(&y)
                            );
                            assert!(
                                f64::from(y) >= horizon.floor() - 2.,
                                "ground line in sky at tick {}: y={y}, horizon={horizon}",
                                v[0]
                            );
                        }
                    }
                }
            }
        }
    }
    assert!(
        clipped > 0 && visible > 0,
        "exercise hidden and visible field segments"
    );
}
