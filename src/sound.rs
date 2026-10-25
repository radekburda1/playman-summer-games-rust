use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound};
pub struct Sounds {
    clips: Vec<Option<Sound>>,
    pub muted: bool,
}
impl Sounds {
    pub async fn load() -> Self {
        let bytes: &[&[u8]] = &[
            include_bytes!("../assets/sounds/12.wav"),
            include_bytes!("../assets/sounds/13.wav"),
            include_bytes!("../assets/sounds/14.wav"),
            include_bytes!("../assets/sounds/15.wav"),
            include_bytes!("../assets/sounds/16.wav"),
            include_bytes!("../assets/sounds/17.wav"),
            include_bytes!("../assets/sounds/18.wav"),
            include_bytes!("../assets/sounds/19.wav"),
            include_bytes!("../assets/sounds/20.wav"),
            include_bytes!("../assets/sounds/21.wav"),
            include_bytes!("../assets/sounds/22.wav"),
            include_bytes!("../assets/sounds/23.wav"),
            include_bytes!("../assets/sounds/24.wav"),
            include_bytes!("../assets/sounds/25.wav"),
            include_bytes!("../assets/sounds/26.wav"),
        ];
        let mut clips = Vec::new();
        for bytes in bytes {
            clips.push(load_sound_from_bytes(bytes).await.ok())
        }
        Self {
            clips,
            muted: false,
        }
    }
    pub fn play(&self, id: usize) {
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
