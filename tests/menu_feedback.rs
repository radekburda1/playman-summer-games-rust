use playman_rust::{Event, Game, Input, bitmap_font::BitmapFont, prompts};
#[test]
fn bitmap_fonts_have_original_heights_and_signed_offsets() {
    assert_eq!(
        [3, 4, 5, 6].map(|id| BitmapFont::get(id).height),
        [22, 29, 18, 28]
    );
    let font = BitmapFont::get(3);
    assert!(font.glyph(' ').sprite < 0);
    assert!(font.glyph('A').sprite >= 0);
    assert!(font.width("SELECT EVENT") < 190);
    assert!(font.glyph('j').x < 0);
}
#[test]
fn correct_number_pulses_for_200ms_and_advances_prompt() {
    let mut g = Game::new(Event::Dash, 1);
    assert_eq!(g.runner.expected, 0);
    g.input(Input::Left);
    let feedback = g.run_feedback.unwrap();
    assert!(feedback.correct);
    assert_eq!(feedback.key, 0);
    assert_eq!(g.runner.expected, 1);
    for (age, sprite) in [(0, 474), (67, 475), (134, 476)] {
        g.elapsed = age;
        assert!(prompts::running(&g).iter().any(|p| p.sprite == sprite));
    }
    g.elapsed = 200;
    assert!(
        prompts::running(&g)
            .iter()
            .all(|p| !(474..=476).contains(&p.sprite))
    );
}
#[test]
fn incorrect_number_shows_x_on_pressed_side_for_300ms() {
    let mut g = Game::new(Event::Dash, 1);
    g.input(Input::Right);
    assert!(!g.run_feedback.unwrap().correct);
    assert_eq!(g.runner.expected, 0);
    let p = prompts::running(&g);
    let normal = p.iter().find(|p| p.sprite == 470).unwrap();
    let error = p.iter().find(|p| p.sprite == 477).unwrap();
    assert_eq!(error.x - normal.x, 112);
    g.elapsed = 299;
    assert!(prompts::running(&g).iter().any(|p| p.sprite == 477));
    g.elapsed = 300;
    assert!(prompts::running(&g).iter().all(|p| p.sprite != 477));
}
#[test]
fn hurdle_feedback_stays_on_consumed_sequence_slot() {
    let mut g = Game::new(Event::Hurdles, 1);
    g.input(Input::Left);
    assert_eq!(g.runner.sequence_index, 1);
    let p = prompts::running(&g);
    assert_eq!(p.len(), 3);
    let flash = p.iter().find(|p| p.sprite == 474).unwrap();
    assert_eq!(p[0].x - flash.x, 34);
}
#[test]
fn ignored_inputs_do_not_invent_highlights() {
    let mut g = Game::new(Event::Dash, 1);
    g.runner.pos = 0x500000;
    g.input(Input::Left);
    assert!(g.run_feedback.is_none());
    assert!(prompts::running(&g).is_empty());
}

#[test]
fn sprint_prompt_anchor_does_not_follow_early_camera_offset() {
    let mut game = Game::new(Event::Dash, 1);
    let initial = prompts::running(&game)[0].x;
    for position in [0, 128, 512, 1024, 2048, 2560, 4096, 5120] {
        game.runner.pos = position << 8;
        assert_eq!(prompts::running(&game)[0].x, initial, "position {position}");
    }
}
