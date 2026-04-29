# Project State

## Current Focus
Replace inline JSON construction with an explicit struct serialization for local scope signing.

## Completed
- [x] Replace `serde_json::json!({ #(#field_values),* }).to_string()` with a generated `__LocalOnly` struct and `serde_json::to_string(&__LocalOnly { #(#field_values),* })`
- [x] Use `unwrap_or_default()` on the serialized JSON to safely handle empty cases
- [x] Remove reliance on the `json!` macro and eliminate the intermediate `.to_string()` step
- [x] Introduce a dedicated struct (`__LocalOnly`) to encapsulate field names and values for clearer serialization logic
