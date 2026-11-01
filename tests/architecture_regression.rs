use playman_rust::{application::gameplay, presentation::effects::TimingEffect};
// Current-game preservation fixtures, distinct from Java parity evidence.
use playman_rust::{
    presentation::{Animation, Camera},
    simulation::Event,
    simulation::Game,
    simulation::Input,
    simulation::Phase,
};
use playman_rust::{rendering::art, rendering::backend as render, rendering::world};
fn hash(mut h: u64, bytes: &[u8]) -> u64 {
    for b in bytes {
        h = (h ^ u64::from(*b)).wrapping_mul(1099511628211);
    }
    h
}
fn replay(event: Event, seed: u64) -> (u64, u64) {
    let mut g = Game::new(event, seed);
    let mut g_effects = TimingEffect::default();
    let art = art::Art::new_software();
    let (mut state, mut pixels) = (14695981039346656037, 14695981039346656037);
    for tick in 0..1200 {
        if !g.finished {
            if tick % 7 == 0 {
                let key = if event == Event::Hurdles {
                    g.runner.sequence[g.runner.sequence_cursor.min(2)]
                } else {
                    g.runner.expected_key
                };
                gameplay::input(
                    &mut g,
                    &mut g_effects,
                    if key == 0 { Input::Left } else { Input::Right },
                );
            }
            match event {
                Event::LongJump if g.phase() == Phase::Approach && g.runner.position > -50000 => {
                    gameplay::input(&mut g, &mut g_effects, Input::ActionDown)
                }
                Event::LongJump if g.phase() == Phase::Air && g.runner.vertical_velocity <= 650 => {
                    gameplay::input(&mut g, &mut g_effects, Input::ActionDown)
                }
                Event::PoleVault
                    if g.phase() == Phase::VaultPlant
                        && g.discipline()
                            .as_vault()
                            .expect("reference scenario discipline")
                            .pole
                            .tip_x
                            + g.discipline()
                                .as_vault()
                                .expect("reference scenario discipline")
                                .pole
                                .tip_velocity_x
                            >= -100000 =>
                {
                    gameplay::input(&mut g, &mut g_effects, Input::ActionDown)
                }
                Event::PoleVault
                    if g.phase() == Phase::VaultStraighten
                        && g.discipline()
                            .as_vault()
                            .expect("reference scenario discipline")
                            .pole
                            .length
                            >= 4800 =>
                {
                    gameplay::input(&mut g, &mut g_effects, Input::ActionUp)
                }
                Event::Javelin if g.phase() == Phase::Approach => {
                    gameplay::input(&mut g, &mut g_effects, Input::ActionDown)
                }
                Event::Javelin if g.phase() == Phase::Charge && g.phase_elapsed_ms >= 1088 => {
                    gameplay::input(&mut g, &mut g_effects, Input::ActionUp)
                }
                _ => {}
            }
            gameplay::step(&mut g, &mut g_effects);
        } else {
            gameplay::step_finish(&mut g, &mut g_effects);
        }
        let r = &g.runner;
        let scalars = [
            r.position,
            r.height,
            r.speed,
            r.vertical_velocity,
            r.tick,
            r.expected_key as i32,
            r.sequence_cursor as i32,
            r.mistakes,
            r.previous_mistakes,
            g.elapsed_ms,
            g.phase_elapsed_ms,
            g.result.unwrap_or(-1),
            g.finished as i32,
            g.peak_height,
        ];
        for n in scalars {
            state = hash(state, &n.to_le_bytes());
        }
        state = hash(
            state,
            format!(
                "{:?}{:?}{:?}",
                g.phase(),
                Camera::for_game(&g),
                Animation::for_game(&g)
            )
            .as_bytes(),
        );
        for cue in g.sound_events.drain(..) {
            state = hash(state, &(cue.resource_id() as u64).to_le_bytes());
        }
        if tick % 120 == 0 {
            world::draw(
                &art,
                &playman_rust::presentation::SceneFrame::new(
                    &g,
                    &g_effects,
                    false,
                    None,
                    tick * 32,
                    None,
                ),
            );
            pixels = hash(pixels, &render::software_frame().bytes);
        }
    }
    (state, pixels)
}

#[test]
fn all_five_disciplines_preserve_simulation_and_pixels() {
    let fixture = include_str!("fixtures/refactor.csv");
    let mut lines = fixture.lines().skip(1);
    for event in Event::ALL {
        for seed in [1, 17, 12345] {
            let (state, pixels) = replay(event, seed);
            assert_eq!(
                format!("{event:?},{seed},{state},{pixels}"),
                lines.next().unwrap()
            );
        }
    }
    assert!(lines.next().is_none());
}

#[test]
fn invalid_phase_setup_is_rejected_without_changing_the_event() {
    let mut game = Game::new(Event::Dash, 1);

    assert!(game.set_phase(Phase::Flight).is_err());
    assert!(game.discipline().as_vault().is_none());
    assert!(game.discipline().as_javelin().is_none());
    assert_eq!(game.event(), Event::Dash);
    assert_eq!(game.phase(), Phase::Run);
}

#[test]
fn presentation_is_repeatable_and_does_not_advance_particles_or_simulation() {
    let game = Game::new(Event::PoleVault, 12345);
    let game_effects = TimingEffect::default();
    let before = format!("{game:?}");
    let art = art::Art::new_software();
    let frame =
        playman_rust::presentation::SceneFrame::new(&game, &game_effects, false, None, 320, None);
    world::draw(&art, &frame);
    let first = render::software_frame().bytes;
    world::draw(&art, &frame);
    assert_eq!(first, render::software_frame().bytes);
    assert_eq!(before, format!("{game:?}"));
}
