# Project State

## Current Focus
Add per-component local state scoping via `az-local-state` and split signed scope from local-only fields.

## Completed
- [x] Component macro now wraps render output in a conditional `az-local-state` container when live state has local fields, enabling client-side hydration of local-only data without affecting global scope.
- [x] Live struct macro validates named fields and generates separate `to_scope` (regular fields) and `to_local_scope` (local-marked fields) methods, plus `LiveStateMetadata::local_fields` listing.
- [x] Library exposes default `local_fields` trait method for namespacing and metadata queries.
- [x] Cargo.lock updated to reflect resolved dependency versions.
