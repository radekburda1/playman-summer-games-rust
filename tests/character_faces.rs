use playman_rust::{
    application::{App, FrameInput, Screen},
    assets::{
        artwork::Artwork,
        character::{Appearance, Face},
    },
    presentation::Animation,
    simulation::{Event, Game},
};
#[test]
fn all_character_choices_cycle_and_have_portraits_and_animated_views() {
    let art = Artwork::new();
    let default = Face::default();
    let mut face = default;
    for _ in 0..Face::COUNT {
        assert!(art.sprite(face.portrait(false), 0, 0., 0., false).is_some());
        assert!(art.sprite(face.portrait(true), 0, 0., 0., false).is_some());
        for sprite in 794..798 {
            assert!(
                art.sprite(face.head_sprite(sprite), 0, 0., 0., false)
                    .is_some()
            );
        }
        assert_eq!(face.next().previous(), face);
        face = face.next();
    }
    assert_eq!(face, default);
}
#[test]
fn character_banks_preserve_default_rendering_and_equipment_hooks() {
    let art = Artwork::new();
    for event in Event::SELECTABLE {
        let game = Game::new(event, 1);
        let (a, b, f) = Animation::for_game(&game).frames(include_bytes!("../assets/tables.bin"));
        let default = art.frame(0, a, b, f, 120., 230.);
        let mut face = Face::default();
        for _ in 0..Face::COUNT {
            let custom = art.frame_with_face(0, a, b, f, 120., 230., face);
            let hooks = |commands: &[playman_rust::assets::artwork::Command]| {
                commands
                    .iter()
                    .filter(|c| matches!(c, playman_rust::assets::artwork::Command::Hook { .. }))
                    .count()
            };
            assert_eq!(hooks(&custom), hooks(&default));
            let differences = default
                .iter()
                .zip(&custom)
                .filter(|(a, b)| format!("{a:?}") != format!("{b:?}"))
                .count();
            if face == Face::default() {
                assert_eq!(differences, 0);
            } else {
                assert!(
                    differences > 0,
                    "missing character substitution for {event:?}"
                );
            }
            face = face.next();
        }
    }
}
#[test]
fn players_keep_independent_faces_in_setup_gameplay_and_replays() {
    let mut app = App::new([None; 6]);
    app.multiplayer = true;
    app.screen = Screen::PlayerSetup;
    app.setup_selection = 1;
    app.update(
        0.,
        FrameInput {
            right: true,
            ..Default::default()
        },
    );
    let first = app.players[0].appearance;
    assert_ne!(first, Appearance::default());
    assert_eq!(app.players[1].appearance, Appearance::default());
    app.setup_selection = 2;
    app.update(
        0.,
        FrameInput {
            left: true,
            ..Default::default()
        },
    );
    let second = app.players[1].appearance;
    app.start();
    assert_eq!(app.race_athletes()[0].appearance, first);
    app.advance();
    let athletes = app.race_athletes();
    assert_eq!(athletes[0].appearance, first);
    assert_eq!(athletes[1].appearance, second);
}
#[test]
fn singleplayer_setup_exposes_exactly_one_customizable_player() {
    let mut app = App::new([None; 6]);
    app.update(
        0.,
        FrameInput {
            enter: true,
            ..Default::default()
        },
    );
    assert!(app.screen == Screen::PlayerSetup);
    assert_eq!(app.setup_player_count(), 1);
    app.update(
        0.,
        FrameInput {
            right: true,
            ..Default::default()
        },
    );
    assert_eq!(app.setup_player_count(), 1);
}

#[test]
fn original_character_metadata_selects_matching_body_and_skin_banks() {
    let art = Artwork::new();
    let playman = Face::default();
    let vera = playman.next().next();
    assert_eq!(playman.scene(), 0);
    assert_eq!(vera.scene(), 1);
    assert_eq!(vera.sprite_for_scene(1, 581), 544); // Female dark-skin arm, not male/tan.
    assert_eq!(vera.sprite_for_scene(1, 790), 802); // Vera's corresponding head view.
    let mut face = playman;
    let table = include_bytes!("../assets/tables.bin");
    for _ in 0..Face::COUNT {
        let bank = if face.scene() == 0 { 346 } else { 514 };
        for slot in 0..21 {
            let offset = bank + slot * 4;
            let sprite = i32::from_be_bytes(table[offset..offset + 4].try_into().unwrap()) as usize;
            assert!(
                art.sprite(
                    face.sprite_for_scene(face.scene(), sprite),
                    0,
                    0.,
                    0.,
                    false
                )
                .is_some()
            );
        }
        for pose in 0..318 {
            assert!(
                !art.frame_with_face(0, pose, pose, 0, 120., 230., face)
                    .is_empty()
            );
        }
        face = face.next();
    }
}
