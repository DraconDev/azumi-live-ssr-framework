# Project State

## Current Focus
Migrate SEO global state from OnceLock to Mutex and implement safe cloning with ref handling

## Completed
- [x] Updated Cargo.lock to reflect latest dependency versions
- [x] Replaced direct `global` access with `SITE_CONFIG.lock().ok().and_then(|guard| guard.clone())` and introduced `global_for_title` clone
- [x] Updated description and image fall‑back logic to use `global.as_ref()` instead of raw `global`
- [x] Modified `full_title` computation to use `ref g` and safely access `site_name`
- [x] Changed `base_url` retrieval to clone via `global.as_ref().and_then(|g| g.base_url.clone())`
- [x] Updated open_graph and Twitter conditional branches to use `ref g` and `as_ref()` checks
- [x] Switched all related pattern matches to use reference semantics for non‑Copy values
