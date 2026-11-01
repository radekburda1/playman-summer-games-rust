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
    std::fs::create_dir_all(format!("{output}/pole-vault")).unwrap();
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
        let mut g = Game::new(Event::PoleVault, 12345);
        for row in csv.lines().skip(1) {
            let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
            match v[1] {
                0 => g.input(Input::Left),
                1 => g.input(Input::Right),
                10 => g.input(Input::ActionDown),
                11 => g.input(Input::ActionUp),
                _ => {}
            }
            if g.finished {
                g.step_finish();
            } else {
                g.step();
            }
            if v[0] % 40 == 0 {
                world::draw(&artwork, &g, false, None, v[0] * 32, None);
                render::save_software(&format!("{output}/pole-vault/{scenario}-{}.png", v[0]));
            }
        }
    }
    std::fs::create_dir_all(format!("{output}/javelin")).unwrap();
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
        let mut g = Game::new(Event::Javelin, 12345);
        for row in csv.lines().skip(1) {
            let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
            match v[1] {
                0 => g.input(Input::Left),
                1 => g.input(Input::Right),
                10 => g.input(Input::ActionDown),
                11 => g.input(Input::ActionUp),
                12 => g.input(Input::ActionDown),
                _ => {}
            }
            if g.finished {
                g.step_finish();
            } else {
                g.step();
            }
            if v[0] % 40 == 0 {
                world::draw(&artwork, &g, false, None, v[0] * 32, None);
                render::save_software(&format!("{output}/javelin/{scenario}-{}.png", v[0]));
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

#[test]
fn carried_pole_matches_original_preparation_and_runup() {
    use macroquad::prelude::{Image, ImageFormat};
    let root = env!("CARGO_MANIFEST_DIR");
    let csv =
        std::fs::read_to_string(format!("{root}/tests/fixtures/pole-vault/clean.csv")).unwrap();
    let art = art::Art::new_software();
    for tick in [0, 80, 120] {
        let mut game = Game::new(Event::PoleVault, 12345);
        for row in csv.lines().skip(1).take(tick + 1) {
            match row.split(',').nth(1).unwrap().parse::<i32>().unwrap() {
                0 => game.input(Input::Left),
                1 => game.input(Input::Right),
                _ => {}
            }
            game.step();
        }
        world::draw(&art, &game, false, None, tick as i32 * 32, None);
        let actual = render::software_frame();
        let bytes = std::fs::read(format!(
            "{root}/tests/fixtures/pole-vault/carry/clean-{tick}.png"
        ))
        .unwrap();
        let expected = Image::from_file_with_format(&bytes, Some(ImageFormat::Png)).unwrap();
        // Compare Java's solid pole palette: unrelated crowd/HUD differences must
        // not hide an absent pole. A missing attachment loses hundreds of pixels.
        let colors = [8045824u32, 22272, 6067456, 18176, 6441289, 4602176];
        let mut pole_pixels = 0;
        let mut mismatches = 0;
        for (a, b) in expected
            .bytes
            .as_chunks::<4>()
            .0
            .iter()
            .zip(actual.bytes.as_chunks::<4>().0.iter())
        {
            let rgb = ((a[0] as u32) << 16) | ((a[1] as u32) << 8) | a[2] as u32;
            if colors.contains(&rgb) {
                pole_pixels += 1;
                if a[..3] != b[..3] {
                    mismatches += 1;
                }
            }
        }
        assert!(
            pole_pixels > 600,
            "reference must contain a visible carried pole"
        );
        assert!(
            mismatches <= 16,
            "tick {tick}: {mismatches}/{pole_pixels} pole pixels differ from Java"
        );
    }
}

#[test]
fn javelin_carried_and_flying_rendering_matches_reference() {
    use macroquad::prelude::{Image, ImageFormat};
    let root = env!("CARGO_MANIFEST_DIR");
    let art = art::Art::new_software();
    let csv = std::fs::read_to_string(format!("{root}/tests/fixtures/javelin/valid.csv")).unwrap();
    for tick in [80, 280] {
        let mut game = Game::new(Event::Javelin, 12345);
        for row in csv.lines().skip(1).take(tick + 1) {
            let v: Vec<i32> = row.split(',').map(|s| s.parse().unwrap()).collect();
            match v[1] {
                0 => game.input(Input::Left),
                1 => game.input(Input::Right),
                10 => game.input(Input::ActionDown),
                11 => game.input(Input::ActionUp),
                12 => game.input(Input::ActionDown),
                _ => {}
            }
            game.step();
        }
        world::draw(&art, &game, false, None, tick as i32 * 32, None);
        let actual = render::software_frame();
        let bytes = std::fs::read(format!(
            "{root}/tests/fixtures/javelin/world/valid-{tick}.png"
        ))
        .unwrap();
        let expected = Image::from_file_with_format(&bytes, Some(ImageFormat::Png)).unwrap();
        let palette = [
            15128068u32,
            5862914,
            0x2e0e00,
            2780928,
            3025408,
            0xfcefef,
            11499374,
            5451825,
        ];
        let mut tested = 0;
        let mut mismatches = 0;
        for (a, b) in expected
            .bytes
            .as_chunks::<4>()
            .0
            .iter()
            .zip(actual.bytes.as_chunks::<4>().0.iter())
        {
            let rgb = ((a[0] as u32) << 16) | ((a[1] as u32) << 8) | a[2] as u32;
            if tick == 280 || palette.contains(&rgb) {
                tested += 1;
                if (0..3).any(|i| a[i].abs_diff(b[i]) > 4) {
                    mismatches += 1;
                }
            }
        }
        assert!(tested > 100, "reference must exercise visible geometry");
        assert!(
            mismatches <= 100,
            "tick {tick}: {mismatches}/{tested} pixels differ from Java"
        );
    }
}
