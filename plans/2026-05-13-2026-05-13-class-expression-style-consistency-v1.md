# Class Expression Style Inconsistency — Fix Plan

## Problem

The Dracon Platform mixes two styles for CSS class expressions in `html!`:

- **Style A (consistent, ~500 occurrences):** `class={"explicit_string"}` — used in `pages.rs`, `base.rs`, dashboard `layout.rs`, `sidebar.rs`
- **Style B (inconsistent, ~31 occurrences):** `class={bare_ident}` — used in `nav.rs` and `footer.rs`

Both produce the same HTML output, but Style B is **ambiguous** — it looks like a Rust variable reference when the identifier is actually a CSS class name. This creates confusion for code readers and AI tools.

## Files to Fix

### 1. `/home/dracon/Dev/dracon-platform/libs/chrome/src/nav.rs` — 22 bare identifiers
All `class={ident}` patterns where `ident` is a CSS class name, not a Rust variable.

| Current | Target | 
|---------|--------|
| `class={user_menu_wrapper}` | `class={"user_menu_wrapper"}` |
| `class={avatar_toggle}` | `class={"avatar_toggle"}` |
| `class={avatar_placeholder}` | `class={"avatar_placeholder"}` |
| `class={dropdown}` | `class={"dropdown"}` |
| `class={dropdown_header}` | `class={"dropdown_header"}` |
| `class={user_name}` | `class={"user_name"}` |
| `class={dropdown_link}` (×2) | `class={"dropdown_link"}` |
| `class={divider}` | `class={"divider"}` |
| `class={mobile_menu_wrapper}` | `class={"mobile_menu_wrapper"}` |
| `class={mobile_burger}` | `class={"mobile_burger"}` |
| `class={mobile_overlay}` | `class={"mobile_overlay"}` |
| `class={mobile_menu_links}` | `class={"mobile_menu_links"}` |
| `class={mobile_nav_link}` | `class={"mobile_nav_link"}` |
| `class={mobile_divider}` | `class={"mobile_divider"}` |
| `class={mobile_nav_btn}` (×2) | `class={"mobile_nav_btn"}` |
| `class={navbar}` | `class={"navbar"}` |
| `class={navbar_inner}` | `class={"navbar_inner"}` |
| `class={logo_section}` | `class={"logo_section"}` |
| `class={links_section}` | `class={"links_section"}` |
| `class={actions_section}` | `class={"actions_section"}` |
| `class={login_btn}` | `class={"login_btn"}` |

### 2. `/home/dracon/Dev/dracon-platform/libs/chrome/src/footer.rs` — 9 bare identifiers

| Current | Target |
|---------|--------|
| `class={footer_columns}` | `class={"footer_columns"}` |
| `class={col_title}` | `class={"col_title"}` |
| `class={col_link}` | `class={"col_link"}` |
| `class={footer_wrap}` | `class={"footer_wrap"}` |
| `class={container}` | `class={"container"}` |
| `class={grid}` | `class={"grid"}` |
| `class={logo_text}` | `class={"logo_text"}` |
| `class={tagline}` | `class={"tagline"}` |
| `class={copyright}` | `class={"copyright"}` |

## Verification

- `cargo check -p chrome` — verify no compile errors
- `cargo test -p chrome` — verify all 132 tests pass
- Visual inspection of rendered HTML confirms no style changes

## Non-changes

The following patterns are CORRECT as-is and should NOT be changed:
- `class={row_class}` in `base.rs` — `row_class` IS a Rust variable
- `class={"home_page"}` etc. in `pages.rs` — already uses Style A
- `class={"workspace_header"}` etc. in dashboard `layout.rs` — already uses Style A
- Dynamic class expressions like `class={"active"}` — conditional classes are string literals
