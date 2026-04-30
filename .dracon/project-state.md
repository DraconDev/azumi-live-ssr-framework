# Project State

## Current Focus
Add documentation for the new auto‑detected optimistic prediction system and update framework comparison to explain metadata injection.

## Completed
- [x] docs(scope): add detailed Prediction FAQ explaining `az-predictions`, how it’s generated, when it appears, and how to override or disable predictions
- [x] docs(scope): clarify in framework comparison that Azumi analyzes `&mut self` methods at compile time and injects prediction metadata as `az-predictions` JSON into the rendered HTML, enabling instant client updates without runtime prediction logic
- [x] updated FRAMEWORK_COMPARISON.md to reflect the new metadata injection mechanism and its impact on optimistic UI
