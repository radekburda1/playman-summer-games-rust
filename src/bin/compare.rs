//! Uses the game's actual world renderer with software primitives, without a window/GPU.
#[allow(dead_code)]
#[path = "../art.rs"]
mod art;
#[path = "../render.rs"]
mod render;
#[path = "../world.rs"]
mod world;
use playman_rust::{Event, Game, Input};
fn main() {
    let output = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "analysis/hurdles/rust".into());
    std::fs::create_dir_all(&output).unwrap();
    let artwork = art::Art::new_software();
    for id in [
        8192, 8195, 8196, 8197, 8200, 8201, 8202, 8204, 8206, 8207, 8208, 16384,
    ] {
        render::clear_background(macroquad::prelude::WHITE);
        artwork.scene_tween(id, 0, 0, 0, 120., 160.);
        render::save_software(&format!("{output}/asset-{id}.png"));
    }
    render::clear_background(macroquad::prelude::WHITE);
    artwork.scene_tween(8201, 0, 0, 0, -257., 173.);
    render::save_software(&format!("{output}/asset-negative-8201.png"));
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
        let mut g = Game::new(Event::Hurdles, 12345);
        for row in csv.lines().skip(1) {
            let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
            match v[1] {
                0 => g.input(Input::Left),
                1 => g.input(Input::Right),
                10 => g.input(Input::ActionDown),
                _ => {}
            }
            if g.finished {
                g.step_finish();
            } else {
                g.step();
            }
            if v[0] % 40 == 0 {
                world::draw(&artwork, &g, false, None, v[0] * 32, Some(5032 + v[0] * 32));
                render::save_software(&format!("{output}/{scenario}-{}.png", v[0]));
            }
        }
    }
    println!("Rendered reference-input replay to {output}");
}

#[test]
fn original_scene_assets_match_software_rendering() {
    use macroquad::prelude::{Image, ImageFormat, WHITE};
    let art = art::Art::new_software();
    for id in [
        8192, 8195, 8196, 8197, 8200, 8201, 8202, 8204, 8206, 8207, 8208, 16384,
    ] {
        render::clear_background(WHITE);
        art.scene_tween(id, 0, 0, 0, 120., 160.);
        let actual = render::software_frame();
        let bytes = std::fs::read(format!(
            "{}/tests/fixtures/hurdles/assets/asset-{id}.png",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        let expected = Image::from_file_with_format(&bytes, Some(ImageFormat::Png)).unwrap();
        let mismatches = actual
            .bytes
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.bytes.as_chunks::<4>().0.iter())
            .filter(|(a, b)| (0..3).any(|i| a[i].abs_diff(b[i]) > 4))
            .count();
        assert_eq!(mismatches, 0, "original scene {id}");
    }
}

#[test]
fn matched_java_world_states_remain_close() {
    use macroquad::prelude::{Image, ImageFormat};
    let artwork = art::Art::new_software();
    for (scenario, tick) in [
        ("clean", 80),
        ("wrong", 160),
        ("boundary", 160),
        ("dive", 320),
    ] {
        let root = env!("CARGO_MANIFEST_DIR");
        let csv = std::fs::read_to_string(format!("{root}/tests/fixtures/hurdles/{scenario}.csv"))
            .unwrap();
        let mut game = Game::new(Event::Hurdles, 12345);
        for row in csv.lines().skip(1).take(tick + 1) {
            let input: i32 = row.split(',').nth(1).unwrap().parse().unwrap();
            match input {
                0 => game.input(Input::Left),
                1 => game.input(Input::Right),
                10 => game.input(Input::ActionDown),
                _ => {}
            }
            if game.finished {
                game.step_finish();
            } else {
                game.step();
            }
        }
        world::draw(
            &artwork,
            &game,
            false,
            None,
            tick as i32 * 32,
            Some(5032 + tick as i32 * 32),
        );
        let actual = render::software_frame();
        let bytes = std::fs::read(format!(
            "{root}/tests/fixtures/hurdles/world/{scenario}-{tick}.png"
        ))
        .unwrap();
        let expected = Image::from_file_with_format(&bytes, Some(ImageFormat::Png)).unwrap();
        let differences = actual
            .bytes
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.bytes.as_chunks::<4>().0.iter())
            .filter(|(a, b)| (0..3).any(|i| a[i].abs_diff(b[i]) > 4))
            .count();
        assert!(
            differences <= 100,
            "{scenario}-{tick}: {differences} pixels differ; inspect original frame before updating the limit"
        );
    }
}
