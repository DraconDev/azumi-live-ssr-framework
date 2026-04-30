# Project State

## Current Focus
Refactor component macro to centralize live‑state scope wrapper generation and ensure SEO tests start from a clean state.

## Completed
- [x] Extracted live‑state scope HTML wrapper into a shared `scope_body` variable in the component macro, removing duplicated code in render function generation.
- [x] Updated SEO unit tests to call `reset_seo()` before initialization, guaranteeing isolated test conditions.
