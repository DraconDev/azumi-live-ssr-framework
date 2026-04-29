# Project State

## Current Focus
Add comprehensive unit tests for component CSS validation functions.

## Completed
- [x] Added test `test_no_external_css_returns_empty` ensuring empty nodes produce empty result.
- [x] Added test `test_style_without_src_returns_empty` confirming style without src is allowed.
- [x] Added test `test_style_with_src_fails` verifying external CSS triggers compile_error and mentions ban.
- [x] Added test `test_global_css_allowed` confirming global.css is permitted.
- [x] Added test `test_collect_no_css_files` verifying empty collection.
- [x] Added test `test_collect_style_with_src` checking collection of a CSS file.
- [x] Added test `test_collect_skips_global_css` ensuring global.css is skipped.
- [x] Added test `test_collect_recurses_into_children` verifying recursion into child elements.
- [x] Added test `test_resolve_strips_leading_slash` confirming leading slash removal.
- [x] Added test `test_resolve_relative_path` confirming relative path handling.
- [x] Updated Cargo.lock to reflect dependency version changes.
