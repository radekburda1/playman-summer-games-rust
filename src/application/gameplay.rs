//! Fixed-step choreography between independent mechanics and visual effects.
use crate::{
    presentation::{effects::TimingEffect, javelin, vault_marker},
    simulation::{Event, Game, Input, Phase},
};
pub fn input(game: &mut Game, effects: &mut TimingEffect, input: Input) {
    if !game.finished
        && game.event() == Event::Javelin
        && game.phase() == Phase::Flight
        && matches!(input, Input::ActionDown)
    {
        while !game.finished {
            step(game, effects);
        }
    } else {
        game.input(input);
    }
}
pub fn step(game: &mut Game, effects: &mut TimingEffect) {
    if game.finished {
        return;
    }
    let previous = game.phase();
    game.step();
    // The original watchdog exits before updating effects.
    if game.elapsed_ms > 60000 {
        return;
    }
    update_effects(game, effects, previous);
}
pub fn step_finish(game: &mut Game, effects: &mut TimingEffect) {
    if !game.finished {
        return;
    }
    let previous = game.phase();
    game.step_finish();
    update_effects(game, effects, previous);
}
fn update_effects(game: &Game, effects: &mut TimingEffect, previous: Phase) {
    match game.event() {
        Event::Javelin => javelin::update_effects(game, effects, previous),
        Event::PoleVault => {
            let state = game.discipline().vault();
            effects.progress = state.timing_progress;
            effects.emit = state.emit_timing;
            effects.step(vault_marker::timing_marker(game));
        }
        _ => {}
    }
}
