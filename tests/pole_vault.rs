use playman_rust::{application::gameplay, presentation::effects::TimingEffect};
use playman_rust::{presentation::Camera, simulation::Event, simulation::Game, simulation::Input};
#[test]
fn vault_matches_original_integer_states() {
    for scenario in [
        "clean",
        "early",
        "late",
        "no-plant",
        "no-release",
        "plant-release",
        "wrong-run",
    ] {
        let csv = std::fs::read_to_string(format!(
            "{}/tests/fixtures/pole-vault/{scenario}.csv",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        let mut game = Game::new(Event::PoleVault, 12345);
        let mut game_effects = TimingEffect::default();
        for row in csv.lines().skip(1) {
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
            let mut actual = vec![
                game.vault_phase(),
                game.phase_elapsed_ms,
                game.runner.position,
                game.runner.speed,
                game.runner.height,
                game.runner.vertical_velocity,
                game.runner.expected_key as i32,
            ];
            actual.extend(
                game.discipline()
                    .as_vault()
                    .expect("reference scenario discipline")
                    .pole
                    .reference_values(),
            );
            actual.extend([
                game.discipline()
                    .as_vault()
                    .expect("reference scenario discipline")
                    .clearance,
                game.discipline()
                    .as_vault()
                    .expect("reference scenario discipline")
                    .fall_height,
                i32::from(
                    game.discipline()
                        .as_vault()
                        .expect("reference scenario discipline")
                        .held,
                ),
                game.discipline()
                    .as_vault()
                    .expect("reference scenario discipline")
                    .camera_offset,
            ]);
            let camera = Camera::for_game(&game);
            actual.extend([camera.x, camera.y, camera.z, camera.angle]);
            if v[2] == 7 {
                assert!(game.finished);
                assert_eq!(
                    game.result,
                    if game
                        .discipline()
                        .as_vault()
                        .expect("reference scenario discipline")
                        .clearance
                        > 0
                    {
                        Some(
                            game.discipline()
                                .as_vault()
                                .expect("reference scenario discipline")
                                .clearance
                                / 855,
                        )
                    } else {
                        None
                    }
                );
            }
            assert_eq!(actual, &v[2..], "{scenario} tick {}", v[0]);
        }
    }
}

#[test]
fn timing_glow_matches_original_particle_states() {
    use std::collections::BTreeMap;
    let root = env!("CARGO_MANIFEST_DIR");
    let csv = std::fs::read_to_string(format!(
        "{root}/tests/fixtures/pole-vault/timing-particles.csv"
    ))
    .unwrap();
    let mut expected = BTreeMap::<i32, Vec<Vec<i32>>>::new();
    for row in csv.lines().skip(1) {
        let values: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
        expected
            .entry(values[0])
            .or_default()
            .push(values[1..].to_vec());
    }
    let input =
        std::fs::read_to_string(format!("{root}/tests/fixtures/pole-vault/clean.csv")).unwrap();
    let mut game = Game::new(Event::PoleVault, 12345);
    let mut game_effects = TimingEffect::default();
    for row in input.lines().skip(1) {
        let values: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
        match values[1] {
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
                let e = &game_effects;
                vec![
                    e.progress,
                    e.anchor[0],
                    e.anchor[1],
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
        if let Some(reference) = expected.get(&values[0]) {
            assert_eq!(&actual, reference, "particle state at tick {}", values[0]);
        } else if values[0] < 143 || values[0] > 224 {
            assert!(actual.is_empty());
        }
    }
}
