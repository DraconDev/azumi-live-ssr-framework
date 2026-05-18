# Azumi Open Items — Systematic Completion

Remaining tasks from review, prioritized by impact:

## P0: FN-034 — Borrow-friendly component props
`#[azumi::component]` currently generates `Props` with owned fields. When a component is called from `render()` with `&self` fields, users must `.clone()` because the builder requires owned values.

Goal: Allow `&str` (and potentially other borrowed types) in component parameters so `{&field}` works without clones.

This is the #1 ergonomic pain point from the dracon-platform audit (20+ clones per page).

## P1: FN-035 — Migration guide
Write `docs/migration/from-axum.md` showing how to incrementally adopt Azumi from a plain Axum app:
1. Add `azumi` dependency, keep existing routes
2. Replace one handler with `html!`
3. Add `#[azumi::component]` for repeated UI
4. Add `#[azumi::action]` for one form
5. Add `#[azumi::live]` for one counter
6. Replace `format!` + `Raw()` with `@for`, `{&field}`, `json_data!`

## P2: Benchmarks
Add real benchmark data to README/why-azumi:
- Lighthouse scores (Performance, Accessibility, Best Practices, SEO)
- TTFB comparison vs HTMX + plain server, Leptos SSR, Dioxus SSR
- Bundle size comparison (azumi.js ~3KB vs HTMX ~14KB vs React ~40KB)

## P2: Production example app
Replace toy demo with something production-caliber (blog with auth, comments, admin panel).

## P3: token_parser.rs modularization
Split 1,349-line file into submodules.

Checklist:
- [ ] FN-034: Borrow-friendly component props
- [ ] FN-035: Migration guide from Axum
- [ ] P2: Benchmark data in docs
- [ ] P2: Production example app plan
- [ ] P3: token_parser.rs modularization
- [ ] All tests pass after each change