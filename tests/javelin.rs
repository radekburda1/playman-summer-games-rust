use playman_rust::{application::gameplay, presentation::effects::TimingEffect};
use playman_rust::{presentation::Camera, simulation::Event, simulation::Game, simulation::Input};
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
        let mut game_effects = TimingEffect::default();
        for row in csv.lines().skip(1) {
            let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
            match v[1] {
                0 => gameplay::input(&mut game, &mut game_effects, Input::Left),
                1 => gameplay::input(&mut game, &mut game_effects, Input::Right),
                10 => gameplay::input(&mut game, &mut game_effects, Input::ActionDown),
                11 => gameplay::input(&mut game, &mut game_effects, Input::ActionUp),
                12 => gameplay::input(&mut game, &mut game_effects, Input::ActionDown),
                _ => {}
            }
            if game.finished {
                gameplay::step_finish(&mut game, &mut game_effects);
            } else {
                gameplay::step(&mut game, &mut game_effects);
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
                    .as_javelin()
                    .expect("reference scenario discipline")
                    .projectile
                    .reference_values()
                    .map(|p| p as i32),
            );
            actual.extend([
                game.discipline()
                    .as_javelin()
                    .expect("reference scenario discipline")
                    .charge_angle,
                game.discipline()
                    .as_javelin()
                    .expect("reference scenario discipline")
                    .release_position,
                game.discipline()
                    .as_javelin()
                    .expect("reference scenario discipline")
                    .initial_vertical,
                game.discipline()
                    .as_javelin()
                    .expect("reference scenario discipline")
                    .landing,
                i32::from(
                    game.discipline()
                        .as_javelin()
                        .expect("reference scenario discipline")
                        .foul,
                ),
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
    let mut game_effects = TimingEffect::default();
    for row in input.lines().skip(1) {
        let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
        match v[1] {
            0 => gameplay::input(&mut game, &mut game_effects, Input::Left),
            1 => gameplay::input(&mut game, &mut game_effects, Input::Right),
            10 => gameplay::input(&mut game, &mut game_effects, Input::ActionDown),
            11 => gameplay::input(&mut game, &mut game_effects, Input::ActionUp),
            _ => {}
        }
        if game.finished {
            gameplay::step_finish(&mut game, &mut game_effects);
        } else {
            gameplay::step(&mut game, &mut game_effects);
        }
        let actual: Vec<Vec<i32>> = game_effects
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
        presentation::javelin::ground_segment,
        simulation::math::{cos, sin},
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
