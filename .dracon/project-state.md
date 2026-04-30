# Project State

## Current Focus
Add static prediction and field constant lookups and implement LiveStateMetadata/LiveState traits for structs

## Completed
- [x] Added `__AZUMI_PREDICTIONS` constant populated from `predictions_entries`
- [x] Implemented `azumi::LiveStateMetadata` for the generated struct with `predictions()`, `struct_name()`, `local_fields()`, `computed_fields()`
- [x] Implemented `azumi::LiveState` for the generated struct with `to_scope()` returning signed JSON
- [x] Removed stale comment referencing manual LiveStateMetadata/LiveState implementations
- [x] Refactored macro to inline generate prediction and field arrays using static lookups
