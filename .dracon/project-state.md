# Project State

## Current Focus
Add a `drop_seo()` helper to clear the global SEO configuration during test setup

## Completed
- [x] Added `drop_seo()` function that locks `SITE_CONFIG` and sets it to `None`
- [x] Modified `test_generate_head_with_type` to call `drop_seo()` before `reset_seo()`
