//! Record comparison and the existing text-file persistence format.
use crate::simulation::Event;
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
