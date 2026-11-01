//! Existing executable-adjacent storage; browser error behavior is preserved.
use crate::application::records::{decode, encode};
use std::path::PathBuf;
fn record_path() -> PathBuf {
    // Store beside the executable; failures are surfaced in the result screen.
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("playman-records.txt")))
        .unwrap_or_else(|| PathBuf::from("playman-records.txt"))
}
pub(super) fn load_records() -> [Option<i32>; 5] {
    std::fs::read_to_string(record_path()).map_or([None; 5], |text| decode(&text))
}
pub(super) fn save_records(r: &[Option<i32>; 5]) -> std::io::Result<()> {
    let path = record_path();
    let temp = path.with_extension("tmp");
    std::fs::write(&temp, encode(r))?;
    std::fs::rename(temp, path)
}
