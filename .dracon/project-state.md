# Project State

## Current Focus
Migrate the SEO global singleton from OnceLock to a Mutex‑protected Option to enable safe mutable access and resetting.

## Completed
- [x] Replace `std::sync::OnceLock` import with `std::sync::Mutex`
- [x] Change `static SITE_CONFIG` from `OnceLock<SeoConfig>` to `Mutex<Option<SeoConfig>>` initialized with `None`
