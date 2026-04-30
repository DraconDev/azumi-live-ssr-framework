# Project State

## Current Focus
Replace OnceLock with Mutex for thread‑safe global SEO config and add reset capability

## Completed
- [x] Changed `SITE_CONFIG` static from `OnceLock<SeoConfig>` to `Mutex<Option<SeoConfig>>`
- [x] Updated `init_seo()` to lock, set config only if empty, and emit warning on repeated init
- [x] Added `reset_seo()` to clear the stored configuration via lock
- [x] Modified `generate_head()` to acquire lock and clone configuration data as needed
- [x] Replaced direct `global` accesses with `Ref`/`as_ref` patterns and explicit cloning of `base_url`
- [x] Updated open‑graph and Twitter handling to use reference semantics (`ref`/`if let Some(ref ...)`)
- [x] Updated `Cargo.lock` to reflect new dependency versions introduced by the concurrency change
