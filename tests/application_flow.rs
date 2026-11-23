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
        let mut app = App::new([None; 6]);
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
    let mut app = App::new([Some(1200), None, None, None, None, None]);
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
    let mut app = App::new([None; 6]);
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
    let records = [
        Some(1000),
        Some(600),
        None,
        Some(400),
        Some(7000),
        Some(4000),
    ];
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
    assert_eq!(app.records, [None; 6]);
    assert!(app.records_dirty);
}

#[test]
fn background_music_stops_before_attempt_and_resumes_after_finish_confirmation() {
    for selected in 0..Event::COUNT {
        let mut app = App::new([Some(0); 6]);
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

#[test]
fn speed_menu_bounds_and_half_speed_preserve_simulation_ticks() {
    let mut slow = App::new([None; Event::COUNT]);
    slow.screen = Screen::Menu;
    for expected in [90, 80, 70, 60, 50, 50] {
        slow.update(
            0.,
            FrameInput {
                down: true,
                ..Default::default()
            },
        );
        assert_eq!(slow.speed_percent(), expected);
        assert_eq!(slow.selected, 0);
    }
    for (selected, _) in Event::SELECTABLE.into_iter().enumerate() {
        let mut normal = App::new([None; Event::COUNT]);
        normal.seed = slow.seed;
        normal.selected = selected;
        slow.selected = selected;
        normal.start();
        slow.start();
        normal.screen = Screen::Play;
        slow.screen = Screen::Play;
        for _ in 0..50 {
            let input = FrameInput {
                left: true,
                ..Default::default()
            };
            normal.update(0.032, input);
            slow.update(0.032, input);
            slow.update(0.032, FrameInput::default());
        }
        assert_eq!(slow.game.elapsed_ms, normal.game.elapsed_ms);
        assert_eq!(slow.game.runner.position, normal.game.runner.position);
        assert_eq!(slow.game.runner.speed, normal.game.runner.speed);
        assert_eq!(
            slow.game.runner.expected_key,
            normal.game.runner.expected_key
        );
    }
    slow.screen = Screen::Ready;
    slow.ready = 0.;
    slow.update(0.1, FrameInput::default());
    assert_eq!(slow.ready, 0.05);
    slow.screen = Screen::Menu;
    for expected in [60, 70, 80, 90, 100, 100] {
        slow.update(
            0.,
            FrameInput {
                up: true,
                ..Default::default()
            },
        );
        assert_eq!(slow.speed_percent(), expected);
    }
}

#[test]
fn handicap_pointer_snaps_clamps_and_requires_a_drag_started_on_the_slider() {
    use playman_rust::application::{PointerInput, menu};

    let mut app = App::new([None; Event::COUNT]);
    app.screen = Screen::Menu;
    let pointer = |x, y, pressed, down| FrameInput {
        pointer: Some(PointerInput {
            x,
            y,
            pressed,
            down,
        }),
        ..Default::default()
    };
    app.update(0., pointer(0., 265., true, true));
    app.update(0., pointer(26., 265., false, true));
    assert_eq!(app.speed_percent(), 100);

    let [left, right] = menu::HANDICAP_TRACK;
    for stop in 0..=5 {
        let x = left + stop as f32 / 5. * (right - left);
        app.update(0., pointer(x, 265., true, true));
        assert_eq!(app.speed_percent(), 50 + stop * 10);
        assert_eq!(app.selected, 0);
    }
    app.update(0., pointer(120., 265., true, true));
    assert_eq!(app.speed_percent(), 80);
    app.update(0., pointer(-100., 500., false, true));
    assert_eq!(app.speed_percent(), 50);
    app.update(0., pointer(1000., 0., false, true));
    assert_eq!(app.speed_percent(), 100);
    app.update(0., pointer(left, 265., false, false));
    app.update(0., pointer(left, 265., false, true));
    assert_eq!(app.speed_percent(), 100);

    app.update(0., pointer(120., 265., true, true));
    app.screen = Screen::Help;
    app.update(0., pointer(left, 265., false, true));
    app.screen = Screen::Menu;
    app.update(0., pointer(left, 265., false, true));
    assert_eq!(app.speed_percent(), 80);

    app.update(0., pointer(left, 265., true, false));
    assert_eq!(app.speed_percent(), 50);
    app.update(0., pointer(right, 265., false, true));
    assert_eq!(app.speed_percent(), 50);
}

#[test]
fn space_advances_every_setup_row_without_appending_to_names() {
    for multiplayer in [false, true] {
        let mut app = App::new([None; Event::COUNT]);
        app.mode_selection = usize::from(multiplayer);
        app.update(0., confirm());
        assert!(app.screen == Screen::PlayerSetup);
        let player_count = app.setup_player_count();
        for row in 0..=player_count + 1 {
            assert_eq!(app.setup_selection, row);
            app.update(
                0.,
                FrameInput {
                    action: true,
                    text: Some(' '),
                    ..Default::default()
                },
            );
            app.update(
                0.,
                FrameInput {
                    text: Some(' '),
                    ..Default::default()
                },
            );
        }
        assert!(app.screen == Screen::Menu);
        for (index, player) in app.players.iter().enumerate() {
            assert_eq!(player.name, format!("PLAYER {}", index + 1));
        }
        app.update(0., confirm());
        assert!(app.screen == Screen::Help);
        app.update(0., confirm());
        assert!(app.screen == Screen::AwaitStart);
    }
}
