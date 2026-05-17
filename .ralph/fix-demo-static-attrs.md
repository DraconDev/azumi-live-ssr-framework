
# Fix all `class="..."`, `id="..."`, and `style="..."` static attributes in the azumi-demo crate

The Azumi macro validator bans static class/id/style attributes. The demo code predates this rule.
All instances need conversion:
- `class="name"` → `class:external="name"`  (for third-party/utility CSS)
- `id="name"` → `id:external="name"`
- `style="..."` → `style={--prop: value}`

## Files to fix:
1. `demo/src/examples/blog/layout.rs`
2. `demo/src/examples/blog/actions.rs`
3. `demo/src/examples/blog/pages.rs`
4. `demo/src/actions/mod.rs` (or its submodules)

## Checklist:
- [ ] Read each file
- [ ] Replace all static class attributes with `class:external="..."`
- [ ] Replace all static id attributes with `id:external="..."`
- [ ] Replace static style attributes with `style={--prop: value}` syntax
- [ ] Also fix the unused import warning in `demo/src/actions/mod.rs`
- [ ] `cargo build -p azumi-demo` passes
