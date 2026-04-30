# Project State

## Current Focus
Rename internal variables in `macros/src/live.rs` to use an underscore prefix to signal they are temporary/local and follow a private naming convention, and regenerate `Cargo.lock`.

## Completed
- [x] Prefix `struct_name_str` to `_struct_name_str`
- [x] Prefix `local_field_names_static` to `_local_field_names_static`
- [x] Prefix `computed_field_names_static` to `_computed_field_names_static`
- [x] Update Cargo.lock dependency manifest
