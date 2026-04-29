# Project State

## Current Focus
Preserve per-component `az-local-state` across Idiomorph outerHTML morphs to prevent client state loss during updates.

## Completed
- [x] Capture `az-local-state` from the target before morphing and restore it on the new element after morphing when Idiomorph is available.
