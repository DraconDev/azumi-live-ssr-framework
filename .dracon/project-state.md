# Project State

## Current Focus
Add predictions metadata and LiveState implementation for structs in macro expansions.

## Completed
- [x] Introduced `__AZUMI_PREDICTIONS` const storing static prediction entries derived from `predictions_entries`.
- [x] Implemented `predictions()` method in `LiveStateMetadata` to return `__AZUMI_PREDICTIONS`.
- [x] Added `struct_name()`, `local_fields()`, and `computed_fields()` methods returning respective static field name slices.
- [x] Added `LiveState` implementation with `to_scope()` serializing the struct to JSON and signing it.
- [x] Removed obsolete comment about not implementing `LiveStateMetadata`.
