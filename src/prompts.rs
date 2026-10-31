//! Feedback timings and prompt positions recovered from d.a(c,int,int).
use crate::{Event, Game, Phase, presentation::Camera};
#[derive(Clone, Copy, Debug)]
pub struct RunFeedback {
    pub key: usize,
    pub correct: bool,
    pub at_ms: i32,
    pub slot: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct Prompt {
    pub sprite: usize,
    pub x: i32,
    pub y: i32,
}
pub fn running(g: &Game) -> Vec<Prompt> {
    if g.finished || g.phase != Phase::Run {
        return Vec::new();
    }
    let cutoff = if g.event == Event::Dash {
        Some(0x500000)
    } else if g.event == Event::Hurdles {
        Some(0x640000)
    } else {
        None
    };
    if cutoff.is_some_and(|x| g.runner.pos >= x) {
        return Vec::new();
    }
    let camera = if g.event.race() {
        Camera::race_at(g.runner.pos >> 8)
    } else {
        Camera::for_game(g)
    };
    let (x, y) = camera.project(
        g.runner.pos >> 8,
        g.runner.height >> 8,
        Camera::athlete_lane(g.event),
    );
    let y = y - 38;
    let position = |key: usize, slot: usize| {
        if g.event == Event::Hurdles {
            x + 47 + slot as i32 * 34
        } else if key == 0 {
            x - 65
        } else {
            x + 47
        }
    };
    let mut result = Vec::new();
    let feedback = g.run_feedback.map(|f| (f, g.elapsed - f.at_ms));
    let bounce = feedback
        .filter(|(f, age)| f.correct && (0..100).contains(age))
        .map_or(0, |(_, age)| 70 * (100 - age) / 100);
    if g.event == Event::Hurdles {
        for slot in g.runner.sequence_index..3 {
            let key = g.runner.sequence[slot];
            result.push(Prompt {
                sprite: 470 + key,
                x: position(key, slot),
                y,
            });
        }
    } else {
        result.push(Prompt {
            sprite: 470 + g.runner.expected,
            x: position(g.runner.expected, 0),
            y: y + bounce,
        });
    }
    if let Some((f, age)) = feedback {
        if f.correct && (0..200).contains(&age) {
            result.push(Prompt {
                sprite: 474 + (age * 3 / 200) as usize,
                x: position(f.key, f.slot),
                y,
            });
        } else if !f.correct && (0..300).contains(&age) {
            result.push(Prompt {
                sprite: 477,
                x: position(f.key, f.slot),
                y,
            });
        }
    }
    result
}
