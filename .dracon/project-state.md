#Project State

## Current Focus
Implemented parenthetical predicate evaluation, advanced ternary expression parsing, and expanded numeric comparison support (including >=, <= and floating‑point values) with dedicated parsing helpers.

## Completed
- [x] Added parenthetical predicate handling (`( expr )`) that bypasses the negation shortcut and delegates to `evaluatePredicate`.
- [x] Replaced the simple `!field` regex match with explicit parenthesis detection and predicate evaluation.
- [x] Introduced `parseTernary` and `findTernaryIndex` methods to correctly locate outermost ternary operators while respecting nested ternaries, quotes, and brackets.
- [x] Updated ternary evaluation to use the new parsing utilities, evaluating condition, true‑branch, and false‑branch separately and converting truthiness to a boolean.
- [x] Upgraded numeric comparison operators (`<`, `>`, `<=`, `>=`) to use `parseFloat`/`Number` and handle decimal literals.
- [x] Modified `<` comparison to parse both left and right operands as floats for more accurate evaluation.
- [x] Added support for `<=`, `>`, and `>=` comparison patterns alongside the existing `<` pattern.
- [x] Removed legacy ternary regex matching code and integrated the new robust parsing logic throughout the expression evaluator.
