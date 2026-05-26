# Azumi — Current TODO

> Updated 2026-05-26. P0 features shipped. Focus: transitions + discoverability.

---

## P0 — THE LAST FEATURE GAP

### Transitions (`az-transition:`)

- [ ] **`az-transition:fade`** — Fade in/out on DOM enter/exit. `az-transition:fade` on any element. After Idiomorph morph, detect new → fade in (`opacity: 0→1`), detect removed → fade out → then remove. ~40 lines, ~1.5KB.
- [ ] **`az-transition:slide`** — Slide from top/bottom. ~30 lines, ~1KB.
- [ ] **`az-transition:scale`** — Scale 0.95↔1.0 with opacity. ~20 lines, ~0.5KB.
- [ ] **Config**: `az-transition:fade duration=300 delay=100`. ~15 lines.
- [ ] **Test** transitions (enter/exit/reorder with @keyed). ~8 cases.
- [ ] **Update AGENTS.md** and `docs/guide.md` with transition reference.

**Total:** ~3KB. Closes the premium-feel gap with Svelte.

---

## P0.5 — DISCOVERABILITY (High Leverage)

### Landing Experience

- [ ] **Working demo at `demo/`** — already exists. Verify it runs with `cargo run`. Add the new `form_validation_demo` page to the router. Add a page showcasing `bind:value` + `@keyed` + transitions together.
- [ ] **"Try in 60 seconds" section** in README — `cargo run --example demo` and you see the working app. Screenshot or GIF.
- [ ] **`why-azumi.md` → update** with new comparison (after P0 features). Add runtime size claim: "10.5KB gzipped." Add feature table showing what Azumi has that HTMX+Alpine doesn't.

### Comparisons

- [ ] **"Azumi vs. HTMX" page** — feature-by-feature: components, scoped CSS, bind:value, @keyed, predictions, form validation, route constants, CSS/HTML/aria validation. HTMX has none of these.
- [ ] **"Azumi vs. Svelte" page** — honest: Svelte wins on ecosystem, transitions, tooling. Azumi wins on: no npm, Rust-native, compile-time safety, scoped CSS validation. Different tradeoffs, different developer.
- [ ] **"Azumi vs. Leptos" page** — Azumi: faster compiles, no Wasm, smaller bundle, scoped CSS. Leptos: fine-grained reactivity, SPA routing, full-stack Rust. When to use which.

### Distribution

- [ ] **Submit to `arewewebyet.org`** — Rust web framework comparison site. Azumi should be listed under "Frontend" or "Live SSR."
- [ ] **Reddit `/r/rust` post** — "Azumi: Svelte-like DX without npm. Components, two-way binding, keyed lists, scoped CSS, compile-time validation. All in `cargo build`."
- [ ] **This Week in Rust** — submit as new framework or major update.
- [ ] **Show HN** — if scope expands beyond Rust audience.

---

## P1 — PREMIUM POLISH (Nice to Have, Lower Priority)

- [ ] **`az-loading` / `az-error` states** — automatic classes during `az-action` fetch. ~1KB.
- [ ] **`debounce=N` on `az-on:input`** — `az-on:input debounce=300 call search`. ~1KB.
- [ ] **Lifecycle hooks** — `az-on:mount`, `az-on:unmount`, `az-on:update`. ~2KB.
- [ ] **Component-based confirm dialogs** — replace `window.confirm()` with styled component. ~2KB.
- [ ] **Spinner component** — animated CSS spinner in prelude. ~20 lines Rust.

---

## DOCUMENTATION

- [ ] **README** — add feature list, quick-start, link to demo.
- [ ] **`docs/guide.md`** — already updated with bind:value and @keyed. Add transitions section when built.
- [ ] **`AGENTS.md`** — already updated. Add transitions reference when built.
- [ ] **Example repo** — standalone minimal app: `git clone → cargo run`. Shows bind:value, @keyed, az-reveal, data-validate, az-action, scoped CSS. One file, ~100 lines.

---

## BUNDLE TRACKER

| State | Size | Features |
|-------|------|----------|
| Before this session | 42KB / 10.5KB | All existing |
| After P0 (now) | 41KB / 10.5KB | +bind:value, +@keyed, +3 validation rules, −ternary DSL |
| After transitions | ~44KB / ~11KB | +fade, slide, scale |

---

## WHEN ENOUGH IS ENOUGH

Azumi is **feature-complete for its niche** with transitions. After that:

- Don't chase Svelte's ecosystem (components, VSCode, community). Wrong game.
- Don't chase Leptos's reactivity (signals, fine-grained). Wrong architecture.
- Don't chase HTMX's adoption (stars, users). Wrong timeline.

**The moat is:** "Components + two-way binding + keyed lists + scoped CSS + compile-time validation + zero npm + single `cargo build`." Nobody else offers this combination. Lean into it.

---

*Tasklist from full audit (audit-tasks.md) and P0 implementation session (2026-05-26).*
