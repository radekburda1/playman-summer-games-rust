use playman_rust::{
    Event, Game,
    presentation::{Animation, Camera, SINE},
};
#[test]
fn projection_matches_executed_original_java() {
    for line in include_str!("fixtures/projection.csv").lines().skip(1) {
        let v: Vec<i32> = line.split(',').map(|v| v.parse().unwrap()).collect();
        assert_eq!(
            Camera::new(v[0], v[1], v[2], v[3]).project(v[4], v[5], v[6]),
            (v[7], v[8]),
            "{line}"
        );
    }
}
#[test]
fn sine_matches_all_512_original_values() {
    let original: Vec<i32> = include_str!("fixtures/sine.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(SINE.as_slice(), original);
}
#[test]
fn race_animation_depends_on_distance_not_elapsed_time() {
    let mut g = Game::new(Event::Dash, 1);
    g.runner.speed = 20000;
    g.runner.pos = 4000 << 8;
    let a = Animation::for_game(&g);
    g.elapsed += 1000;
    g.phase_time += 1000;
    assert_eq!(a, Animation::for_game(&g));
    g.runner.pos += 160 << 8;
    assert_eq!(Animation::for_game(&g).time, a.time + 160);
}
#[test]
fn field_animation_wraps_negative_runup() {
    let mut g = Game::new(Event::LongJump, 1);
    g.phase = playman_rust::Phase::Run;
    let a = Animation::for_game(&g);
    let (i, j, f) = a.frames(include_bytes!("../assets/tables.bin"));
    assert_ne!(i, j);
    assert!((0..65536).contains(&f));
}
#[test]
fn pole_run_repeats_only_after_intro() {
    let mut g = Game::new(Event::PoleVault, 1);
    g.phase = playman_rust::Phase::Run;
    g.runner.speed = 20000;
    let intro = Animation::for_game(&g);
    assert_eq!(intro.time, 0);
    assert_eq!(intro.loop_start, 4);
    g.runner.pos += 2240 << 8;
    let (frame, _, _) = Animation::for_game(&g).frames(include_bytes!("../assets/tables.bin"));
    let table = include_bytes!("../assets/tables.bin");
    let offset = 3080 + 4 * 2;
    assert_eq!(
        frame,
        u16::from_be_bytes([table[offset], table[offset + 1]]) as usize
    );
}
