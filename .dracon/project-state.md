# Project State

## Current Focus
Shift state handling to keep predictions in-memory via WeakMap and drop client‑side state mutation.

## Completed
- [x] Removed the `setState` client‑side method that mutated `az-scope`.
- [x] Removed the bottom‑level initialization code (`window.azumi = new Azumi();` etc.).
- [x] Modified `applyPrediction` to store state in `this.scopes` WeakMap instead of updating the `az-scope` attribute.
- [x] Simplified the rollback call to pass only `originalState`.
- [x] Updated prediction execution to retain original state and log results.
- [x] Dropped client‑side signature handling and attribute re‑signing.
