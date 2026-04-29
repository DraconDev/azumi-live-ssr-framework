# Project State

## Current Focus
Emit compile‑time validation constants for local and computed fields

## Completed
- [x] Added `__AZUMI_LOCAL_FIELDS` constant array containing local field identifiers
- [x] Added `__AZUMI_COMPUTED_FIELDS` constant array containing computed field identifiers
- [x] Integrated the constants into the generated impl block for validation by `#[azumi::live_impl]`
