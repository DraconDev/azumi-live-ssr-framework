# Project State

## Current Focus
Simplify `set` action handling and tighten numeric comparison logic in the Azumi expression parser.

## Completed
- [x] Remove unnecessary normalization logic from the `set` action, returning the action directly.
- [x] Update numeric comparison regexes to use `\d+(?:\.\d+)?`, preventing accidental matches with non‑numeric tokens.
- [x] Correct string literal unescaping regex to properly handle escaped `"` and `\`.
- [x] Harmonise increment/decrement regex patterns with the new numeric format.
