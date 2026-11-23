//! Logical menu geometry shared by input handling and rendering.
pub const HANDICAP_PANEL: [f32; 4] = [12., 231., 216., 56.];
pub const HANDICAP_TRACK: [f32; 2] = [26., 214.];

pub fn handicap_at(x: f32) -> u8 {
    let [left, right] = HANDICAP_TRACK;
    let stop = (((x - left) / (right - left)).clamp(0., 1.) * 5.).round() as u8;
    50 + stop * 10
}
