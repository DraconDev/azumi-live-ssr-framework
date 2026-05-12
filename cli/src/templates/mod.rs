/// Template for Cargo.toml - uses {{project_name}} and {{azumi_version}} placeholders
pub const CARGO_TOML: &str = include_str!("Cargo.toml.hbs");

/// Template for src/main.rs - the main application entry point
pub const MAIN_RS: &str = include_str!("main.rs.hbs");
