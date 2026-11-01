//! Record comparison and the existing text-file persistence format.
use crate::Event;
use std::path::PathBuf;
pub fn better(event: Event, new: Option<i32>, old: Option<i32>) -> bool {
    match (new, old) {
        (Some(a), Some(b)) => {
            if event.race() {
                a < b
            } else {
                a > b
            }
        }
        (Some(_), None) => true,
        _ => false,
    }
}
fn record_path() -> PathBuf {
    // Store beside the executable; failures are surfaced in the result screen.
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("playman-records.txt")))
        .unwrap_or_else(|| PathBuf::from("playman-records.txt"))
}
pub fn load_records() -> [Option<i32>; 5] {
    std::fs::read_to_string(record_path()).map_or([None; 5], |text| decode(&text))
}
/// Missing, negative and malformed entries retain the original no-record semantics.
pub fn decode(text: &str) -> [Option<i32>; 5] {
    let mut records = [None; 5];
    for (slot, line) in records.iter_mut().zip(text.lines()) {
        *slot = line.parse::<i32>().ok().filter(|n| *n >= 0);
    }
    records
}
pub fn encode(records: &[Option<i32>; 5]) -> String {
    records
        .iter()
        .map(|r| r.map_or("-".into(), |v| v.to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}
pub fn save_records(r: &[Option<i32>; 5]) -> std::io::Result<()> {
    let path = record_path();
    let temp = path.with_extension("tmp");
    std::fs::write(&temp, encode(r))?;
    std::fs::rename(temp, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn record_format_preserves_slots_and_tolerates_invalid_entries() {
        let records = [Some(965), None, Some(1400), Some(500), Some(8271)];
        assert_eq!(encode(&records), "965\n-\n1400\n500\n8271");
        assert_eq!(decode(&encode(&records)), records);
        assert_eq!(
            decode("-1\ninvalid\n0\n42\n-\n999"),
            [None, None, Some(0), Some(42), None]
        );
    }
}
