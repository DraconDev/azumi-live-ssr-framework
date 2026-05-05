# Project State

## Current Focus
Add migration path documentation for making `PAGE_META` async-safe using `tokio::task_local!`

## Context
The current implementation uses `thread_local!` for `PAGE_META`, which is not async-safe. This change documents the migration path to use `tokio::task_local!` instead, including required type changes and considerations.

## Completed
- [x] Documented migration steps for async safety
- [x] Specified type changes needed (`Rc<AtomicU32>` → `Arc<AtomicU32>`)
- [x] Clarified `RefCell` usage remains compatible with `tokio::task_local!`

## In Progress
- [ ] Implementation of the migration (pending careful testing across async boundaries)

## Blockers
- Requires verification that the migration doesn't introduce race conditions in async contexts

## Next Steps
1. Implement the migration according to documented steps
2. Add comprehensive tests for async safety
