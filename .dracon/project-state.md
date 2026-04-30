# Project State

## Current Focus
Adds az‑predictions attribute to component HTML when predictions metadata is available

## Completed
- [x] Retrieves predictions metadata via `LiveStateMetadata::predictions`
- [x] Serializes predictions to JSON string safely with fallback empty string
- [x] Appends `az-predictions` attribute only if predictions are non‑empty and not the default array
