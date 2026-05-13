# Attribute Value Inconsistency Fix

## Problem

Azumi's `html!` macro treats bare identifiers in attribute positions differently than expected. `class={class_name}` stringifies the identifier to `class="class_name"`, which is **ambiguous** — it looks like a Rust variable reference when it's actually a CSS class name.

The consistent convention should be `class={"class_name"}` — quoted = literal string, unquoted = variable reference.

## Files to Fix

### 1. `libs/chrome/src/nav.rs` — 22 bare identifiers

| Line | Current | Fixed |
|------|---------|-------|
| 117 | `class={user_menu_wrapper}` | `class={"user_menu_wrapper"}` |
| 118 | `class={avatar_toggle}` | `class={"avatar_toggle"}` |
| 119 | `class={avatar_placeholder}` | `class={"avatar_placeholder"}` |
| 124 | `class={dropdown}` | `class={"dropdown"}` |
| 125 | `class={dropdown_header}` | `class={"dropdown_header"}` |
| 126 | `class={user_name}` | `class={"user_name"}` |
| 129 | `class={dropdown_link}` | `class={"dropdown_link"}` |
| 131 | `class={divider}` | `class={"divider"}` |
| 133 | `class={format!("{} {}", dropdown_link, danger)}` | `class={"dropdown_link danger"}` |
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

### 2. `libs/chrome/src/footer.rs` — 9 bare identifiers

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

## Verification

- [ ] `cargo check -p chrome` passes
- [ ] `cargo test -p chrome` passes
- [ ] `cargo check -p ai-hub-app` passes
- [ ] `cargo check -p dashboard-app` passes

## Files NOT Affected (Already Consistent)

These files already use `class={"string"}`:
- `libs/chrome/src/components/pages.rs`
- `libs/chrome/src/components/base.rs`
- `sites/dashboard-app/src/chrome/layout.rs`
- `sites/dashboard-app/src/chrome/sidebar.rs`
- `sites/ai-hub/src/render.rs`
