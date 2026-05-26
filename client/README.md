# JavaScript Build Pipeline

## Files

| File | Size | Purpose |
|------|------|---------|
| `client/azumi.js` | 47KB | Azumi runtime source (1,281 lines) |
| `client/idiomorph.js` | 39KB | Idiomorph morphing library |
| `client/azumi.d.ts` | 6KB | TypeScript type definitions |
| `src/client.min.js` | 42KB | **Bundle** = Idiomorph + Azumi, minified, embedded via `include_str!()` |
| `dracon-platform/libs/chrome/src/static/azumi-runtime.js` | 47KB | Production copy (Azumi only, no Idiomorph) — served as `/static/azumi.js` |

## How src/client.min.js is built

```
client/idiomorph.js + client/azumi.js
         │
         ▼  concatenate + minify
  src/client.min.js
         │
         ▼  include_str!("client.min.js")
  src/lib.rs → AZUMI_JS constant
```

**Minification:** Currently manual (no automated script). Target: single-line with minimal whitespace.

**To regenerate:** Concatenate the two source files, then minify. Future: add `make minify` or a build script.

## Hot Reload Guard

All three copies have the dev-mode guard:

```js
// In constructor():
if (window.location.port || document.querySelector('meta[name="azumi-dev"]')) {
    this.connectHotReload();
}
```

**Note:** The hot reload functions (~85 lines) are still in the bundle for all copies. They're just not called in production. Full removal from the production bundle requires a build step that strips them.

## Development Workflow

1. Edit `client/azumi.js`
2. Regenerate `src/client.min.js` (concatenate + minify)
3. `cargo build` embeds the new bundle
4. If using separate Azumi + Idiomorph scripts (demo mode), also copy to static directory

## Known Issues

- `src/client.min.js` has 1,497 lines despite being "minified" — likely due to minification expanding certain constructs
- No automated minification pipeline — currently manual
- No CI check for bundle size
