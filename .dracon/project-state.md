# Project State

## Current Focus
Refactored HTML content injection patterns to use standard HTML tags instead of direct `Raw()` usage

## Context
This change eliminates direct `Raw()` usage in favor of safer HTML tag-based injection patterns, aligning with Azumi's XSS protection goals and improving code consistency

## Completed
- [x] Replaced `inline_css!` and `inline_script!` macros with standard `<style>` and `<script>` tag patterns
- [x] Updated documentation to reflect new injection patterns
- [x] Maintained all safety guarantees while improving code readability

## In Progress
- [x] Refactoring of related test cases to use new patterns

## Blockers
- None identified

## Next Steps
1. Update related documentation sections
2. Verify all test cases are working with new patterns
