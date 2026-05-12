# Azumi Follow-up Tasks

Created: 2026-05-11

## High Priority

### 1. Monitor TypeScript File Growth
**Trigger:** Switch to `esbuild` or `swc` when:
- More than 5 TypeScript source files in `libs/chrome/src/ts/`
- Compiled output exceeds 100KB total
- Module imports become complex (cross-file dependencies)

**Current state:** 2 files (~30KB) — `tsc` is fine for now.

**Rationale:** `tsc` is simple and zero-config. Bundlers add node_modules bloat and config complexity. Only upgrade when the pain is real.

---

### 2. Add Rust-Side `az-action` Form Handler Macros
**Problem:** HTML fragment actions require repetitive boilerplate:

```rust
pub async fn handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Form(payload): axum::extract::Form<FormData>,
) -> impl IntoResponse {
    // ... validation ...
    Html(format!(r#"<div class="success">...</div>"#))
}
```

**Idea:** A macro or helper trait that:
- Auto-validates CSRF tokens
- Auto-rate-limits by IP
- Returns `Html` fragments with standard success/error wrappers
- Integrates with `az-target` swap

**Example target API:**
```rust
#[azumi::action]
pub async fn save_profile(
    state: &AppState,
    form: SaveProfileForm,
) -> ActionResult {
    // business logic only
    Ok(html! { <div>"Saved!"</div> })
}
```

---

## Medium Priority

### 3. Document `az-ui` + `az-bind` Patterns
**Problem:** Platform team doesn't use Azumi's reactive features. They default to JS for simple toggles.

**Deliverable:** Add to `AI_INTERACTIVITY_GUIDE.md`:
- Tab panels with `az-ui` + `az-on`
- Live search with `az-bind`
- Modal show/hide with `az-ui`
- Accordion with `az-on="click toggle"`

**Include before/after examples showing JS → Azumi migration.**

---

### 4. Consider CSS Consolidation
**Problem:** Every component inlines its own CSS. No shared variables, no caching.

**Options:**
- A. CSS custom properties (variables) in `<style>` block
- B. Shared CSS file with Azumi class validation
- C. CSS-in-Rust macro (compile-time CSS generation)

**Current approach works** but produces larger HTML and prevents browser caching of styles.

---

## Low Priority

### 5. ✅ Evaluate `ai-rankings.ts` for Rust Simplification (Completed 2026-05-12)
**Verdict:** Keep as TypeScript — real-time slider computation requires client-side JS.
**Documentation:** See `libs/chrome/src/ts/AI_RANKINGS_EVALUATION.md`

---

## Completed Work (Reference)

See compressed session summaries for full context:
- (b1) Azumi refactoring + module extraction
- (b2) Dracon platform JS analysis
- (b5) TypeScript pipeline setup
- (b6) Strategic analysis
- (b7) Framework enhancements
- (b10) Complete JS elimination

**Stats:**
- 15 JS files → 3 files (80% bespoke JS eliminated)
- 1,519 Rust tests passing
- 254 JS tests passing
- All builds green
