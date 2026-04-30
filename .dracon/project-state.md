# Project State

## Current Focus
Replace the SEO global singleton with a thread‑safe mutable guard that can be reset for testing.

## Completed
- [x] Changed `SITE_CONFIG` from `OnceLock<SeoConfig>` to `Mutex<Option<SeoConfig>>`.
- [x] Updated `init_seo` to acquire the lock, insert the config only when empty, and emit a warning on subsequent calls.
- [x] Added `reset_seo` (test‑only) to clear the global state by locking and setting `None`.
