fn main() {
    // Only generate the Tauri context when building the real app; pure-logic
    // test runs (`--no-default-features`) skip it for speed.
    if std::env::var("CARGO_FEATURE_APP").is_ok() {
        tauri_build::build();
    }
}
