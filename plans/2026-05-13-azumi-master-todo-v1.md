# Azumi — Master TODO

> Full-stack Rust web framework. Simple, secure, compiled.
> Born from: *"My backend is Rust — why is my frontend the thing that breaks?"*

---

## The North Star

A Rust full-stack framework where:

- **No second language** for the frontend. One compiler from DB schema to DOM attribute. No type-safety gap at the `fetch()` boundary.
- **No WASM download tax.** ~3KB runtime. HTML is truth. Interactivity is surgical, not the default.
- **Compiler catches frontend bugs.** CSS class typos, bad HTML nesting, missing alt text, wrong attributes — blocked at compile time, not in production.
- **Feels simple.** Not Leptos-complex (no reactivity model to learn). Not Dioxus-heavy (no WASM build pipeline). Just Rust + `html!` + attributes.

**The problem this solves:**
> *"My backend is Rust — rock solid, memory safe, compiled. My frontend is JS/TS — runtime errors, npm fragility, undefined is not a function. Why?"*

**Competitor positioning:**
| Competitor | Why not |
|-----------|---------|
| Next.js / SvelteKit | Language boundary at fetch(). Two type systems, two build pipelines, two ecosystems. Type safety breaks at the API call. |
| Leptos / Dioxus | WASM download tax (~150KB+). Complex reactivity model. DOM bridge overhead. Overkill for "form submit" and "button click." |
| HTMX | No type safety — everything is strings (URLs, swap targets, triggers). The compiler can't help you catch mistakes. |
| Maud | No interactivity story. Just templates — no form actions, no live state, no optimistic UI. |

**Differentiator:** Compile-time CSS-HTML co-validation + zero-hydration architecture + HMAC-signed state + ~3KB runtime + AI-ready validation pipeline.

---

## Progress

| Phase | Status | Last Updated |
|-------|--------|-------------|
| P0: Foundation | **4/4 complete** (azumi-cli ✅, routes! macro ✅, docs consolidation ✅, quickstart ✅) | 2026-05-12 |
| P1: API Polish | **4/4 complete** (live+live_impl merge ✅, prelude cleanup ✅, v48 promise ✅, page deprecation ✅) | 2026-05-12 |
| P2: Interactivity | **4/4 complete** (ActionResult ✅, error messages ✅, patterns docs ✅, head! deprecation ✅) | 2026-05-12 |
| P3: Production | **3/4 complete** (Axum optional ✅, benchmarks ✅, security audit ✅, streaming HTML pending) | 2026-05-13 |
| P4: Ecosystem | **4/4 complete** (form validation ✅, SSE ✅, component library ✅, example app ✅) | 2026-05-13 |
| Dracon Platform | Ongoing (tag bump ✅, docs eval ✅, class consistency ✅, Raw→safe injection ✅) | 2026-05-13 |
| Review Fixes | **5/5 complete** (CLI version ✅, page hidden ✅, re-exports ✅, Redirect ✅, docs accuracy ✅) | 2026-05-13 |

---

## Non-Goals (Things We Will NOT Do)

| Temptation | Why Not |
|-----------|---------|
| **ORM integration** | Pick sqlx/diesel/sea-orm independently. We provide patterns (demo), not packages. |
| **Authentication package** | Too many variants (sessions, JWT, OAuth, OIDC, magic links). Signed state is the foundation; let ecosystem handle auth. |
| **CSS framework integration** | Tailwind/Material/Bootstrap change too fast. Our compile-time validation is the right abstraction level. |
| **WASM / SSR hybrid** | Defeats "no WASM" philosophy. Point users to Leptos if they want WASM. We own "server-rendered with surgical interactivity." |
| **Client-side SPA routing** | MPA + Idiomorph is simpler, faster, no JS-for-navigation requirement. |
| **File-system based routing** | `routes!` macro is simpler, fully type-checked, no build scripts or filesystem coupling. |
| **Custom build tooling** | `cargo` works fine. No Vite/Rollup/Webpack equivalent needed. |
| **Separate template language** | `html!` IS the template language — it's Rust. No syntax highlighting issues, no editor plugins. |

---

## Remaining Work — Prioritized

### NEXT SESSION (P5: Release & Adoption)

- [ ] **P5.1** — Publish `azumi` core crate to crates.io (CLI is already published)
- [ ] **P5.2** — Create v48.0.0 git tag with semver promise (bump version from 47.20.29 → 48.0.0)
- [ ] **P5.3** — Update dracon-platform Cargo.toml to reference v48.0.0
- [ ] **P5.4** — Create more example apps: blog app (CRUD), dashboard (multi-route)
- [ ] **P5.5** — Community outreach: post to r/rust, Hacker News Show HN, write blog post

### SHORT-TERM (P6: Polish)

- [ ] **P6.1** — Implement streaming HTML (P3.2): optional `async fn render_stream()` on Component
- [ ] **P6.2** — Audit and document the `class={format!(...)}` patterns in billing.rs — consider pre-formatting with variables
- [ ] **P6.3** — Evaluate ai-hub-copy.ts (199 lines) for Rust/Azumi simplification
- [ ] **P6.4** — Add `vstack-0` utility class (gap: 0) for `vstack` with no gap
- [ ] **P6.5** — Consolidate remaining `flex-direction: column` patterns in ai-hub (9 inline patterns)
- [ ] **P6.6** — Cross-repo integration tests: add `cargo test --workspace` to dracon-platform CI

### MEDIUM-TERM (P7: Growth)

- [ ] **P7.1** — Create `azumi-ui` component library crate (Button, Input, Card, Modal, Tabs, Accordion, Form)
- [ ] **P7.2** — Add template hot reload via WebSocket + Idiomorph server-side diff
- [ ] **P7.3** — Form validation macro: `validated!` macro with auto-generated `aria-invalid`
- [ ] **P7.4** — SSE documentation and tutorial (feature exists in `src/streaming.rs` but has no docs)

### LONG-TERM (P8: Maturity)

- [ ] **P8.1** — Truly merge `#[live]` struct + impl into a single macro expansion (currently a dispatcher)
- [ ] **P8.2** — Add `azumi check` CLI command for offline validation without full build
- [ ] **P8.3** — Write comprehensive API reference documentation (auto-generated or hand-maintained)
- [ ] **P8.4** — Performance benchmarks: macro expansion time, memory at scale, concurrent render throughput

---

## Dracon Platform — Tactical Items

| Item | Status | Notes |
|------|--------|-------|
| DP.1 — Tag bump v47.19.10 → v47.20.29 | ✅ Done | Cargo.toml confirmed at v47.20.29 |
| DP.2 — Migrate `Raw(format!("<style>"))` to safe injection | ✅ Done | 6 patterns in chrome + 2 in ai-hub → `<head>` with `<style>{var}</style>` |
| DP.3 — Fix `class={bare_ident}` inconsistency | ✅ Done | footer.rs (6 fixes), nav.rs (22 fixes), nav.rs logo format! (1 fix) |
| DP.4 — Fix `class={format!(...)}` anti-patterns | ✅ Done | 5 files: nav.rs, base.rs, pages.rs, billing.rs, sidebar.rs |
| DP.5 — Fix `vstack-12` undefined class | ✅ Done | pages.rs:392 → `vstack-15` |
| DP.6 — Remove dead `vstack-02` CSS | ✅ Done | base_css.rs (0 usages) |
| DP.7 — Update AGENTS.md version refs | ✅ Done | 47.19.10 → 47.20.29 |
| DP.8 — Evaluate ai-rankings.ts for Rust simplification | ✅ Done | Keep as TypeScript — real-time slider computation requires client-side JS |
| DP.9 — Audit remaining TS for Azumi-native alternatives | ⏳ Next | Need to evaluate ai-hub-copy.ts (199 lines) |
| DP.10 — CSS consolidation (~18 patterns remain) | 🔍 Evaluated | All remaining are legitimate (padding, background, border, responsive overrides) |
| DP.11 — TS pipeline monitoring | 🔍 Monitoring | 2 TS files + 1 .d.ts — `tsc` is fine; switch to esbuild if >5 files |
| DP.12 — Cross-repo integration tests | ⏳ Pending | Add `cargo test --workspace` to platform CI |

---

## Current Build Status

| Check | Result |
|-------|--------|
| `cargo build -p azumi` | ✅ |
| `cargo test --workspace` (azumi) | ✅ 1,678 passed, 0 failed |
| `cargo build --no-default-features` (azumi, no axum) | ✅ |
| `cargo build -p azumi-todo-example` | ✅ |
| `azumi-cli new my-app` | ✅ Verified end-to-end |
| Dracon-platform `cargo check -p chrome` | ✅ |
| Dracon-platform `cargo test -p chrome` | ✅ 132 passed |
| Dracon-platform `cargo check -p ai-hub-app` | ✅ |
| Dracon-platform `cargo check -p dashboard-app` | ✅ |
| **Total cross-repo tests** | **1,810 passed, 0 failed** |

---

## Key Files Modified (This Arc of Work)

### Azumi Framework (`/home/dracon/Dev/azumi/`)

| File | Change |
|------|--------|
| `Cargo.toml` | Axum optional feature, new benches, example workspace |
| `src/lib.rs` | Prelude cleanup (13→9), `routes!` macro, hidden re-exports |
| `src/action.rs` | `ActionResult` enum (Ok/Err/Redirect), `IntoResponse` impls |
| `src/form.rs` | New: `FormValidator`, `ValidatedForm`, ARIA-accessible components |
| `src/streaming.rs` | New: `SseEvent`, `sse()` helper, keep-alive |
| `src/hot_reload.rs` | Fix: `Message::Text(msg.into())` for Axum 0.8 |
| `macros/src/lib.rs` | `#[doc(hidden)]` on page/head/live_impl/predict |
| `macros/src/live.rs` | `expand_live` dispatcher (struct + impl) |
| `macros/src/action.rs` | Auto-detect `ActionResult` return type |
| `macros/src/css_validator.rs` | Improved error messages with alternatives |
| `cli/` | New: `azumi-cli` scaffolding, `azumi new`, components template |
| `benches/full_page.rs` | New: full-page render benchmark |
| `docs/guide.md` | New: consolidated framework guide (566 lines) |
| `docs/interactivity.md` | New: pattern catalog (421 lines) |
| `docs/comparison.md` | New: framework comparison (74 lines) |
| `docs/SECURITY_AUDIT.md` | New: security audit document |
| `examples/todo/` | New: working todo example app |
| `README.md` | Rewritten: 200 lines, brand + quickstart + stability promise |
| `CHANGELOG.md` | v48.0.0 entry with semver commitment |

### Dracon Platform (`/home/dracon/Dev/dracon-platform/`)

| File | Change |
|------|--------|
| `Cargo.toml` | Azumi tag bump v47.19.10 → v47.20.29 |
| `libs/chrome/src/base_css.rs` | Added vstack utilities, removed dead `vstack-02` |
| `libs/chrome/src/components/pages.rs` | vstack-12 → vstack-15, class={format!} migrations |
| `libs/chrome/src/components/base.rs` | class={format!} migrations, dead CSS removal |
| `libs/chrome/src/nav.rs` | 22 bare identifiers fixed, logo format! fix, nav_class pre-format |
| `libs/chrome/src/footer.rs` | 6 bare identifiers fixed, brand_col/custom_col → vstack |
| `libs/chrome/src/render.rs` | 5 `Raw(format!("<style>"))` → safe `<head>` injection |
| `sites/ai-hub/src/render.rs` | 2 `Raw(format!("<style>"))` → safe `<head>` injection |
| `sites/dashboard-app/src/chrome/sidebar.rs` | class={format!} migration |
| `sites/dashboard-app/src/render/billing.rs` | class={format!} pre-format |
| `AGENTS.md` | Version refs updated 47.19.10 → 47.20.29 |

---

*Last updated: 2026-05-13*
*Direction: Full-stack Rust framework — simple, secure, compiled. One language from DB to DOM.*
*"My backend is Rust — why is my frontend the thing that breaks?"*
