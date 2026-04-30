# Project State

## Current Focus
Add parenthetical grouping support to expression evaluation in Azumi client and tests.

## Completed
- [x] Enable `(...)` grouping in expression evaluation by stripping outer parentheses and recursively evaluating the inner expression.
- [x] Extend Azumi client (`client/azumi.js` and minified `src/client.min.js`) with parenthetical handling before string-literal checks.
- [x] Mirror parenthetical logic in `tests/azumi_js_tests.js` to keep test evaluation consistent with production behavior.
- [x] Update `Cargo.lock` to reflect latest dependency versions.
