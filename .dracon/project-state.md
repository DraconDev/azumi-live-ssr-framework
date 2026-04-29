# Project State

## Current Focus
Add compile‑time validation checks for predictions that reject references to local or computed fields

## Completed
- [x] Introduce `validation_items` vector to collect compile‑time warnings
- [x] Iterate over `analysis.predictions` and generate `__AZUMI_CHECK` constants per prediction
- [x] Dynamically reject predictions that reference local fields using `local_const_entries`
- [x] Dynamically reject predictions that reference computed fields using `computed_const_entries`
- [x] Emit `compile_error!` messages when a prediction mutates a forbidden field
