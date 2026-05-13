# Fix: Normalize `{bare_ident}` → `{"string"}` for CSS class names

## Problem

The Dracon Platform's `html!` components inconsistently use both `{bare_ident}` and `{"string"}` syntax for CSS class names in `class={}` attributes:

- **`pages.rs`**, **`base.rs`**, **`layout.rs`**, **`sidebar.rs`** — already use `class={"string"}` ✅
- **`nav.rs`** — uses `class={bare_ident}` for ~23 CSS class names ❌
- **`footer.rs`** — uses `class={bare_ident}` for ~9 CSS class names ❌

The `{bare_ident}` pattern works because Azumi's codegen fallback path stringifies the token stream, but it's fragile and confusing — it looks like a Rust variable reference when it's actually a CSS class name.

## Scope

### nav.rs Fixes (23 occurrences)

| Line | Current | Fixed |
|------|---------|-------|
| 117 | `class={user_menu_wrapper}` | `class={"user_menu_wrapper"}` |
| 118 | `class={avatar_toggle}` | `class={"avatar_toggle"}` |
| 119 | `class={avatar_placeholder}` | `class={"avatar_placeholder"}` |
| 124 | `class={dropdown}` | `class={"dropdown"}` |
| 125 | `class={dropdown_header}` | `class={"dropdown_header"}` |
| 126 | `class={user_name}` | `class={"user_name"}` |
| 129 | `class={dropdown_link}` | `class={"dropdown_link"}` |
| 133 | `class={format!("{} {}", dropdown_link, danger)}` | `class={"dropdown_link danger"}` (static — always applied) |
| 238 | `class={mobile_menu_wrapper}` | `class={"mobile_menu_wrapper"}` |
| 239 | `class={mobile_burger}` | `class={"mobile_burger"}` |
| 247 | `class={mobile_overlay}` | `class={"mobile_overlay"}` |
| 248 | `class={mobile_menu_links}` | `class={"mobile_menu_links"}` |
| 250 | `class={mobile_nav_link}` | `class={"mobile_nav_link"}` |
| 252 | `class={mobile_divider}` | `class={"mobile_divider"}` |
| 254 | `class={mobile_nav_btn}` | `class={"mobile_nav_btn"}` |
| 256 | `class={mobile_nav_btn}` | `class={"mobile_nav_btn"}` |
| 358 | `class={navbar}` | `class={"navbar"}` |
| 360 | `class={navbar_inner}` | `class={"navbar_inner"}` |
| 361 | `class={logo_section}` | `class={"logo_section"}` |
| 369 | `class={links_section}` | `class={"links_section"}` |
| 375 | `class={actions_section}` | `class={"actions_section"}` |
| 383 | `class={login_btn}` | `class={"login_btn"}` |

### nav.rs Format Fixes (2 occurrences) — Move format! outside html!

| Line | Current | Fixed |
|------|---------|-------|
| 77 | `class={if gradient { format!("{} {}", logo, has_gradient) } else { logo.to_string() }}` | Pre-format before html!: `let logo_class = if gradient { "logo has_gradient" } else { "logo" };` then `class={logo_class}` |
| 133 | `class={format!("{} {}", dropdown_link, danger)}` | `class={"dropdown_link danger"}` (always static) |

### footer.rs Fixes (9 occurrences)

| Line | Current | Fixed |
|------|---------|-------|
| 62 | `class={footer_columns}` | `class={"footer_columns"}` |
| 65 | `class={col_title}` | `class={"col_title"}` |
| 68 | `class={col_link}` | `class={"col_link"}` |
| 96 | `class={footer_wrap}` | `class={"footer_wrap"}` |
| 97 | `class={container}` | `class={"container"}` |
| 98 | `class={grid}` | `class={"grid"}` |
| 100 | `class={logo_text}` | `class={"logo_text"}` |
| 101 | `class={tagline}` | `class={"tagline"}` |
| 102 | `class={copyright}` | `class={"copyright"}` |

## Exclusions (legitimate dynamic class expressions)

- **sidebar.rs:15,21,27,33** — `class={format!("dashboard_sidebar_link{}", ...)}` — These are **legitimate dynamic expressions** that build class strings WITH conditional logic (active/inactive state). `format!` is used properly here for string concatenation, not HTML/JS building. Could optionally pre-format for cleanliness, but not required.

## Files Already Consistent (no changes needed)

- `pages.rs` — Uses `class={"string"}` ✅
- `base.rs` — Uses `class={"string"}` (and `class={row_class}` for actual variable) ✅
- `dashboard/src/chrome/layout.rs` — Uses `class={"string"}` consistently ✅
- `dashboard/src/chrome/sidebar.rs` — Uses `class={"string"}` for static, `format!` for dynamic active class ✅

## Verification

After applying all fixes:
```bash
cd /home/dracon/Dev/dracon-platform
cargo check -p chrome 2>&1 | tail -5
cargo test -p chrome 2>&1 | grep "test result:"
```
No regressions expected — all changes are syntactically identical at the HTML output level.

## Future Guidance

When adding new `class={}` attributes in `html!`:
- **Static class names**: Always use `class={"class_name"}` with quotes
- **Dynamic classes**: Use a pre-formatted Rust variable, then `class={my_var}`
- **Conditional classes**: Pre-format before the `html!` block
