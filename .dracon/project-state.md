# Project State

## Current Focus
Refactor LiveState expansion to replace struct snapshot with inline JSON map for state signing

## Completed
- [x] Replace struct `__LocalOnly` creation with direct insertion into a `serde_json::Map`
- [x] Serialize each field value into the map using `serde_json::to_value` and `stringify!`
- [x] Use the resulting map for JSON serialization in `to_local_scope`
