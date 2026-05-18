# Azumi TODO

## Positioning & Docs Rewrite (P0)

The framework is 9/10 technically, 5/10 on story. Core pitch: **HTMX that actually works for interactive apps. No JS ecosystem, no ecosystem churn.**

- [x] **P0: Rewrite README.md** — Subtitle: "Server-rendered HTML with client interactivity — all Rust, zero custom JavaScript". Origin story, trilemma table, pitch, quickstart, when to use / when NOT to.
- [x] **P0: Create `docs/why-azumi.md`** — Full narrative: origin, two problems, thesis, competitive landscape, "Azumi is complete", "no custom JS" proof, when to use / when NOT to.
- [x] **P1: Restructure `docs/guide.md`** — Inlined interactivity pattern catalog. Updated TrustedHtml as public.
- [x] **P1: Kill doc sprawl** — 12 docs → 3 (README, why-azumi, guide) + docs/archive/.
- [x] **P1: "Stability as a feature" + "No custom JS" messaging** — In README + why-azumi. Production proof: 3 sites, 98 html! calls, zero custom JS.
- [x] **P1: Honest scope section** — "When NOT to use Azumi" in README + why-azumi.
- [x] **P2: Update crate-level doc** — src/lib.rs with subtitle, quick start, key features.
- [x] **P2: Real-world benchmark data** — Added runtime size comparison table (Azumi 10KB gzipped vs HTMX 15KB vs React 46KB). Updated README + why-azumi with real measurements.
- [ ] **P2: Production example app** — Replace toy demo with something production-caliber.

## Framework Improvements (from dracon-platform audit)

- [x] **FN-034 (HIGH):** Borrow-friendly component props — `&str`/`&T` params in `#[azumi::component]` auto-inject lifetimes. Zero-clone rendering from `&self`.
- [x] **FN-035 (NORMAL):** Migration guide — `docs/migration/from-axum.md` with 6-step incremental adoption path (add dep → html! → component → action → live → safe patterns)
- [x] **FN-033 (HIGH, partial):** `TrustedHtml` promoted to public API ✅. `html_content!` evaluated — **not needed**, TrustedHtml already solves CMS injection. Closed.
- [x] **FN-036 (NORMAL):** Demo review — homepage positioning updated, blog actions.rs format!→html! fixed, route constants added to homepage+lesson9, .clone() audited (legitimately needed), no Raw/aria issues found.

## Future / Consider

- [ ] P2: i18n / localization — `t!("key")` macro with compile-time key validation
- [ ] P2: Email template rendering — `#[azumi::email]` macro with CSS inlining + plain-text generation
- [ ] P3: Token parser modularization — `token_parser.rs` (1,349 lines) could be split into submodules

## Completed ✅

<details>
<summary>All completed items (collapse)</summary>

- [x] P0: Fix demo `Form` import
- [x] P0: Fix all demo static attribute errors
- [x] P0: Fix demo type errors
- [x] P0: Fix unused import warnings in demo
- [x] P0: CI already exists
- [x] P1: Make doc-tests runnable (14 passing, 16 ignored)
- [x] P1: Split `html_structure_validator.rs` into sub-modules
- [x] P0: Fix LiveState `panic!` on serde failure
- [x] P0: Hot reload production guard
- [x] P1: Route constants for compile-time link safety
- [x] P1: `#[live_state]` explicit attribute
- [x] P1: Configurable state max age
- [x] P1: Cache devtools env var check
- [x] P1: Unify live state error messages
- [x] P1: Fix `az-action` expression evaluation bug
- [x] P1: Tests for route constants
- [x] P1: Demo showcase route constants
- [x] P1: AGENTS.md updated
- [x] P2: TypeScript definitions for `azumi.js` (client/azumi.d.ts)
- [x] P2: aria-* value validation (15+ aria attributes)
- [x] P3: Benchmarks in CI (already existed)
- [x] P3: Vendored idiomorph checksum (FNV-1a in build.rs)
- [x] Security: Dev token timing attack fixed
- [x] Security: AZUMI_DEV_TOKEN cached
- [x] Quality: `escape_tag_content` no longer panics
- [x] Quality: `init_seo` returns `SeoError` enum
- [x] Quality: Devtools headers use `HeaderValue::from_static`
- [x] Cleanup: Zero clippy warnings workspace-wide
- [x] Cleanup: Removed `.ralph/` stale files
- [x] Cleanup: `DESIGN_ROUTE_SAFETY.md` resolution note
- [x] API: `TrustedHtml` promoted to public API + prelude
- [x] Research: dracon-platform audit (98 html! calls, 30+ Raw(), 20+ clone())
- [x] Research: Competitive landscape analysis (Leptos 0.8, Dioxus 0.6, HTMX, SvelteKit 2)
- [x] Positioning: Full assessment written to POSITIONING_2026-05-17.md
</details>
