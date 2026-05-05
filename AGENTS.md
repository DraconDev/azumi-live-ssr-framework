# Azumi — AI Agent Guide

## Philosophy

Azumi is an **AI-first, compiler-validated web framework**. Every decision prioritizes what makes AI-generated code correct by default:

- **Safe macros for real work, bare tags for injection**: `json_data!` handles serde + variable naming; `<style>{var}</style>` and `<script>{var}</script>` auto-escape content
- **Zero escape hatches**: `Raw()`, `from_fn()`, `TrustedHtml` are `#[doc(hidden)]` — AIs should never reach for them
- **Compile-time validation**: Wrong patterns produce clear errors with exact alternatives shown

## Three Golden Rules for AI-Generated Code

1. **NO `Raw()` inside `html!`** — compile error, use the safe patterns instead
2. **NO `format!` building web content** inside `html!` — compile error, use interpolation or safe macros
3. **ALWAYS use `<style>{var}</style>` / `<script>{var}</script>` for CSS/JS injection, and `json_data!` for JSON data**

## Safe Injection Patterns

### `<style>{css_var}</style>`
Safe CSS injection. Auto-escapes `</style>` (case-insensitive) inside `html!`.

```rust
html! {
    <style>{THEME_CSS}</style>
}
// Renders: <style>.my_class { color: red; }</style>
```

### `<script>{js_var}</script>`
Safe JavaScript injection. Auto-escapes `</script>` (case-insensitive) inside `html!`.

```rust
html! {
    <script>{TRACKING_JS}</script>
}
// Renders: <script>console.log('track');</script>
```

### `json_data!("VAR_NAME" = &rust_data)`
Safe JSON data injection for JavaScript. Auto-serializes, escapes `</script>`.

```rust
let data = serde_json::json!({"key": "value"});
html! {
    {azumi::json_data!("APP_DATA" = &data)}
}
// Renders: <script>APP_DATA = {"key":"value"};</script>
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
│   │   ├── inline_inject.rs            # json_data! macro only
│   │   ├── html_structure_validator.rs # Raw/format! validation + HTML rules
│   │   ├── token_parser.rs             # html! AST parser (handles <style>{var}</style>)
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
│   ├── inline_inject_tests.rs          # json_data! + auto-escape tests
│   ├── security_xss_tests.rs           # XSS breakout prevention tests
│   ├── integration_inject_tests.rs     # Integration tests for injection patterns
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
// ✅ Use auto-escaping or json_data!:
html! { {azumi::json_data!("DATA" = &x)} }

// ❌ format! building web content in html!
html! { {format!("window.location = '{}'", url)} }
// ✅ Do formatting outside html!:
let url = format!("window.location = '{}'", url);
html! { <script>{url}</script> }

// ❌ format! building CSS in html!
html! { {format!(".btn {{ color: {}; }}", c)} }
// ✅ Use <style> tag with auto-escape:
html! { <style>{css_var}</style> }
```

## Notes

- `<style>{var}</style>` and `<script>{var}</script>` auto-escape `</style>` and `</script>` sequences (case-insensitive). You do NOT need a macro for this.
- `json_data!("VAR" = &data)` is the only safe injection macro. It does real work: serde serialization + variable naming + `<script>` tag wrapping.
- For external JS/CSS, use `<script src="...">` and `<style src="...">`.

## Tests

```bash
# Run everything
cargo test

# Run macro-specific tests
cargo test -p azumi-macros --lib

# Run injection tests
cargo test --test inline_inject_tests
cargo test --test security_xss_tests
cargo test --test integration_inject_tests
```

## Key Constants

- `azumi::AZUMI_RULES` — array of rule strings for AI reference
- `azumi::AZUMI_AI_HASH` — hash of AI rules for integrity verification
