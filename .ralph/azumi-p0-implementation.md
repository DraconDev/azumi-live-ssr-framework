## Goal
Implement P0 items from todo.md: bug fixes first, then bind:value, @keyed, shrink DSL.

## Checklist

### Bug Fixes
- [x] Fix stale azumi-runtime.js — done. All copies have conditional guard.
- [x] Investigate src/client.min.js line count anomaly — confirmed NOT anomalous.
- [x] Document JS propagation pipeline — client/README.md created.

### P0.3: Shrink Predicate DSL
- [x] Kill ternary support: removed parseTernary(), findTernaryIndex(), findOperatorIndex()
- [x] Kill numeric comparisons from evaluatePredicate()
- [x] Simplify evaluateExpression() — keep only field lookup + literals
- [x] Simplify evaluatePredicate() — keep only !, ==, !=, truthy
- [x] Rebuild src/client.min.js — build.rs regenerated automatically. Bundle: 40KB/10.4KB gzipped.
- [x] Verify bundle size reduction — 42KB→40KB (-2KB).

### P0.1: bind:value
- [x] Client-side syncBinding() method — added to delegate(), handles input/change, 200ms debounce, checkbox/radio support, nested field paths
- [x] bind:value/bind:checked macro recognition — codegen.rs generates data-bind-value="field.path"
- [x] bind:checked checkbox/radio logic in syncBinding

### P0.2: @keyed
- [x] Client-side key matching — morphKeyed() method matches old↔new children by data-key attribute, morphs pairs, inserts adds, removes deletes
- [x] @keyed syntax in token parser — ForBlock now has optional key_expr field, parsed as @keyed(expr)
- [x] data-key attribute generation in codegen — GenerationContext.key_expr propagated to first element in for body