# Azumi — Full Tasklist

> Current state: 2026-05-27. P0 features shipped. 225+ tests pass. 44KB/11KB bundle.

---

## ✅ DONE THIS SESSION

### Bug Fixes
- [x] Hot reload guard — conditional `if (window.location.port || ...)` in all copies
- [x] JS pipeline documented — `client/README.md`
- [x] Line count anomaly resolved — `src/client.min.js` bundles Idiomorph + Azumi, correct minification

### P0 Features
- [x] `bind:value` & `bind:checked` — two-way form binding (client + macro), 200ms debounce, checkbox/radio/nested paths
- [x] `@keyed(id)` — keyed list updates (client + parser + codegen), matches old↔new by `data-key`
- [x] `az-transition:fade` / `:slide` / `:scale` — enter/exit animations, configurable duration, works with @keyed
- [x] Shrunk predicate DSL — killed ternary (154 lines), `&&`/`||`, numeric comparisons. Only `!`, `==`, `!=`, truthy remain
- [x] `data-validate` extended — +3 rules: `max`, `min`, `pattern` (was 5, now 8)

### Framework Hardening
- [x] `class={"literal"}` banned — compile error, same as `class="literal"`. AI can't bypass validation
- [x] `class:external` purified — all Tailwind/Bootstrap examples replaced with component-oriented names (`payment-widget`, `cms-card`)
- [x] All 14 files with banned patterns migrated to `class:external="..."` or proper variables

### Tests
- [x] `tests/bind_value_tests.rs` — 8 tests (attribute gen, checkbox, nested, select/textarea, multi, preserve attrs)
- [x] `tests/keyed_tests.rs` — 8 tests (data-key, no-key, empty, string keys, first-only, @if inside, component integration)
- [x] `tests/transition_tests.rs` — 8 tests (fade/slide/scale, duration, multi, keyed, nested, preserve attrs)
- [x] UI test snapshots updated

### Documentation
- [x] `AGENTS.md` — +5 sections: bind:value, @keyed, Scoped CSS, Form Validation, Transitions
- [x] `AI_COOKBOOK.md` — new. 10 patterns, common mistakes, file-by-file template, quick-reference tables
- [x] `docs/guide.md` — +@keyed, +bind:value, +transitions sections, runtime size corrected to 11KB
- [x] `docs/why-azumi.md` — updated competitive landscape, feature comparison matrix, runtime size
- [x] `README.md` — feature table, updated comparisons, runtime size
- [x] `audit-tasks.md` — full framework audit (24KB)
- [x] `todo.md` — this file

---

## P1 — PREMIUM POLISH (Next Priority)

### Loading & Error States
- [ ] `az-loading` class on `az-action` elements during fetch — auto-add/remove, CSS `button[az-loading] { opacity: 0.6 }`
- [ ] `az-error` class on `az-target` on fetch failure — auto-display error fragment, clear on next success
- [ ] `Spinner` component — animated CSS spinner in prelude. ~20 lines Rust.

### Event Enhancements
- [ ] `debounce=N` on `az-on:input` — `az-on:input debounce=300 call search -> #results`. ~25 lines client.

### Lifecycle Hooks
- [ ] `az-on:mount` — fire after element appears in DOM (analytics, focus, widget init). ~25 lines client.
- [ ] `az-on:unmount` — fire before element leaves DOM (cleanup, save state). ~20 lines client.
- [ ] `az-on:update` — fire after element content changes (re-init widgets). ~20 lines client.

### Structured Confirms
- [ ] Component-based confirm — `az-confirm={MyConfirm}` instead of `window.confirm()`. ~40 lines client + component.
- [ ] Built-in `Confirm` component — accessible, keyboard support, focus trap. ~80 lines Rust.

### Reactive Declarations
- [ ] `#[react]` computed fields — `#[react] greeting: String` recomputed when deps change. ~80 lines client + ~40 macro.

---

## P2 — NICE TO HAVE

- [ ] Optimistic lists — `data-predict="append"` / `data-predict="remove"`. ~3KB.
- [ ] `az-intersect` — fire action on viewport enter/exit (infinite scroll, lazy load). ~1KB.
- [ ] Prediction undo animation — flash red on rollback. ~1KB.
- [ ] Scoped CSS stable hashing — content-based, not position-based. ~20 lines.
- [ ] `<style scoped>` / `<style global>` — explicit opt-in/out. ~10 lines parser.

---

## DISCOVERABILITY

- [ ] Working demo — verify `demo/` runs with `cargo run`. Add form_validation_demo to router.
- [ ] "Try in 60 seconds" — README section with one-command demo.
- [ ] "Azumi vs. HTMX" page — feature-by-feature comparison.
- [ ] "Azumi vs. Svelte" page — honest tradeoffs: Svelte wins ecosystem, Azumi wins no-npm + compile safety.
- [ ] "Azumi vs. Leptos" page — Azumi wins build speed + bundle size, Leptos wins reactivity.
- [ ] Submit to `arewewebyet.org` — Rust web framework comparison.
- [ ] Reddit `/r/rust` post
- [ ] This Week in Rust submission

---

## INFRASTRUCTURE

- [ ] Bundle size CI check — fail if gzipped > 12KB.
- [ ] JS build pipeline automated — currently manual: `client/azumi.js` → `build.rs` minifies → `src/client.min.js`.
- [ ] Client JS module split — `core.js`, `actions.js`, `state.js`, `validate.js`, `reveal.js` for development.
- [ ] Example repo — standalone `git clone → cargo run` app showing all features.

---

## RESEARCH (No Code Yet)

- [ ] HMAC optional — `#[azumi::live(security = "none")]` for unsigned state. Simpler deployment.
- [ ] `az-on` syntax — promote `on:click={state.method}` over `az-on:click call method`.
- [ ] Client routing lite — `az-action` that replaces `<body>` content for tab/wizard flows.
- [ ] WASM escape hatch — `#[azumi::wasm_component]` for compute-heavy components (canvas, editor).

---

## BUNDLE TRACKER

| State | Uncompressed | Gzipped | Notes |
|-------|-------------|---------|-------|
| Before session | 42KB | 10.5KB | Original |
| After P0 | 44KB | 11.2KB | +bind:value, +@keyed, +transitions, +3 validation rules, −ternary DSL |
| After P1 (est.) | ~48KB | ~12KB | +loading/error, +debounce, +lifecycle, +confirms, +#[react] |
| After P2 (est.) | ~53KB | ~13KB | +optimistic lists, +intersect, +undo anim |

---

## POSITIONING

> **Azumi = components + two-way binding + keyed lists + scoped CSS + transitions + form validation + optimistic UI + compile-time safety. All from `cargo build`. Zero npm. 11KB gzipped.**

| Competitor | Azumi wins on | Competitor wins on |
|-----------|--------------|-------------------|
| HTMX+Alpine | Components, scoped CSS, bind:value, @keyed, transitions, validation, compile safety | Community, ecosystem, simplicity |
| Svelte | No npm, Rust-native, compile-time CSS/HTML/route validation | Ecosystem, transitions, VSCode, community |
| Leptos | Build speed, bundle size, no Wasm, scoped CSS | Fine-grained reactivity, SPA routing, signals |

---

*Generated 2026-05-27. All line counts and sizes verified against source.*
