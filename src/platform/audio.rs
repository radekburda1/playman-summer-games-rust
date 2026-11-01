use macroquad::audio::{
    PlaySoundParams, Sound, load_sound_from_bytes, play_sound, set_sound_volume, stop_sound,
};
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/music/catalog.rs"
));
pub(super) struct Sounds {
    clips: Vec<Option<Sound>>,
    pub(super) muted: bool,
    music: Option<Sound>,
    music_id: Option<usize>,
    jingle_until: f64,
    requested_jingle: Option<usize>,
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
        }
    }
    pub(super) async fn update_music(&mut self, background: usize) {
        for sound in self.clips.iter().flatten() {
            set_sound_volume(sound, if self.muted { 0. } else { 0.55 });
        }
        let restart = self.requested_jingle.is_some();
        let now = macroquad::time::get_time();
        let (id, looped) = if let Some(id) = self.requested_jingle.take() {
            self.jingle_until = now + MUSIC_DURATION[id];
            (id, false)
        } else if self.jingle_until > now {
            if let Some(sound) = self.music.as_ref() {
                set_sound_volume(sound, if self.muted { 0. } else { 0.45 });
            }
            return;
        } else {
            (background, true)
        };
        if restart || self.music_id != Some(id) {
            if let Some(previous) = self.music.take() {
                stop_sound(&previous);
            }
            self.music_id = Some(id);
            self.music = load_sound_from_bytes(MUSIC[id]).await.ok();
            if let Some(sound) = self.music.as_ref() {
                play_sound(
                    sound,
                    PlaySoundParams {
                        looped,
                        volume: if self.muted { 0. } else { 0.45 },
                    },
                );
            }
        } else if let Some(sound) = self.music.as_ref() {
            set_sound_volume(sound, if self.muted { 0. } else { 0.45 });
        }
    }
    pub(super) fn play(&mut self, cue: crate::simulation::audio::SoundCue) {
        let id = cue.resource_id();
        if id < 12 {
            self.requested_jingle = Some(id);
            return;
        }
        if !self.muted
            && let Some(Some(sound)) = id.checked_sub(12).and_then(|i| self.clips.get(i))
        {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: 0.55,
                },
            )
        }
    }
}
