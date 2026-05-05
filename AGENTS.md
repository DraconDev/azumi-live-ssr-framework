# Azumi — AI Agent Guide

## Philosophy

Azumi is an **AI-first, compiler-validated web framework**. Every decision prioritizes what makes AI-generated code correct by default:

- **Named macros over magic syntax**: `json_data!`, `inline_css!`, `inline_script!` are explicit, searchable, and unambiguous
- **Zero escape hatches**: `Raw()`, `from_fn()`, `TrustedHtml` are `#[doc(hidden)]` — AIs should never reach for them
- **Compile-time validation**: Wrong patterns produce clear errors with exact alternatives shown

## Three Golden Rules for AI-Generated Code

1. **NO `Raw()` inside `html!`** — compile error, use the safe macros instead
2. **NO `format!` building web content** inside `html!` — compile error, use interpolation or safe macros
3. **ALWAYS use the safe macros** for structured content injection

## Safe Injection Macros

### `json_data!("VAR_NAME" = &rust_data)`
Safe JSON data injection for JavaScript. Auto-serializes, escapes `</script>`.

```rust
let data = serde_json::json!({"key": "value"});
html! {
    {azumi::json_data!("APP_DATA" = &data)}
}
// Renders: <script>APP_DATA = {"key":"value"};</script>
```

### `inline_css!(css_var)`
Safe CSS injection. Escapes `</style>`.

```rust
html! {
    {azumi::inline_css!(THEME_CSS)}
}
// Renders: <style>THEME_CSS.content</style>
```

### `inline_script!(js_var)`
Safe JavaScript injection. Escapes `</script>`.

```rust
html! {
    {azumi::inline_script!(TRACKING_JS)}
}
// Renders: <script>TRACKING_JS.content</script>
```

## Validation Pipeline (in order)

The `html!` macro runs these validators sequentially, short-circuiting on first error:

1. **CSS validation** — validates CSS property names and syntax
2. **Node order** — enforces Script → Content → Style ordering
3. **Raw usage** — **blocks ALL `Raw()` in html!** (unconditional)
4. **Format detection** — blocks `format!` with HTML/CSS/JS/DOM patterns
5. **Class/ID validation** — classes must be defined in `<style>` blocks
6. **HTML structure rules** — tables, lists, forms, buttons, headings, paragraphs
7. **Attribute validation** — valid HTML5 attributes, `data-*`, `aria-*`, `on:*` events

## Escape Hatches (all `#[doc(hidden)]`)

| Type | Visibility | When to use |
|------|-----------|-------------|
| `from_fn` | `#[doc(hidden)]` | Internal macro expansion only |
| `from_fn_once` | `#[doc(hidden)]` | Internal macro expansion only |
| `Raw<T>` | `#[doc(hidden)]` | Internal framework SEO generation only |
| `TrustedHtml` | `#[doc(hidden)]` | Pre-sanitized HTML from trusted sources (rare edge case) |

AIs should **never** generate code using these. If one is needed, flag it to the human developer.

## Project Structure

```
azumi/
├── macros/
│   ├── src/
│   │   ├── lib.rs                      # html! macro + validation pipeline
│   │   ├── inline_inject.rs            # json_data!, inline_css!, inline_script!
│   │   ├── html_structure_validator.rs # Raw/format! validation + HTML rules
│   │   ├── token_parser.rs             # html! AST parser
│   │   ├── css_validator.rs            # CSS property validation
│   │   ├── component.rs                # #[azumi::component] attribute macro
│   │   ├── page.rs                     # #[azumi::page] attribute macro
│   │   ├── action.rs                   # #[azumi::action] attribute macro
│   │   └── live.rs                     # #[azumi::live] attribute macro
├── src/
│   ├── lib.rs                          # Public API, Component trait, AZUMI_RULES
│   ├── script.rs                       # TrustedHtml, azumi_script(), escape helpers
│   ├── seo.rs                          # SEO helpers (uses Raw internally, outside html!)
│   ├── security.rs                     # HMAC state signing
│   ├── test/mod.rs                     # Test utilities (render, assert_selector)
│   └── context.rs                      # Page metadata context
├── tests/
│   ├── inline_inject_tests.rs          # 22 tests for safe injection macros
│   ├── ai_lint_tests.rs                # AI-first validation tests
│   └── ... (36 other test files)
├── client/
│   ├── azumi.js                        # Client runtime (~3KB)
│   └── idiomorph.js                    # DOM morphing library
├── AI_GUIDE_FOR_WRITING_AZUMI.md       # Comprehensive AI reference
└── AGENTS.md                           # This file — AI agent instructions
```

## Common Anti-Patterns (caught at compile time)

```rust
// ❌ Raw() is always wrong in html!
html! { @{Raw(format!("<div>{}</div>", x))} }
// ✅ Use macros:
html! { {azumi::json_data!("DATA" = &x)} }

// ❌ format! building web content in html!
html! { {format!("window.location = '{}'", url)} }
// ✅ Do formatting outside html!:
let url = format!("window.location = '{}'", url);
html! { {azumi::inline_script!(url)} }

// ❌ format! building CSS in html!
html! { {format!(".btn {{ color: {}; }}", c)} }
// ✅ Use inline_css!:
html! { {azumi::inline_css!(css_var)} }
```

## Tests

```bash
# Run everything
cargo test

# Run macro-specific tests
cargo test -p azumi-macros --lib

# Run injection macro tests
cargo test --test inline_inject_tests
```

## Key Constants

- `azumi::AZUMI_RULES` — array of rule strings for AI reference
- `azumi::AZUMI_AI_HASH` — hash of AI rules for integrity verification
