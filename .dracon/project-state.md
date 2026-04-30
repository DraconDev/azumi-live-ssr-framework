# Project State

## Current Focus
Update operator index expectations in azumi.js tests to match right-to-left scanning semantics and add new ternary negation test.

## Completed
- [x] Removed incorrect test for first `&&` in chain and added test asserting rightmost `&&` at index 7.
- [x] Updated expectation for `&&` in “a < b && c > d” from index 6 to 8.
- [x] Added test for `!` operator in ternary expression, expecting index 0.
- [x] Updated Cargo.lock to reflect new dependency versions and binary size changes.
