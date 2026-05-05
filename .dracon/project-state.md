# Project State

## Current Focus
Refactored context management by moving the `Context` enum to a shared module.

## Context
The `Context` enum was previously defined in the macros module but is now being shared across the codebase for consistent HTML rendering modes and CSS scoping.

## Completed
- [x] Moved `Context` enum to shared `context` module
- [x] Removed redundant `Context` enum definition in macros module

## In Progress
- [x] Updating dependent code to use the new shared `Context` enum

## Blockers
- None identified

## Next Steps
1. Update all references to `Context` in the codebase to use the shared version
2. Verify consistent behavior across all HTML rendering modes
