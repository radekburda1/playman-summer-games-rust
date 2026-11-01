//! Architectural regression: the deterministic core must not reach outer layers.
use std::{fs, path::Path};
fn inspect(path: &Path) {
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            inspect(&path);
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let source = fs::read_to_string(&path).unwrap();
        // Check identifiers, including grouped imports, ignoring reference comments.
        let code = source
            .lines()
            .map(|line| line.split("//").next().unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        for word in code.split(|c: char| !c.is_alphanumeric() && c != '_') {
            assert!(
                ![
                    "application",
                    "presentation",
                    "rendering",
                    "platform",
                    "macroquad"
                ]
                .contains(&word),
                "{} imports outer layer {word}",
                path.display()
            );
        }
        for forbidden in ["std::fs", "std::env", "include_bytes!"] {
            assert!(
                !code.contains(forbidden),
                "{} accesses {forbidden}",
                path.display()
            );
        }
    }
}
#[test]
fn simulation_has_no_outer_layer_or_device_dependencies() {
    inspect(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src/simulation"));
}
