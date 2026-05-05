# Project State

## Current Focus
Enhance security and API consistency by adding `#[must_use]` annotations to public functions

## Context
The changes add `#[must_use]` annotations to public functions to improve API safety by making it clear when return values must be used. This prevents accidental discarding of important return values that could lead to security vulnerabilities or incorrect behavior.

## Completed
- [x] Added `#[must_use]` to `from_fn` in lib.rs
- [x] Added `#[must_use]` to `from_fn_once` in lib.rs
- [x] Added `#[must_use]` to `render_to_string` in lib.rs
- [x] Added `#[must_use]` to `escape_css_string` in lib.rs
- [x] Added `#[must_use]` to `compute_scope_id` in lib.rs
- [x] Added `#[must_use]` to `scope_css` in lib.rs
- [x] Added `#[must_use]` to `azumi_script` in lib.rs
- [x] Added `#[must_use]` to `escape_tag_content` in script.rs
- [x] Added `#[must_use]` to `escape_script_content` in script.rs
- [x] Added `#[must_use]` to `escape_style_content` in script.rs
- [x] Added `#[must_use]` to `session_cleanup_script` in script.rs
- [x] Added `#[must_use]` to `sign_state` in security.rs
- [x] Added `#[must_use]` to `sign_state_for_user` in security.rs
- [x] Added `#[must_use]` to `verify_state` in security.rs
- [x] Added `#[must_use]` to `verify_state_for_user` in security.rs

## In Progress
- [x] All `#[must_use]` annotations have been added

## Blockers
- None

## Next Steps
1. Verify all annotated functions are actually meant to be `#[must_use]`
2. Update documentation to reflect the new API safety guarantees
