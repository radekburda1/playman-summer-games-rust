//! Native file storage and browser localStorage adapters.
#[cfg(not(target_arch = "wasm32"))]
use crate::application::records::decode;
use crate::application::records::encode;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
fn record_path() -> PathBuf {
    // Store beside the executable; failures are surfaced in the result screen.
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("playman-records.txt")))
        .unwrap_or_else(|| PathBuf::from("playman-records.txt"))
}
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn load_records() -> [Option<i32>; 5] {
    std::fs::read_to_string(record_path()).map_or([None; 5], |text| decode(&text))
}
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn save_records(r: &[Option<i32>; 5]) -> std::io::Result<()> {
    let path = record_path();
    let temp = path.with_extension("tmp");
    std::fs::write(&temp, encode(r))?;
    std::fs::rename(temp, path)
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn playman_record_load(slot: i32) -> i32;
    fn playman_record_save(ptr: *const u8, len: usize) -> i32;
}
#[cfg(target_arch = "wasm32")]
pub(super) fn load_records() -> [Option<i32>; 5] {
    std::array::from_fn(|slot| {
        let value = unsafe { playman_record_load(slot as i32) };
        (value >= 0).then_some(value)
    })
}
#[cfg(target_arch = "wasm32")]
pub(super) fn save_records(records: &[Option<i32>; 5]) -> std::io::Result<()> {
    let text = encode(records);
    if unsafe { playman_record_save(text.as_ptr(), text.len()) } == 1 {
        Ok(())
    } else {
        Err(std::io::Error::other("Browser record storage unavailable"))
    }
}
