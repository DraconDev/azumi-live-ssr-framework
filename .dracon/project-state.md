# Project State

## Current Focus
Implement client‑side state reading and binding evaluation for new az‑bind directives.

## Completed
- [x] Add `readState(scopeElement)` to resolve state from WeakMap, az-ui, or az-scope in priority order.
- [x] Implement `evaluateBinding(expr, state)` to handle boolean negation, equality, inequality, and simple truthy checks.
- [x] Refactor `updateBindings` to use `readState`, support legacy `data-bind`, and new directive patterns:
  * `az-bind:text` for text content binding.
  * `az-bind:class:*` (colon syntax) and `az-bind.class.*` (dot syntax) for dynamic class toggling.
- [x] Remove duplicated state resolution logic from `updateBindings`.
- [x] Add graceful handling for browsers lacking `getAttributeNames`.
- [x] Update comments and documentation strings to reflect new binding behaviour.
