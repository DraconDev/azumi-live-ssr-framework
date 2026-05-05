# Project State

## Current Focus
Refactored inline injection patterns to use standard HTML tags instead of custom macros.

## Context
The previous approach used specialized macros (`inline_css!`, `inline_script!`) for injecting CSS and JavaScript, which were being phased out in favor of standard HTML tags with variable interpolation. This change aligns with the framework's stricter XSS protection policies and simplifies the template syntax.

## Completed
- [x] Replaced `inline_css!` macro with standard `<style>` tag using `{variable}` syntax
- [x] Replaced `inline_script!` macro with standard `<script>` tag using `{variable}` syntax
- [x] Updated validation rules to allow expression children in `<style>` tags
- [x] Improved documentation to reflect the new injection pattern

## In Progress
- [x] No active work in progress for this change

## Blockers
- None identified for this specific change

## Next Steps
1. Update related documentation and examples
2. Verify compatibility with existing template patterns
