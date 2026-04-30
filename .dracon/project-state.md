# Project State

## Current Focus
Replace OnceLock with Mutex for SEO global state and add reset_seo() call in test_generate_head_with_type

## Completed
- [x] Switched `use std::sync::OnceLock;` to `use std::sync::Mutex;` in `src/seo.rs`.
- [x] Added `reset_seo();` invocation before `generate_head` in `test_generate_head_with_type`.
