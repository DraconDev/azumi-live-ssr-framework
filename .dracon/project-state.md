# Project State

## Current Focus
Introduce client‑side ephemeral state management via `az-ui` and expression‑based bindings, restoring the `set` command and adding full expression evaluators.

## Completed
- [x] Document the new `az-ui` attribute and its ephemeral, no‑round‑trip behavior
- [x] Add `az-bind:class` and `az-bind:text` syntax for expression‑driven CSS class and text updates
- [x] Restore the `set` command for mutating `az-ui` state without server involvement
- [x] Implement `evaluatePredicate()` and `evaluateExpression()` full evaluators
- [x] Add `readState` priority resolution (WeakMap → `az-ui` → `az-scope`)
- [x] Update README release notes to version **30.3.1** and bump the project version
- [x] Update Cargo.toml version reference in README to **30.3.1**
- [x] Refactor client logging to use `log`, `warn`, and `error` methods instead of direct `console.*` calls
- [x] Add validation warnings for invalid `set` command format and empty `az-ui` attribute
- [x] Add warnings for prototype‑polluting path attempts
- [x] Update `tasks.md` to reflect the new version and feature set
