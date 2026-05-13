# Plan: Normalize Attribute Expressions in `html!` Macros

## Objective

Standardize all attribute expressions in `html!` macros across the Dracon Platform to use explicit string literals `{"class_name"}` instead of bare identifiers `{class_name}`. This makes the distinction between literal CSS class names and dynamic Rust variables visually unambiguous.

## Problem

In Azumi's `html!` macro, `class={bare_ident}` is treated as a string literal `"bare_ident"` — it works identically to `class={"bare_ident"}`. However, the codebase mixes two styles:

| Style | Example | Ambiguity |
|-------|---------|-----------|
| **Explicit string** | `class={"home_page"}` | ✅ Clearly a literal class name |
| **Bare identifier** | `class={navbar}` | ❌ Looks like a variable reference |

A reader must check whether `navbar` is a Rust variable defined elsewhere or a CSS class name. This is a readability issue that compounds across 5 files.

## Scope (Verified)

### nav.rs — 22 bare identifiers to fix

All are CSS class names, not Rust variables. Every `class=` expression below should become `class={"..."}`:

| Line(s) | Current | Target |
|---------|---------|--------|
| 117 | `class={navbar}` | `class={"navbar"}` |
| 120 | `class={navbar_inner}` | `class={"navbar_inner"}` |
| 123 | `class={skip_link}` | `class={"skip_link"}` |
| 126 | `class={logo_section}` | `class={"logo_section"}` |
| 140 | `class={links_section}` | `class={"links_section"}` |
| 141 | `class={actions_section}` | `class={"actions_section"}` |
| 153 | `class={login_btn}` | `class={"login_btn"}` |
| 170 | `class={user_menu_wrapper}` | `class={"user_menu_wrapper"}` |
| 173 | `class={avatar_toggle}` | `class={"avatar_toggle"}` |
| 184 | `class={avatar_placeholder}` | `class={"avatar_placeholder"}` |
| 187 | `class={dropdown}` | `class={"dropdown"}` |
| 192 | `class={dropdown_header}` | `class={"dropdown_header"}` |
| 194 | `class={user_name}` | `class={"user_name"}` |
| 198 | `class={dropdown_link}` | `class={"dropdown_link"}` |
| 208 | `class={dropdown_link}` | `class={"dropdown_link"}` |
| 215 | `class={dropdown_link}` | `class={"dropdown_link"}` |
| 222 | `class={divider}` | `class={"divider"}` |
| 253 | `class={mobile_menu_wrapper}` | `class={"mobile_menu_wrapper"}` |
| 260 | `class={mobile_burger}` | `class={"mobile_burger"}` |
| 289 | `class={mobile_menu_links}` | `class={"mobile_menu_links"}` |
| 296 | `class={mobile_nav_link}` | `class={"mobile_nav_link"}` |
| 302 | `class={mobile_divider}` | `class={"mobile_divider"}` |
| 308+ | `class={mobile_nav_btn}` | `class={"mobile_nav_btn"}` |

Also in nav.rs, check for non-class attributes:
| Line | Current | Target |
|------|---------|--------|
| ~170 | `id={dropdown_id}` | `id={"dropdown_id"}` (if literal) or keep if variable |
| Various | `type={"button"}` | ✅ Already explicit |

### footer.rs — 9 bare identifiers to fix

| Line(s) | Current | Target |
|---------|---------|--------|
| 62 | `class={footer_columns}` | `class={"footer_columns"}` |
| 65 | `class={col_title}` | `class={"col_title"}` |
| 66 | `class={col_link}` | `class={"col_link"}` |
| 68 | `class={col_link}` | `class={"col_link"}` |
| 96 | `class={footer_wrap}` | `class={"footer_wrap"}` |
| 97 | `class={container}` | `class={"container"}` |
| 98 | `class={grid}` | `class={"grid"}` |
| 100 | `class={logo_text}` | `class={"logo_text"}` |
| 101 | `class={tagline}` | `class={"tagline"}` |
| 102 | `class={copyright}` | `class={"copyright"}` |

### Already Consistent (no changes needed)

- **pages.rs** — Already uses `class={"string_explicit"}` throughout ✅
- **base.rs** — Uses `class={"string_explicit"}` for literals, `class={row_class}` for real variable ✅
- **sidebar.rs** — Already uses `class={"string_explicit"}` ✅
- **dashboard-app/layout.rs** — Already uses `class={"string_explicit"}` ✅
- **ai-hub/render.rs** — Already uses `class={"string_explicit"}` ✅

## Implementation Plan

- [ ] Fix **nav.rs** — 22+ bare `class={ident}` → `class={"ident"}`, also `id={dropdown_id}` if literal
- [ ] Fix **footer.rs** — 9 bare `class={ident}` → `class={"ident"}`
- [ ] Build and test verification

## Verification

After applying:
```bash
cd /home/dracon/Dev/dracon-platform
cargo check -p chrome 2>&1 | tail -5
cargo test -p chrome 2>&1 | grep "test result:"
```

Expected: Zero build errors, all tests passing.

## Commands to Apply (sed)

```bash
# nav.rs — all bare class identifiers to string literals
cd /home/dracon/Dev/dracon-platform/libs/chrome/src
sed -i \
  -e 's/class={navbar}/class={"navbar"}/g' \
  -e 's/class={navbar_inner}/class={"navbar_inner"}/g' \
  -e 's/class={skip_link}/class={"skip_link"}/g' \
  -e 's/class={logo_section}/class={"logo_section"}/g' \
  -e 's/class={links_section}/class={"links_section"}/g' \
  -e 's/class={actions_section}/class={"actions_section"}/g' \
  -e 's/class={login_btn}/class={"login_btn"}/g' \
  -e 's/class={user_menu_wrapper}/class={"user_menu_wrapper"}/g' \
  -e 's/class={avatar_toggle}/class={"avatar_toggle"}/g' \
  -e 's/class={avatar_placeholder}/class={"avatar_placeholder"}/g' \
  -e 's/class={dropdown}/class={"dropdown"}/g' \
  -e 's/class={dropdown_header}/class={"dropdown_header"}/g' \
  -e 's/class={user_name}/class={"user_name"}/g' \
  -e 's/class={dropdown_link}/class={"dropdown_link"}/g' \
  -e 's/class={divider}/class={"divider"}/g' \
  -e 's/class={mobile_menu_wrapper}/class={"mobile_menu_wrapper"}/g' \
  -e 's/class={mobile_burger}/class={"mobile_burger"}/g' \
  -e 's/class={mobile_menu_links}/class={"mobile_menu_links"}/g' \
  -e 's/class={mobile_nav_link}/class={"mobile_nav_link"}/g' \
  -e 's/class={mobile_divider}/class={"mobile_divider"}/g' \
  -e 's/class={mobile_nav_btn}/class={"mobile_nav_btn"}/g' \
  -e 's/id={dropdown_id}/id={"dropdown_id"}/g' \
  nav.rs

# footer.rs — all bare class identifiers to string literals
sed -i \
  -e 's/class={footer_columns}/class={"footer_columns"}/g' \
  -e 's/class={col_title}/class={"col_title"}/g' \
  -e 's/class={col_link}/class={"col_link"}/g' \
  -e 's/class={footer_wrap}/class={"footer_wrap"}/g' \
  -e 's/class={container}/class={"container"}/g' \
  -e 's/class={grid}/class={"grid"}/g' \
  -e 's/class={logo_text}/class={"logo_text"}/g' \
  -e 's/class={tagline}/class={"tagline"}/g' \
  -e 's/class={copyright}/class={"copyright"}/g' \
  footer.rs
```

## Verification Commands

```bash
cd /home/dracon/Dev/dracon-platform
cargo check -p chrome 2>&1 | tail -5
cargo test -p chrome 2>&1 | grep "test result:"
```
