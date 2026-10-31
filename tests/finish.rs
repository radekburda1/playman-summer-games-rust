use playman_rust::{Event, Game, presentation::Animation};
#[test]
fn runout_moves_then_stops_without_changing_result_or_clock() {
    let mut game = Game::new(Event::Dash, 1);
    game.finished = true;
    game.result = Some(1021);
    game.elapsed = 10464;
    game.runner.pos = 6553600;
    game.runner.speed = 20000;
    let start = game.runner.pos;
    for _ in 0..100 {
        game.step_finish();
    }
    assert!(game.runner.pos > start);
    assert!(game.runner.speed < 4000);
    assert_eq!(game.result, Some(1021));
    assert_eq!(game.elapsed, 10464);
    assert!(game.phase_time > 0);
    let stopped = game.runner.pos;
    game.step_finish();
    assert_eq!(game.runner.pos, stopped);
}
#[test]
fn record_and_nonrecord_use_original_reaction_sequences() {
    let mut game = Game::new(Event::Dash, 1);
    game.finished = true;
    game.runner.speed = -1;
    game.finish_good = true;
    assert_eq!(Animation::for_game(&game).offset, 3440);
    game.phase_time = 200;
    assert_eq!(Animation::for_game(&game).offset, 3446);
    game.finish_good = false;
    game.phase_time = 0;
    assert_eq!(Animation::for_game(&game).offset, 3402);
    game.phase_time = 1600;
    assert_eq!(Animation::for_game(&game).offset, 3430);
}

#[test]
fn feedback_uses_original_score_bands_not_personal_record() {
    assert_eq!(Event::Dash.score(Some(1071)), 734);
    assert_eq!(Event::Dash.finish_message(Some(1071)), "NICE!");
    assert_eq!(Event::Dash.finish_message(Some(965)), "AWESOME!");
    assert_eq!(Event::Dash.finish_message(Some(1282)), "NOT GOOD ENOUGH");
}
