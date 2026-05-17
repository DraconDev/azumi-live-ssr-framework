# Azumi TODO

- [x] P0: Fix demo `Form` import — added `use axum::extract::Form;` to `demo/src/examples/blog/actions.rs`
- [x] P0: Fix all demo static attribute errors — converted `class="..."` → `class:external="..."`, `id="..."` → `id:external="..."`, `style="..."` → `style={var}` across `layout.rs`, `actions.rs`, `pages.rs`
- [x] P0: Fix demo type errors — `</>` fragment → `<div>`, `impl Component` → generic `T: Component`, `Box<dyn Component>` for match arms, `PostContent` lifetime fix
- [x] P0: Fix unused import warnings in demo — removed `azumi::prelude::*` from `contact.rs`, `contact_action` re-export from `mod.rs`, `Post` from `pages.rs`
- [x] P0: CI already exists at `.github/workflows/ci.yml` — comprehensive (demo build, clippy, tests, MSRV, audit, deny, cross-platform, benchmarks)
- [x] P1: Make doc-tests runnable — 14 now passing (up from 2), 16 ignored (down from 27). Made `ActionResult`, CSP builder/nonce, `FormValidator`, `render_to_writer`, `FnComponent`, `AzumiScript` doc-tests runnable. Remaining ignored tests genuinely need `axum` router/handler context.
- [x] P1: Split `html_structure_validator.rs` (1,478 lines) into focused sub-modules: `tag_data.rs` (303 lines), `html_rules.rs` (166 lines), `html_structure_validator.rs` (975 lines)
