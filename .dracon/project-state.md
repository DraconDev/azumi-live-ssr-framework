# Project State

## Current Focus
Add compile‑time validation for prediction methods that rejects references to local or computed fields by using struct‑level constant field lists.

## Completed
- [x] Replaced deprecated `local_const_entries` and `computed_const_entries` with `struct_name::__AZUMI_LOCAL_FIELDS` and `struct_name::__AZUMI_COMPUTED_FIELDS` to generate compile‑time `compile_error!` diagnostics.
- [x] Updated validation‑item generation to query those struct constants for field name matches and produce targeted error messages.
- [x] Integrated the generated validation items into the struct implementation block after the original method definitions.
