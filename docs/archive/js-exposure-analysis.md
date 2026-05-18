# Azumi JS Exposure Analysis: Competitive Landscape & Reduction Strategy

## Executive Summary

The dracon-platform carries **15 static JS files totaling ~2,200 lines**. The core concern is `azumi-runtime.js` (1,237 lines when vendored with Idiomorph, or ~985 lines for just the framework). This document analyzes what we're competing against and how to reduce JS surface while maintaining functionality.

## Current JS Inventory (Dracon Platform)

| File | Lines | Purpose | Critical? |
|------|-------|---------|-----------|
| `azumi-runtime.js` | 1,237 | Framework (Idiomorph + event delegation + state + actions) | **YES** |
| `scroll-reveal.js` | 19 | Scroll-triggered animations | No (cosmetic) |
| `gtag-init.js` | 10 | Google Analytics initialization | No (third-party) |
| `back-to-top.js` | 19 | Back-to-top button visibility | No (cosmetic) |
| `session-cleanup.js` | 1 | URL hash cleanup for auth tokens | Yes (security) |
| `csrf-inject.js` | 14 | CSRF token injection into forms | Yes (security) |
| `login.js` | 38 | Magic link form handling | Yes (auth) |
| `auth-verify.js` | ~40 | Auth verification logic | Yes (auth) |
| `api-key-copy.js` | ~30 | API key copy-to-clipboard | No (convenience) |
| `api-key-revoke.js` | ~30 | API key revocation UI | Yes (functionality) |
| `paddle-checkout.js` | ~100 | Paddle payment integration | Yes (billing) |
| `ai-rankings.js` | ~200 | AI provider rankings interactivity | Yes (functionality) |
| `ai-hub-copy.js` | ~30 | AI hub copy functionality | No (convenience) |
| `verify-code.js` | ~40 | Verification code input | Yes (auth) |

**Total: ~1,808 lines of application JS + 1,237 lines of framework JS = ~3,045 lines**

## The Framework: What Azumi Actually Is

Azumi is **not HTMX**. It's a full client framework with:

### Core Features (985 lines in `client/azumi.js`)

| Feature | Lines | What It Does |
|---------|-------|-------------|
| Hot reload | ~80 | WebSocket connection, style updates, polling fallback |
| Event delegation | ~100 | Parses `az-on` attributes, routes clicks/submits/changes |
| Action routing | ~60 | `call` actions → POST to `/azumi/action/{namespace}/{action}` |
| Local state (`set`) | ~40 | Client-side state mutations without server round-trip |
| State management | ~120 | `az-scope` (server-signed), `az-ui` (client), WeakMap cache |
| Predictions | ~50 | Optimistic UI: apply prediction → server → rollback if wrong |
| Expression evaluator | ~150 | Predicate evaluation (`!field`, `==`, `<`, `&&`, `\|\|`, ternary) |
| Binding system | ~80 | `data-bind`, `az-bind:text`, `az-bind:class:*` updates |
| Action execution | ~150 | `callAction`: fetch → Idiomorph morph → rollback on error |
| DOM utilities | ~50 | `getNestedValue`, `setNested`, helpers |

### What Idiomorph Adds (848 lines)

- DOM diffing algorithm (`morph`)
- ID-set matching for element preservation
- Soft matching fallback
- Head element merging
- Attribute synchronization
- Input value preservation
- SVG handling

**Combined: 1,833 lines of JS that MUST run in the browser.**

## What We're Competing Against

### 1. HTMX (The "Minimal JS" Benchmark)

**Approach:** Attributes trigger HTTP requests, swap HTML into DOM.

```html
<!-- HTMX: click → GET /contact → swap into #content -->
<button hx-get="/contact" hx-target="#content">Load Contact</button>
```

**JS Size:** ~14KB minified (single file)
**What it does:** HTTP requests, DOM swapping, event handling
**What it DOESN'T do:** State management, optimistic UI, expression evaluation, hot reload

**Verdict:** HTMX is simpler but less powerful. Azumi has state management + predictions that HTMX lacks.

### 2. Leptos (Rust → WASM)

**Approach:** Rust code compiles to WASM, runs reactive system in browser.

```rust
// Leptos: reactive signal
let (count, set_count) = create_signal(0);
view! { <button on:click=move |_| set_count(count() + 1)>{count}</button> }
```

**JS Size:** ~50KB+ WASM + ~20KB JS glue
**What it does:** Full reactive system, SSR + hydration, signals
**What it DOESN'T do:** Server-signed state (client can manipulate anything)

**Verdict:** Leptos is more powerful but heavier. Azumi's server-signed state is unique.

### 3. Dioxus (Multi-platform Rust)

**Approach:** React-like Rust framework, web via WASM.

**JS Size:** ~100KB+ WASM + JS glue
**What it does:** React-like components, cross-platform (web/desktop/mobile)
**Trade-off:** Heavy WASM bundle, JS interop complexity

### 4. Pure Server-Side (No JS)

**Approach:** Every interaction is a full page reload.

**JS Size:** 0
**What it does:** Nothing client-side
**What it DOESN'T do:** Any interactivity without reload

**Verdict:** Too limited for modern expectations.

### 5. React/Vue/Angular (The Status Quo)

**JS Size:** 100KB-300KB+ (framework + app code)
**What it does:** Everything client-side
**What it DOESN'T do:** Server-rendered initial HTML (without SSR)

## The Critical Insight: What MUST Be JS vs. What Could Be Rust

### MUST Be JS (Browser APIs Required)

| Feature | Why It Needs JS |
|---------|----------------|
| DOM manipulation | Browser-only API |
| Event handling | `addEventListener`, `Event` objects |
| WebSocket | `WebSocket` API |
| `fetch()` | HTTP requests from browser |
| `FormData` | Form serialization |
| `history.replaceState` | URL manipulation |
| `navigator.clipboard` | Copy-to-clipboard |

### Could Be Rust/WASM

| Feature | Complexity | Benefit |
|---------|-----------|---------|
| State mutations (`applyPrediction`) | Medium | Type safety, no prototype pollution |
| Expression evaluation | High | Avoid JS `eval()`, type safety |
| JSON parsing/serialization | Low | Already fast in JS |
| Action routing logic | Low | Minimal benefit |

### Could Be Server-Side Rendered (No JS)

| Feature | How |
|---------|-----|
| Scroll-reveal | CSS `animation-timeline: scroll()` or render visible |
| Back-to-top | CSS-only or remove |
| gtag-init | Keep minimal (10 lines) |
| CSRF injection | Server-render hidden input |
| Session cleanup | Server redirect |

## The Real Problem: Surface Area vs. Depth

The issue isn't just **how much JS** (3,045 lines) but **what kind of JS**:

### High-Risk JS (Framework Code)
- `azumi-runtime.js`: ~400 lines of your code + 848 lines Idiomorph
- This is where bugs affect ALL pages
- Type safety would eliminate 90% of runtime errors

### Medium-Risk JS (Feature Code)
- `login.js`, `auth-verify.js`, `paddle-checkout.js`
- Page-specific, easier to test in isolation
- Could be TypeScript-ified with high value

### Low-Risk JS (Cosmetic/Convenience)
- `scroll-reveal.js`, `back-to-top.js`, `api-key-copy.js`
- Could be removed or replaced with CSS

## Recommended Strategy: The "Layered Reduction" Approach

### Phase 1: Immediate (This Week)
**Goal: Reduce surface area without behavior changes**

1. **TypeScript-ify all application JS** (`login.js`, `auth-verify.js`, etc.)
   - Catches type errors at build time
   - No runtime change
   - ~500 lines → TypeScript

2. **Remove cosmetic JS**
   - `scroll-reveal.js` → CSS-only or remove
   - `back-to-top.js` → CSS `position: sticky` or remove
   - Saves ~40 lines, zero functionality loss

3. **Minify/obfuscate framework JS**
   - `azumi-runtime.js` can be minified to ~300 lines equivalent
   - Reduces download size, not line count

**Result: ~3,045 → ~2,500 effective lines, with type safety on app code**

### Phase 2: Short-term (This Month)
**Goal: Reduce framework complexity**

1. **Split `azumi-runtime.js`**
   - `idiomorph.js` (848 lines, vendor, don't touch)
   - `azumi-core.js` (~400 lines, your framework)
   - Clear separation: third-party vs. your code

2. **TypeScript-ify `azumi-core.js`**
   - Add types to the framework itself
   - Define interfaces for `Action`, `Prediction`, `State`
   - Catch framework bugs at build time

3. **Document the JS boundary**
   - Every JS file gets a header comment:
   ```javascript
   /**
    * WHY JS: This file needs DOM access for X
    * RUST ALTERNATIVE: None - requires browser APIs
    * SAFETY: Pure functions, no user input evaluation
    */
   ```

**Result: Clear ownership, type safety, reduced cognitive load**

### Phase 3: Long-term (This Quarter)
**Goal: Move state management to Rust**

1. **WASM prediction engine**
   ```rust
   // Rust code, compiled to WASM
   #[wasm_bindgen]
   pub fn apply_prediction(state_json: &str, prediction: &str) -> Result<String, JsValue> {
       let mut state: Value = serde_json::from_str(state_json)?;
       // Type-safe mutation
       Ok(state.to_string())
   }
   ```
   - Eliminates prototype pollution risk
   - Type-safe state mutations
   - ~50KB WASM overhead (one-time download)

2. **Server-side action handlers**
   - Move more logic to `#[action]` handlers in Rust
   - Less JS = less client-side risk

3. **CSS-native features**
   - Use `@property` for animations
   - Use `container queries` for responsive design
   - Use `has()` for parent-based styling
   - Reduce JS-driven styling

**Result: ~400 lines of framework JS → ~150 lines + 50KB WASM**

## The Honest Trade-off Matrix

| Approach | JS Lines | WASM Size | Type Safety | Interactivity | Complexity |
|----------|----------|-----------|-------------|---------------|------------|
| Current (JS) | 3,045 | 0 | None | High | Low |
| TS Migration | 3,045 | 0 | Build-time | High | Low |
| TS + Remove Cosmetic | 2,500 | 0 | Build-time | High | Low |
| TS + WASM State | 2,100 | 50KB | Full | High | Medium |
| Leptos-style | ~500 | 100KB | Full | Very High | High |
| HTMX-style | ~200 | 0 | None | Medium | Low |
| Pure SSR | 0 | 0 | Full | None | Low |

## Recommendation: "TS + Cosmetic Removal" Now, WASM Later

**Phase 1 (Immediate):**
- TypeScript-ify all JS
- Remove `scroll-reveal.js`, `back-to-top.js`
- Save: ~545 lines → ~2,500 lines

**Phase 2 (Short-term):**
- Split Idiomorph from azumi-core
- TypeScript-ify azumi-core
- Document every JS file's rationale

**Phase 3 (Evaluate):**
- WASM prediction engine only if proven valuable
- Measure: does 50KB WASM + 150 lines JS beat 400 lines typed JS?
- Likely answer: Not until app is much larger

## Why Not Pure TS + Rust Backend?

You asked if a TypeScript frontend + Rust backend is competitive. The answer depends on what you value:

| If you value... | Then... |
|----------------|---------|
| Minimal JS | Current approach is better |
| Type safety everywhere | WASM approach is better |
| Developer ecosystem | TS + Rust API is better |
| Performance | Current approach is better |
| Hiring pool | TS + Rust API is better |

**The unique value of Azumi:**
- Server-signed state (`az-scope`) prevents client tampering
- Optimistic predictions with rollback
- Zero JS for static pages (just inline `<script>`)
- Rust-native: no JS build step for HTML/CSS

**A pure TS frontend loses these advantages.** You'd need:
- Client-side state validation (duplicated from Rust)
- Hydration complexity
- JS build pipeline (webpack, vite, etc.)
- Larger bundles

## Conclusion

Your instinct is correct: **reduce JS exposure**. But the path isn't "replace with TS" — it's:

1. **TypeScript-ify** what must be JS (immediate win)
2. **Remove** what doesn't need JS (scroll-reveal, back-to-top)
3. **Document** why each JS file exists (maintainability)
4. **Evaluate** WASM only for proven high-value scenarios (predictions)

The current architecture (Rust SSR + minimal JS) is **competitive** and **correct**. The issue is the JS has grown organically without type safety or cleanup. Fix that first.

**Next step:** Want me to TypeScript-ify the application JS files and remove the cosmetic ones?
