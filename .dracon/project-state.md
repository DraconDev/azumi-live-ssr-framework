# Project State

## Current Focus
Refactored inline injection patterns to use standard HTML tags instead of direct `Raw()` usage

## Context
This change continues the security-focused refactoring of the token parser to enforce stricter XSS protection by eliminating direct `Raw()` usage in favor of safer injection patterns.

## Completed
- [x] Replaced `azumi::inline_script!(AI_HUB_COPY_JS)` with `<script>{TRACKING_JS}</script>`
- [x] Replaced `azumi::inline_css!(HUB_GLOBAL_CSS)` with `<style>{GLOBAL_CSS}</style>`
- [x] Updated error messages to recommend external files or expressions instead of `Raw()`
- [x] Simplified allowed injection patterns to only permit `json_data!` macro or data-* attributes for scripts
- [x] Simplified allowed injection patterns to only permit `<style>{variable}</style>` or style attributes for CSS

## In Progress
- [x] This refactoring is part of the ongoing security initiative to eliminate all `Raw()` usage

## Blockers
- None identified in this commit

## Next Steps
1. Verify all affected components still function correctly with the new injection patterns
2. Update documentation to reflect the new safer injection patterns
