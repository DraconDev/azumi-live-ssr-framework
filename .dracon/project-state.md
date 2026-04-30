# Project State

## Current Focus
Introduce a cloned `global_for_title` variable to safely reuse SEO config in title generation and adjust calls to use `as_ref()` for borrowing safety, plus add `reset_seo()` in the unit test.

## Completed
- [x] Clone `global` into `global_for_title` before using it later
- [x] Change `.or(global.and_then(...))` to `.or(global.as_ref().and_then(...))` for safe reference
- [x] Update the `full_title` condition to use `global_for_title` instead of `global`
- [x] Add `reset_seo();` call in `test_generate_head_with_type` to reset global state
