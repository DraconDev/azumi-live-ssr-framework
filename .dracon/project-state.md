# Project State

## Current Focus
Enforce stricter safety guarantees by completely banning `Raw()` usage and standardizing safe injection patterns

## Context
The project is implementing stricter security measures for HTML content handling, replacing the unsafe `Raw` type with safer alternatives while maintaining developer convenience.

## Completed
- [x] Removed `Raw` type and its documentation
- [x] Added hidden documentation for internal component constructor
- [x] Added `TrustedHtml` type for pre-sanitized HTML injection
- [x] Standardized safe injection patterns in documentation
- [x] Added procedural macros for JSON data, CSS, and JavaScript injection

## In Progress
- [x] Enforcing compile-time errors for `Raw()` usage

## Blockers
- Need to ensure all existing codebases are updated to use the new safe injection patterns

## Next Steps
1. Update all codebases to use the new safe injection macros
2. Add comprehensive documentation for the new injection patterns
3. Implement runtime validation for format strings in HTML expressions
