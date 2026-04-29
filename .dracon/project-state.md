# Project State

## Current Focus
Add and validate local‑state and computed‑field serialization logic through comprehensive live tests.

## Completed
- [x] Introduce `LocalCounterState`, `ServerOnlyState`, `NoLocalState`, and `ComputedState` structs annotated with `#[azumi::live]`, including default implementations.
- [x] Implement tests verifying that `to_local_scope()` includes only `#[local]` fields and excludes others.
- [x] Implement tests ensuring `to_scope()` includes only non‑local, non‑computed fields.
- [x] Add tests confirming that scopes are empty when no local fields exist.
- [x] Add tests confirming that computed fields marked with `#[computed]` are omitted from the serialized scope.
