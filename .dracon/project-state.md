# Project State

## Current Focus
Auto‑injection of prediction metadata into generated HTML via `az-predictions` JSON

## Completed
- [x] Simple mutations are auto‑analyzed and stored in `LiveStateMetadata`
- [x] Component macro injects predictions as `az-predictions` JSON on the scope div
- [x] Client JS reads this attribute and auto‑executes predictions on button clicks
- [x] Documentation updated to reflect auto‑injection and manual `data-predict` usage
- [x] Retained guidance for complex cases using `#[azumi::predict("...")]`
