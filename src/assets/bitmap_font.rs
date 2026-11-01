//! The original glyph images and signed per-character offsets (fonts 3..6).
#[derive(Debug)]
pub struct Glyph {
    pub code: u16,
    pub sprite: i16,
    pub advance: i16,
    pub x: i16,
    pub y: i16,
}
pub struct BitmapFont {
    pub height: i16,
    pub glyphs: &'static [Glyph],
}
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts.rs"));
impl BitmapFont {
    pub fn get(id: usize) -> &'static Self {
        &FONTS[id.saturating_sub(3).min(3)]
    }
    pub fn glyph(&self, c: char) -> &Glyph {
        self.glyphs
            .iter()
            .find(|g| g.code as u32 == c as u32)
            .unwrap_or(&self.glyphs[0])
    }
    pub fn width(&self, text: &str) -> i32 {
        text.chars().map(|c| self.glyph(c).advance as i32).sum()
    }
}
