# Project State

## Current Focus
Adds an `az-predictions` attribute to the generated HTML component when prediction metadata is present and non‑empty.

## Completed
- [x] Extract predictions via `LiveStateMetadata::predictions()` in component rendering
- [x] Serialize predictions to a JSON string for attribute value
- [x] Conditionally render `az-predictions` attribute only when the JSON is non‑empty and not `"[]"`
