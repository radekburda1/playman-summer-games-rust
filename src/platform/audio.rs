use crate::simulation::audio::SoundCue;
use macroquad::audio::{
    PlaySoundParams, Sound, load_sound_from_bytes, play_sound, set_sound_volume, stop_sound,
};
use macroquad::experimental::coroutines::{Coroutine, start_coroutine, stop_coroutine};
use std::collections::VecDeque;

// Original sound-table delays: resource 12 waits 373 ms; resource 14 waits 460 ms.
// These are queue delays, not the full WAV lengths; celebration can overlap a jingle.
#[derive(Default)]
struct ResultQueue {
    cues: VecDeque<SoundCue>,
    until: f64,
}
impl ResultQueue {
    fn next(&mut self, now: f64) -> Option<SoundCue> {
        if now < self.until {
            return None;
        }
        let cue = self.cues.pop_front()?;
        self.until = now
            + match cue {
                SoundCue::ResultIntro => 0.373,
                SoundCue::Celebration => 0.460,
                _ => 0.,
            };
        Some(cue)
    }
    fn active(&self, now: f64) -> bool {
        !self.cues.is_empty() || now < self.until
    }
}

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/music/catalog.rs"
));
struct PendingMusic {
    id: usize,
    looped: bool,
    task: Coroutine<Option<Sound>>,
}

pub(super) struct Sounds {
    clips: Vec<Option<Sound>>,
    pub(super) muted: bool,
    music: Option<Sound>,
    music_id: Option<usize>,
    jingle_until: f64,
    requested_jingle: Option<usize>,
    pending_music: Option<PendingMusic>,
    results: ResultQueue,
}
impl Sounds {
    pub(super) async fn load() -> Self {
        let bytes: &[&[u8]] = &[
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/12.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/13.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/14.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/15.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/16.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/17.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/18.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/19.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/20.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/21.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/22.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/23.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/24.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/25.wav")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sounds/26.wav")),
        ];
        let mut clips = Vec::new();
        for bytes in bytes {
            clips.push(load_sound_from_bytes(bytes).await.ok())
        }
        Self {
            clips,
            muted: false,
            music: None,
            music_id: None,
            jingle_until: 0.,
            requested_jingle: Some(11),
            pending_music: None,
            results: ResultQueue::default(),
        }
    }
    pub(super) fn begin_attempt(&mut self) {
        self.stop_music();
        self.requested_jingle = None;
        self.results = ResultQueue::default();
        for id in [SoundCue::ResultIntro, SoundCue::Celebration] {
            if let Some(Some(sound)) = self.clips.get(id.resource_id() - 12) {
                stop_sound(sound);
            }
        }
    }
    fn stop_music(&mut self) {
        if let Some(pending) = self.pending_music.take() {
            stop_coroutine(pending.task);
        }
        if let Some(previous) = self.music.take() {
            stop_sound(&previous);
        }
        self.music_id = None;
        self.jingle_until = 0.;
    }
    fn poll_music(&mut self, now: f64) {
        let loaded = self
            .pending_music
            .as_ref()
            .and_then(|pending| pending.task.retrieve());
        if let Some(loaded) = loaded {
            let PendingMusic { id, looped, .. } = self.pending_music.take().unwrap();
            self.music = loaded;
            if !looped {
                self.jingle_until = now + MUSIC_DURATION[id];
            }
            if let Some(sound) = self.music.as_ref() {
                play_sound(
                    sound,
                    PlaySoundParams {
                        looped,
                        volume: if self.muted { 0. } else { 0.45 },
                    },
                );
            }
        }
    }
    pub(super) fn update_music(&mut self, background: Option<usize>) {
        let now = macroquad::time::get_time();
        self.poll_music(now);
        if let Some(cue) = self.results.next(now) {
            if cue == SoundCue::Record {
                self.requested_jingle = Some(cue.resource_id());
            } else {
                self.play_sample(cue);
            }
        }
        let background = background.filter(|_| !self.results.active(now));
        for sound in self.clips.iter().flatten() {
            set_sound_volume(sound, if self.muted { 0. } else { 0.55 });
        }
        let restart = self.requested_jingle.is_some();
        let (id, looped) = if let Some(id) = self.requested_jingle.take() {
            (id, false)
        } else if self.jingle_until > now {
            if let Some(sound) = self.music.as_ref() {
                set_sound_volume(sound, if self.muted { 0. } else { 0.45 });
            }
            return;
        } else if let Some(background) = background {
            (background, true)
        } else {
            self.stop_music();
            return;
        };
        if restart || self.music_id != Some(id) {
            self.stop_music();
            self.music_id = Some(id);
            if !looped {
                self.jingle_until = f64::INFINITY;
            }
            // Decode outside the main frame future: WASM loading awaits next_frame.
            // A coroutine lets the normal render loop draw every loading frame.
            let task = start_coroutine(async move { load_sound_from_bytes(MUSIC[id]).await.ok() });
            self.pending_music = Some(PendingMusic { id, looped, task });
        } else if let Some(sound) = self.music.as_ref() {
            set_sound_volume(sound, if self.muted { 0. } else { 0.45 });
        }
    }
    pub(super) fn play(&mut self, cue: SoundCue) {
        if matches!(
            cue,
            SoundCue::ResultIntro | SoundCue::Celebration | SoundCue::Record
        ) {
            self.results.cues.push_back(cue);
            return;
        }
        self.play_sample(cue);
    }
    fn play_sample(&self, cue: SoundCue) {
        let id = cue.resource_id();
        if !self.muted
            && let Some(Some(sound)) = id.checked_sub(12).and_then(|i| self.clips.get(i))
        {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: 0.55,
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_queue_matches_original_player_start_ticks() {
        for scenario in ["result-good", "result-poor", "result-good-record"] {
            let expected: Vec<(i32, usize)> =
                include_str!("../../tests/fixtures/audio/result-cues.csv")
                    .lines()
                    .skip(1)
                    .filter_map(|row| {
                        let c: Vec<_> = row.split(',').collect();
                        (c[0] == scenario).then(|| (c[1].parse().unwrap(), c[2].parse().unwrap()))
                    })
                    .collect();
            let mut queue = ResultQueue::default();
            for (_, resource) in &expected {
                queue.cues.push_back(match resource {
                    12 => SoundCue::ResultIntro,
                    14 => SoundCue::Celebration,
                    8 => SoundCue::Record,
                    _ => panic!("Unexpected original result cue"),
                });
            }
            let mut actual = Vec::new();
            for tick in 0..=40 {
                if let Some(cue) = queue.next(tick as f64 * 0.032) {
                    actual.push((tick, cue.resource_id()));
                }
            }
            assert_eq!(actual, expected, "{scenario}");
            assert!(!queue.active(2.));
        }
    }
}
