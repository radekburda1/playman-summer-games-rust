use playman_rust::{
    application::{App, FrameInput, Screen},
    simulation::{Event, Game},
};
fn confirm(app: &mut App) {
    app.update(
        0.,
        FrameInput {
            enter: true,
            ..Default::default()
        },
    );
}
fn multiplayer(count: usize, selected: usize, tournament: bool) -> App {
    let mut app = App::new([None; 5]);
    app.multiplayer = true;
    app.player_count = count;
    app.selected = selected;
    app.tournament = tournament;
    app.start();
    app
}
#[test]
fn mode_selection_and_setup_boundaries() {
    let mut app = App::new([None; 5]);
    assert!(app.screen == Screen::ModeSelect);
    app.update(
        0.,
        FrameInput {
            right: true,
            ..Default::default()
        },
    );
    confirm(&mut app);
    assert!(app.screen == Screen::PlayerSetup);
    app.update(
        0.,
        FrameInput {
            left: true,
            ..Default::default()
        },
    );
    assert_eq!(app.player_count, 6);
    app.update(
        0.,
        FrameInput {
            right: true,
            ..Default::default()
        },
    );
    assert_eq!(app.player_count, 2);
    app.setup_selection = 1;
    app.players[0].name.clear();
    app.update(
        0.,
        FrameInput {
            text: Some('a'),
            ..Default::default()
        },
    );
    assert_eq!(app.players[0].name, "A");
    app.setup_selection = 3;
    confirm(&mut app);
    assert!(app.screen == Screen::Menu);
    confirm(&mut app);
    assert!(app.screen == Screen::Help);
}
#[test]
fn every_event_rotates_players_then_attempts_and_preserves_bests() {
    for count in [2, 6] {
        for (selected, event) in Event::ALL.into_iter().enumerate() {
            let mut app = multiplayer(count, selected, false);
            let rounds = if event.race() { 1 } else { 3 };
            for round in 0..rounds {
                for player in 0..count {
                    assert_eq!(app.current_player, player);
                    assert_eq!(app.attempt, round);
                    let value = if round == 1 {
                        None
                    } else {
                        Some(1000 + player as i32)
                    };
                    app.game.result = value;
                    app.finish();
                    app.advance();
                }
            }
            assert!(app.screen == Screen::Standings);
            for player in 0..count {
                assert_eq!(
                    app.players[player].results[selected],
                    Some(1000 + player as i32)
                );
            }
            assert_eq!(app.standings()[0], if event.race() { 0 } else { count - 1 });
            confirm(&mut app);
            assert!(app.screen == Screen::Menu);
        }
    }
}
#[test]
fn tournament_totals_are_awarded_once_per_event() {
    let mut app = multiplayer(6, 0, true);
    for (selected, event) in Event::ALL.into_iter().enumerate() {
        assert_eq!(app.selected, selected);
        app.game = Game::new(event, 1);
        for _ in 0..if event.race() { 1 } else { 3 } {
            for _ in 0..6 {
                app.game.result = Some(1000);
                app.finish();
                app.advance();
            }
        }
        assert!(app.screen == Screen::Standings);
        let expected: i32 = Event::ALL[..=selected]
            .iter()
            .map(|e| e.score(Some(1000)))
            .sum();
        assert!(app.players.iter().all(|p| p.total == expected));
        assert_eq!(app.standings(), vec![0, 1, 2, 3, 4, 5]);
        confirm(&mut app);
        if selected < 4 {
            assert!(app.screen == Screen::Help);
            app.start();
        } else {
            assert!(app.screen == Screen::Menu);
        }
    }
}
#[test]
fn retry_does_not_advance_turn_and_fouls_rank_last() {
    let mut app = multiplayer(2, 0, false);
    app.game.result = Some(1100);
    app.finish();
    app.screen = Screen::Results;
    app.update(
        0.,
        FrameInput {
            retry: true,
            ..Default::default()
        },
    );
    assert_eq!(app.current_player, 0);
    assert_eq!(app.players[0].total, 0);
    app.game.result = Some(1000);
    app.finish();
    app.advance();
    app.game.result = None;
    app.finish();
    app.advance();
    assert_eq!(app.standings(), vec![0, 1]);
}
