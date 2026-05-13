use std::fs;

fn main() {
    // Read parent Cargo.toml to extract the azumi crate version
    let cargo_toml = fs::read_to_string("../Cargo.toml")
        .expect("Failed to read parent Cargo.toml — build.rs must run from cli/ directory");

    let version_line = cargo_toml
        .lines()
        .find(|l| l.starts_with("version = "))
        .expect("version line not found in parent Cargo.toml");

    let version = version_line
        .split('"')
        .nth(1)
        .expect("version value not found");

    let out = format!(r#"pub const AZUMI_VERSION: &str = "v{}";"#, version);

    fs::write("src/version.rs", out).expect("Failed to write version.rs");
}
