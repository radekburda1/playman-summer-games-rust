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

#[test]
fn reset_records_requires_confirmation_and_cancel_preserves_records() {
    let records = [Some(1000), Some(600), None, Some(400), Some(7000)];
    let mut app = App::new(records);
    let reset = FrameInput {
        retry: true,
        ..Default::default()
    };
    app.screen = Screen::Menu;
    app.update(0.016, reset);
    assert!(app.screen == Screen::ResetRecords);
    assert_eq!(app.records, records);
    app.update(
        0.016,
        FrameInput {
            escape: true,
            ..Default::default()
        },
    );
    assert!(app.screen == Screen::Menu);
    assert_eq!(app.records, records);
    assert!(!app.records_dirty);
    app.update(0.016, reset);
    app.update(0.016, confirm());
    assert!(app.screen == Screen::Menu);
    assert_eq!(app.records, [None; 5]);
    assert!(app.records_dirty);
}

#[test]
fn background_music_stops_before_attempt_and_resumes_after_finish_confirmation() {
    for selected in 0..5 {
        let mut app = App::new([Some(0); 5]);
        app.selected = selected;
        app.screen = Screen::Menu;
        assert!(app.background_music().is_some());
        app.update(0., confirm());
        assert!(app.screen == Screen::Help);
        assert!(app.background_music().is_some());
        app.update(0., confirm());
        assert!(app.screen == Screen::AwaitStart);
        assert!(app.background_music().is_none());
        for screen in [Screen::Ready, Screen::Play, Screen::Finish] {
            app.screen = screen;
            assert!(app.background_music().is_none());
        }
        app.screen = Screen::Play;
        app.update(
            0.,
            FrameInput {
                pause: true,
                ..Default::default()
            },
        );
        assert!(app.paused);
        assert!(app.background_music().is_none());
        app.paused = false;
        app.game.result = Some(1200);
        app.game.runner.speed = -1;
        app.attempt = 2; // Last field attempt returns to the menu after confirmation.
        app.finish();
        assert!(app.background_music().is_none());
        app.update(0., confirm());
        assert!(app.background_music().is_none()); // Finish slide has not completed.
        app.update(0.4, FrameInput::default());
        assert!(app.background_music().is_some());
    }
}
