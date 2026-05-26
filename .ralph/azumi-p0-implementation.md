## Goal
Implement P0 items from todo.md: bug fixes first, then bind:value, @keyed, shrink DSL.

## Checklist

### Bug Fixes
- [x] Fix stale azumi-runtime.js — done. Source & minified both have conditional guard.
- [x] Investigate src/client.min.js line count anomaly — confirmed NOT anomalous (bundles idiomorph + azumi, 2,161→1,497 is correct minification).
- [x] Document JS propagation pipeline — client/README.md created.

### P0.3: Shrink Predicate DSL
- [x] Kill ternary support: removed parseTernary(), findTernaryIndex(), findOperatorIndex() — 227 lines / 6,896 bytes saved from client/azumi.js
- [x] Kill numeric comparisons from evaluatePredicate() — removed <, >, <=, >=
- [x] Simplify evaluateExpression() — removed ternary, ||, +N, -N, string escaping. Keep only field lookup + literals.
- [x] Simplify evaluatePredicate() — removed &&, ||, ternary, numeric ops. Keep only !, ==, !=, truthy.
- [ ] Rebuild src/client.min.js from modified source (needs minification step — currently manual)
- [ ] Verify bundle size reduction

### P0.1: bind:value
- [ ] Client-side syncBinding() method
- [ ] bind:value attribute recognition in token parser
- [ ] bind:checked for checkboxes

### P0.2: @keyed
- [ ] Client-side key matching in Idiomorph morph step
- [ ] @keyed syntax in token parser
- [ ] data-key attribute generation in codegen