## Goal
Implement P0 items from todo.md: bug fixes first, then bind:value, @keyed, shrink DSL.

## Checklist

### Bug Fixes
- [ ] Fix stale azumi-runtime.js in production (copy from client/azumi.js)
- [ ] Investigate src/client.min.js line count anomaly (should be fewer lines than source)
- [ ] Document JS propagation pipeline

### P0.3: Shrink Predicate DSL
- [ ] Kill ternary support: remove parseTernary(), findTernaryIndex(), findOperatorIndex()
- [ ] Kill numeric comparisons from evaluatePredicate()
- [ ] Simplify evaluateExpression() — keep only field lookup, string/number/boolean literals
- [ ] Simplify evaluatePredicate() — keep only !, ==, !=, truthy
- [ ] Verify bundle size reduction

### P0.1: bind:value
- [ ] Client-side syncBinding() method
- [ ] bind:value attribute recognition in token parser
- [ ] bind:checked for checkboxes

### P0.2: @keyed
- [ ] Client-side key matching in Idiomorph morph step
- [ ] @keyed syntax in token parser
- [ ] data-key attribute generation in codegen