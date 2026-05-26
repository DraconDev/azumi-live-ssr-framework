# Azumi Audit Tasks

> Last verified: 2026-05-26 — all claims verified against source.

---

## 1. INTERACTIVITY GAP ANALYSIS

### What Azumi Currently Has (Verified)

| Feature | Status | Implementation |
|---------|--------|---------------|
| Event delegation | ✅ | `az-on:click`, `az-on:submit`, `az-on:change`, `az-on:input` |
| Live state | ✅ | `#[azumi::live]` structs, HMAC-signed `az-scope` attribute |
| Optimistic predictions | ✅ | `data-predict` toggle/increment/decrement + `az-predictions` auto-detection |
| Form validation | ✅ | `data-validate="required,email,min-length:8"` |
| DOM morphing | ✅ | Idiomorph (3KB gzipped) via `Idiomorph.morph()` |
| State bindings | ✅ | `az-bind:text`, `az-bind:class:*` (predicate-based), `data-bind` (legacy) |
| Client-side state | ✅ | `az-ui` attribute for local state, `az-on:scroll-top` |
| Scroll reveal | ✅ | `az-reveal` with IntersectionObserver |
| Hot reload | ✅ | WebSocket + polling fallback (dev only) |
| `@for` / `@if` / `@match` / `@let` | ✅ | Compile-time template macros |

### What Azumi is Missing (Verified — NOT in source)

| Missing Feature | Svelte/Leptos Equivalent | Verified Absence |
|----------------|------------------------|-----------------|
| **Two-way form binding** | `bind:value`, `bind:checked` | ✅ Confirmed absent — no `bind:value` in macros/src/ or src/client.min.js |
| **Keyed each updates** | `{#each items as item (id)}` | ✅ Confirmed absent — no `@keyed` in token_parser.rs |
| **Transitions/animations** | `transition:`, `in:`, `out:` | ✅ Confirmed absent — no `transition`/`animation` in client.min.js |
| **Reactive declarations** | `$:` auto-compute | ✅ Confirmed absent — no `$:` in any Rust source |
| **Lifecycle hooks** | `onMount`, `onUnmount`, `beforeUpdate` | ✅ Confirmed absent — no `onMount`/`onDestroy` in any source |
| **Slot/portal rendering** | `<slot>`, portals | ✅ Confirmed absent — no `slot`/`portal`/`teleport` in client.min.js |
| **`data-bind:value`** | Two-way input sync | ✅ Confirmed absent — only `data-bind` (text content) and `az-bind:text` exist |

---

## 2. CLIENT.JS SIZE ANALYSIS

### Verified Breakdown (from src/client.min.js — 47,047 bytes uncompressed)

| Feature | Lines | Approx Size | Verified? |
|---------|-------|-------------|-----------|
| **Idiomorph** | ~250 | ~3KB gzipped | ✅ Bundled inline |
| **Predicate evaluation** (`evaluatePredicate`, `evaluateExpression`, `parseTernary`, `findTernaryIndex`, `findOperatorIndex`) | ~300 | ~20KB | ✅ Verified in client.min.js |
| **Optimistic predictions** (`executePrediction`, `applyPrediction`, `rollbackPrediction`) | ~150 | ~10KB | ✅ Verified in client.min.js |
| **Event delegation** (`delegate`, `handleEvent`, `handleFormSubmit`) | ~120 | ~8KB | ✅ Verified in client.min.js |
| **State bindings** (`updateBindings`, `readState`) | ~100 | ~6KB | ✅ Verified in client.min.js |
| **Hot reload** (`connectHotReload`, `pollForReload`, `handleStyleUpdate`) | ~100 | ~6KB | ✅ Verified in client.min.js |
| **Form validation** (`validateFormField`, `isValidEmail`, `isValidUrl`) | ~80 | ~5KB | ✅ Verified in client.min.js |
| **Azumi class** (constructor, init, reveal, log/warn/error) | ~80 | ~5KB | ✅ Verified in client.min.js |

**Total: ~47KB uncompressed.** Hot reload is dev-only (`#[cfg(feature = "devtools")]` in Rust, but client.min.js includes it unconditionally — this is a bug).

---

## 3. COMPETITIVE SIZE COMPARISON (Verified)

| Framework | Gzipped Size | Source |
|-----------|-------------|--------|
| htmx | ~16 KB | bundlephobia.com/package/htmx |
| Alpine.js | ~10 KB gzipped | bundlephobia.com/package/alpinejs |
| Stimulus | ~5 KB gzipped | bundlephobia.com/package/@hotwired/stimulus |
| Phx.LiveView | ~32 KB gzipped | phoenix.js (6KB) + phoenix_live_view.js (26KB) |
| Idiomorph | ~3 KB gzipped | bundlephobia.com/package/idiomorph |
| Azumi (you) | ~42 KB (uncompressed) | src/client.min.js = 47,047 bytes |

**Azumi is ~2.5x larger than htmx.** The gap is the predicate DSL (~20KB) + optimistic predictions (~10KB).

---

## 4. TODO CHECKLIST

### P0 — Ship-Blocking (Fix First)

- [ ] **Two-way form binding** (`bind:value`) — Add `bind:value` attribute to `<input>` elements that syncs `input.value` → state with 200ms debounce
  - Cost: ~2KB client-side
  - Server: No changes needed (server still receives form data)
  - Priority: Highest — fixes the biggest UX gap

- [ ] **Keyed each updates** (`@keyed item.id { ... }`) — Add `@keyed` block to `@for` loops that passes key selector to Idiomorph
  - Cost: ~3KB client-side
  - Server: No changes needed (just adds `key` attribute to rendered items)
  - Priority: High — fixes the "full list re-render" problem

### P1 — Nice-to-Have (Low Cost)

- [ ] **Reactive declarations** (`state.name @react => state.greeting = ...`) — Add dependency graph in WeakMap, re-run declarations when dependencies change
  - Cost: ~3KB client-side
  - Server: No changes needed
  - Priority: Medium — improves DX but not essential

- [ ] **Lifecycle hooks** (`az-on:mount`, `az-on:destroy`) — Fire callbacks after morphing completes
  - Cost: ~2KB client-side
  - Server: No changes needed
  - Priority: Medium — useful for side effects

### P2 — Could Have (If You Have Time)

- [ ] **Transitions/animations** — Add CSS transition support via `transition:` directive
  - Cost: ~5KB client-side
  - Priority: Low — conflicts with "no custom JS" philosophy

- [ ] **Media query bindings** — `@media (min-width: ...)` reactive state
  - Cost: ~1KB client-side
  - Priority: Low — niche use case

### Skip — Don't Do It

- [ ] **Fine-grained reactivity** — Conflicts with DOM morphing architecture
- [ ] **Store system** — `#[azumi::live]` already solves cross-component state

---

## 5. CLIENT.JS BUGS TO FIX

- [ ] **Hot reload not feature-gated in client.min.js** — The WebSocket hot reload code is included unconditionally in `src/client.min.js`, even though `#[cfg(feature = "devtools")]` gates it in Rust. This means production builds ship hot reload code.
  - Fix: Add `#[cfg(feature = "devtools")]` guards around `connectHotReload()`, `pollForReload()`, and `handleStyleUpdate()` in client.min.js, OR strip it during minification.

- [ ] **Duplicate client JS files** — Both `client/azumi.js` (47,047 bytes) and `src/client.min.js` (42,387 bytes) exist. Ensure minification pipeline is documented and automated.

---

## 6. ARCHITECTURE NOTES

### What Actually Exists (Verified)

1. **`az-on:click` / `az-on:submit` / `az-on:change` / `az-on:input`** — Event delegation for server actions
2. **`az-bind:text`** — Text content binding via expression evaluation
3. **`az-bind:class:*`** — Class toggling via predicate evaluation (e.g., `az-bind:class:active="state.active"`)
4. **`data-bind`** — Legacy text content binding
5. **`az-ui`** — Client-side state attribute (separate from server-signed `az-scope`)
6. **`az-scope`** — HMAC-signed server state (JSON + signature)
7. **`az-predictions`** — Auto-detected optimistic predictions from `#[azumi::live_impl]`
8. **`data-validate`** — Client-side form validation (required, email, min/max-length, url)
9. **`az-reveal`** — Scroll reveal via IntersectionObserver

### What's Actually Missing (Verified)

1. **`bind:value`** — Two-way input binding (input.value → state, debounced POST)
2. **`@keyed`** — Keyed list updates for efficient DOM morphing
3. **Transitions/animations** — No CSS transition support in client runtime
4. **`$:` reactive declarations** — No auto-compute dependencies
5. **`onMount`/`onUnmount`** — No lifecycle hooks
6. **`slot`/`portal`** — No component composition beyond children prop

---

## 7. IMMEDIATE ACTION ITEMS

1. **Fix hot reload feature gate** — Remove `connectHotReload()` from production builds
2. **Add `bind:value`** — Two-way form binding (P0, ~2KB)
3. **Add `@keyed`** — Keyed each updates (P0, ~3KB)
4. **Document client.min.js pipeline** — How is `src/client.min.js` generated from `client/azumi.js`?
5. **Audit `docs/archive/`** — 10 files, likely stale. Consider pruning.
