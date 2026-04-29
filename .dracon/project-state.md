# Project State

## Current Focus
Filter out `local` and `computed` attributes from struct fields and adjust field processing logic

## Completed
- [x] Replace filtered field struct creation with explicit `filtered_field` variables including `ty`, `vis`, `colon_token`, and `ident` clones
- [x] Retain only non‑`local` and non‑`computed` attributes when filtering field attributes
- [x] Rebuild the punctuated field list using retained attributes instead of a manual pairs collector
- [x] Update `Cargo.lock` to reflect newer dependency versions
