# Project State

## Current Focus
Refactor client-side state predictions to use in-memory WeakMap storage instead of modifying server-signed `az-scope` attributes

## Completed
- [x] Remove `setState` method - local-only state changes no longer supported
- [x] Refactor `updateBindings` to read predictions from WeakMap first, fall back to server state
- [x] Add dot-notation support for nested field binding (e.g., "user.name")
- [x] Simplify `rollbackPrediction` to delete WeakMap entries instead of restoring attributes
- [x] Remove `set` action type from `execute` method
- [x] Update dependency versions in Cargo.lock
