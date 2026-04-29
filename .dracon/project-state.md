# Project State

## Current Focus
Filter out azumi-specific attributes (`local`, `computed`) from live structs in the LiveState macro, ensuring generated structs only contain relevant attributes.

## Completed
- [x] Implement filtering of `local` and `computed` attributes from struct fields before code generation in macros/src/live.rs
- [x] Preserve remaining field attributes while constructing new `syn::Field` instances for the filtered struct
- [x] Replace original `struct_fields` with `filtered_named_fields` when generating the expanded struct definition
- [x] Update Cargo.lock to lock new dependency versions (binary unchanged)
