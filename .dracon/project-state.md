# Project State

## Current Focus
Improve JavaScript expression evaluator handling of string literals, operator parsing, and clarify behavior of equality, property lookup, and `||` in expressions.

## Completed
- [x] Fixed operator index search to scan from right to left, correctly handle nested parentheses/brackets and ignore operators inside quoted strings.
- [x] Added string literal detection to prevent premature operator matching and to respect escaped quotes.
- [x] Simplified equality and inequality checks to use strict comparison without defaulting to empty strings.
- [x] Updated predicate evaluation to return boolean based on direct state lookup (`state[expr]`) and removed unnecessary `hasOwnProperty` guard.
- [x] Adjusted expression evaluation to match original `azumi.js` behavior by using `expr in state` instead of `hasOwnProperty`.
- [x] Added tests documenting known limitation: numeric comparisons only support integer literals (floats are parsed as integers on the right-hand side).
- [x] Updated tests to reflect that `||` in expressions is treated as part of the field name, not as JavaScript short‑circuit logical OR.
