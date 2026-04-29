# Project State

## Current Focus
Refactor Escaped<T> Display to bypass formatter and emit via EscapedWriter while keeping HTML-escaped rendering.

## Completed
- [x] Replace `f` usage with `_f` in `Escaped<T>::fmt` and route formatting through `EscapedWriter`; preserves escape behavior without writing to the supplied formatter.
- [x] Update Cargo.lock (dependency tree refresh, no semantic version bumps in public API).
