# Project State
This commit modifies the macro generation logic in `macros/src/lib.rs`. The primary change is refactoring the `generate_body_with_context` function to replace custom quote formatting with safer alternation expressions, ensuring consistent and error-free string generation across different contexts. The updated logic maintains the same functionality but improves robustness in handling escape sequences and attribute value names.

## Completed
- Refactored macro injection to remove fragile quine-style string building and improve safety.
