# Project State

## Current Focus
Improved guidance for CSS usage patterns in Azumi's HTML structure validator

## Context
The validator was updated to reflect new procedural macros for safe CSS injection, replacing the previous Raw() usage pattern which bypassed Azumi's CSS scoping and validation.

## Completed
- [x] Updated error messages to recommend new procedural macros (inline_css!) instead of Raw() for CSS
- [x] Added examples for both small CSS (style blocks) and external CSS (inline_css!)
- [x] Removed outdated guidance about using Raw() for CSS
- [x] Updated documentation references to point to the correct guide section

## In Progress
- [ ] No active work in progress

## Blockers
- None identified

## Next Steps
1. Verify all existing codebases have migrated from Raw() CSS usage to the new macros
2. Update related documentation and examples throughout the project
