# Project State

## Current Focus
Switched SEO global state from OnceLock to a Mutex‑protected guard and added a reset helper for tests

## Completed
- [x] Refactored `init_seo` to use `SITE_CONFIG.lock()` and guard.is_none() to preserve first initialization
- [x] Added `reset_seo()` test function that clears the global SEO configuration
- [x] Updated concurrency handling to use Mutex instead of OnceLock.set
