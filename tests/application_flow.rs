use playman_rust::{
    application::{App, FrameInput, Screen},
    simulation::Event,
    simulation::Game,
    simulation::audio::SoundCue,
};
fn confirm() -> FrameInput {
    FrameInput {
        action: true,
        ..Default::default()
    }
}
#[test]
fn confirmation_slides_without_advancing_simulation_then_race_counts_down() {
    for event in Event::ALL {
        let mut app = App::new([None; 5]);
        app.selected = Event::ALL.iter().position(|e| *e == event).unwrap();
        app.start();
        for _ in 0..20 {
            app.update(0.1, FrameInput::default());
        }
        assert!(app.screen == Screen::AwaitStart);
        assert_eq!(app.game.elapsed_ms, 0);
        app.update(0.1, confirm());
        assert!(app.outgoing.is_some());
        for _ in 0..4 {
            app.update(0.1, FrameInput::default());
        }
        assert_eq!(app.game.elapsed_ms, 0);
        if event.race() {
            assert!(app.screen == Screen::Ready);
            let mut cues = Vec::new();
            for _ in 0..51 {
                cues.extend(app.update(0.1, FrameInput::default()).sounds);
            }
            assert!(app.screen == Screen::Play);
            assert!(cues.contains(&SoundCue::Set));
            assert!(cues.contains(&SoundCue::Gun));
        } else {
            assert!(app.screen == Screen::Play);
        }
    }
}
#[test]
fn records_are_independent_of_performance_and_equal_results_do_not_overwrite() {
    let mut app = App::new([Some(1200), None, None, None, None]);
    app.game = Game::new(Event::Dash, 1);
    app.game.result = Some(1150);
    app.finish();
    assert!(app.game.finish_record);
    assert!(!app.game.finish_good);
    assert!(app.records_dirty);
    app.records_dirty = false;
    app.finish();
    assert!(!app.game.finish_record);
    assert!(!app.records_dirty);
    app.game.result = Some(965);
    app.records[0] = Some(900);
    app.finish();
    assert!(!app.game.finish_record);
    assert!(app.game.finish_good);
}
#[test]
fn paused_frames_never_accumulate_catchup_ticks() {
    let mut app = App::new([None; 5]);
    app.screen = Screen::Play;
    app.game.runner.speed = 20000;
    app.update(
        0.1,
        FrameInput {
            pause: true,
            ..Default::default()
        },
    );
    for _ in 0..100 {
        app.update(0.1, FrameInput::default());
    }
    assert_eq!(app.game.elapsed_ms, 0);
    app.update(
        0.1,
        FrameInput {
            pause: true,
            ..Default::default()
        },
    );
    assert_eq!(app.game.elapsed_ms, 0);
    app.update(0.032, FrameInput::default());
    assert_eq!(app.game.elapsed_ms, 32);
}
