# Project State

## Current Focus
Refactored HTML content injection patterns to use standard HTML tags with auto-escaping

## Context
This change eliminates direct `Raw()` usage in favor of standard HTML tags (`<style>`, `<script>`) with built-in escaping, aligning with stricter XSS protection policies.

## Completed
- [x] Removed `inline_css!` and `inline_script!` macros in favor of standard HTML tags
- [x] Updated documentation to reflect new injection patterns
- [x] Modified `AZUMI_RULES` to document the new safe injection methods
- [x] Updated `Raw` documentation to emphasize safe alternatives

## In Progress
- [ ] No active work in progress

## Blockers
- None identified

## Next Steps
1. Update all codebases using the old injection patterns to use the new syntax
2. Verify all edge cases where special characters might have been previously unescaped
