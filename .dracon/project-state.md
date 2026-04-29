# Project State

## Current Focus
Refactor client‑side state handling to keep predictions in an in‑memory WeakMap, make `az-scope` immutable after render, and remove the legacy `set` action.

## Completed
- [x] Removed support for `"set"` actions in `execute`, limiting execution to `"call"` actions only.
- [x] Introduced `this.scopes` WeakMap to store optimistic prediction state per scope element, keeping it ephemeral and client‑only.
- [x] Updated prediction execution to parse signatures but **no longer modify** the `az-scope` attribute; instead, the new state is saved in the WeakMap.
- [x] Simplified `updateBindings` to read state from the WeakMap first, falling back to parsing the signed `az-scope` attribute only for initial values.
- [x] Refactored `rollbackPrediction` to restore the original state from the WeakMap and clear the entry when no original state exists, leaving the server‑signed `az-scope` untouched.
- [x] Adjusted all call sites (e.g., after a server response) to use the new rollback signature that takes only `originalState`.
- [x] Deleted the entire `setState` implementation and related signature handling code, eliminating client‑side mutations of the signed scope attribute.
- [x] Cleaned up related console messages and comments to reflect the new immutable‑scope model.
