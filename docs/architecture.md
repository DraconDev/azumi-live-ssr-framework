# Azumi Architecture Guide

## Crate Structure

```
azumi/                      # Main crate — user-facing API
├── src/
│   ├── lib.rs              # Component trait, re-exports, render helpers
│   ├── script.rs           # Escape functions, AzumiScript, TrustedHtml
│   ├── security.rs         # HMAC state signing, verification, VerifyStateError
│   ├── context.rs          # Page metadata (thread-local), request path
│   ├── seo.rs              # SEO helpers, OpenGraph, Twitter Cards
│   ├── action.rs           # Action trait, inventory-based registry
│   ├── hot_reload.rs       # Dev server hot reload (feature-gated)
│   ├── devtools.rs         # Dev tools router (feature-gated)
│   └── test/
│       └── mod.rs          # Test utilities (render, assert_selector)
│
├── macros/                 # Proc macro crate — compile-time codegen
│   ├── src/
│   │   ├── lib.rs          # Proc macro entry points + validation pipeline orchestration
│   │   ├── token_parser.rs # HTML tokenizer (syn-based Parse impl)
│   │   ├── context.rs      # Context/GenerationContext types
│   │   ├── codegen.rs      # generate_body_with_context — render instruction generation
│   │   ├── validators.rs   # validate_nodes — attribute/HTML/accessibility validation
│   │   ├── style_processing.rs # process_styles + collect_all_styles — CSS hoisting/extraction
│   │   ├── html_structure_validator.rs
│   │   ├── accessibility_validator.rs
│   │   ├── css_validator.rs
│   │   ├── style.rs        # CSS DSL parser (style! block)
│   │   ├── css.rs          # CSS property/color utilities
│   │   ├── inline_inject.rs# json_data! macro only
│   │   ├── component.rs    # #[azumi::component] attribute macro
│   │   ├── page.rs         # #[azumi::page] attribute macro
│   │   ├── action.rs       # #[azumi::action] attribute macro
│   │   ├── live.rs         # #[azumi::live] + #[azumi::live_impl]
│   │   ├── head.rs         # head! macro
│   │   ├── schema.rs       # Schema derive macro (feature-gated)
│   │   └── asset_rewriter.rs
│   └── Cargo.toml
│
├── benches/                # Criterion benchmarks
│   ├── escape.rs           # Escape function performance
│   ├── render.rs           # html! render performance
│   └── scope_css.rs        # CSS scoping performance
│
├── tests/                  # Integration tests (~40 files)
│   ├── inline_inject_tests.rs    # json_data! + auto-escape tests
│   ├── security_xss_tests.rs     # XSS breakout prevention
│   ├── integration_inject_tests.rs
│   └── ui/                      # Compile-fail UI tests
│
├── client/                 # Browser-side runtime
│   ├── azumi.js            # Azumi coordinator (~3KB)
│   └── idiomorph.js        # DOM morphing library
│
├── demo/                   # Workspace member — tutorial/example app
│
└── docs/                   # Documentation
    ├── architecture.md     # This file
    └── migration/          # Per-version migration guides
```

## Validation Pipeline

When `html!` macro is invoked, the following validators run sequentially:

```
Token Parser  →  CSS Validation  →  Node Order  →  Raw Usage Check
  ↓
Format Detection  →  Class/ID Validation  →  HTML Structure Rules
  ↓
Attribute Validation  →  Accessibility Validation  →  Code Generation
```

1. **Token Parser** — Parses HTML into `Vec<Node>` (Element, Text, Expression, Block, Fragment)
2. **CSS Validation** — Validates CSS property names and values in `<style>` blocks
3. **Node Order** — Enforces Script → Content → Style ordering
4. **Raw Usage** — Blocks ALL `Raw()` calls inside `html!`
5. **Format Detection** — Blocks `format!()` building HTML/CSS/JS/DOM content inside `html!`
6. **Class/ID Validation** — Classes must be defined in `<style>` blocks as snake_case
7. **HTML Structure Rules** — Tables, lists, forms, headings, paragraphs nesting rules
8. **Attribute Validation** — Valid HTML5 attributes, `data-*`, `aria-*`, `on:*` events
9. **Code Generation** (`codegen.rs`) — Generates Rust expressions that construct HTML at runtime

## Module Responsibilities

| Module | Lines | Responsibility |
|--------|-------|---------------|
| `lib.rs` | ~360 | Proc macro entry points + validation pipeline orchestration |
| `codegen.rs` | ~440 | `generate_body_with_context` — recursive HTML render instruction generation |
| `validators.rs` | ~290 | `validate_nodes` — attribute/class/HTML/accessibility validation |
| `style_processing.rs` | ~160 | `process_styles` + `collect_all_styles` — CSS hoisting and extraction |
| `token_parser.rs` | ~900 | HTML tokenizer, syn-based Parse implementations |
| `html_structure_validator.rs` | ~500 | HTML nesting rules, table/list/form validation |
| `accessibility_validator.rs` | ~200 | ARIA roles, img alt, button content, iframe titles |
| `style.rs` | ~1,200 | CSS DSL parser, property validation, minification |

### `validate_nodes` Parameters

The `validators::validate_nodes` function accepts `valid_classes` and `valid_ids` — sets of class and ID names that are defined in `<style>` blocks within the current component scope. These sets are built by extracting selectors from scoped CSS via `crate::css::extract_selectors()`. This naming convention (prefixed with `valid_`) signals that these are allowlists for validation, not arbitrary data structures.

## Rendering Flow

```
html! { <div class={my_class}>"Hello"</div> }
              ↓
      Macro expansion (compile-time)
              ↓
  from_fn_once(move |f| {
      // Style hoisting
      let style_bindings = ...;
      // Bind validation checks
      const _: () = { ... };
      // HTML construction
      write!(f, "<div class=\"")?;
      my_class.render_azumi(f)?;
      write!(f, "\">")?;
      write!(f, "{}", Escaped("Hello"))?;
      write!(f, "</div>")?;
  })
              ↓
      Runtime rendering
              ↓
      <div class="my_class">Hello</div>
```

## Auto-Escaping for `<style>` and `<script>`

When the html! macro encounters expression children inside `<style>` or `<script>` tags:

```
<script>{JS_CONTENT}</script>
       ↓
Context::Script mode activates
       ↓
Expression renders via azumi::escape_script_content()
       ↓
Output: <script>console.log('hello');</script>

<style>{CSS_CONTENT}</style>
       ↓
Context::Style mode activates
       ↓
Expression renders via azumi::escape_style_content()
       ↓
Output: <style>.btn { color: red; }</style>
```

## Key Design Decisions

- **AI-first, compiler-validated**: Every decision prioritizes what makes AI-generated code correct by default
- **Zero escape hatches**: `Raw()`, `from_fn()`, `TrustedHtml` are `#[doc(hidden)]` — AIs should never reach for them
- **Compile-time validation**: Wrong patterns produce clear errors with exact alternatives shown
- **No `no_std`** (yet): Core rendering depends on `std::fmt`
- **Axum-only**: Framework is tightly coupled to Axum for routing and WebSocket support
