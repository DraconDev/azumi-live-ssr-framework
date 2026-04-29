# Project State

## Current Focus
Add automatic struct name resolution and state signing to LiveStateMetadata and LiveState implementations.

## Completed
- [x] Impl `LiveStateMetadata::struct_name()` returning the struct name string.
- [x] Add empty `LiveStateMetadata::predictions()` method placeholder.
- [x] Implement `LiveState::to_scope` that serializes state to JSON and signs it.
- [x] Remove unnecessary struct name extraction code from `expand_live_impl`.
- [x] Update macro expansion to include new trait methods and state signing logic.
