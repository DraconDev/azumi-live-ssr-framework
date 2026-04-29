# Project State

## Current FocusFix quote escaping in Azumi string literals and streamline macro quote handling by removing unnecessary quote stripping.

## Completed
- [x] Fixed quote escaping in Azumi string parsing by replacing outer quote slice with a regex that escapes backslashes before inner quotes.
- [x] Removed the `strip_outer_quotes` utility and directly use attribute values in macro generation, simplifying quote handling.
- [x] Updated `HeadArgs` title extraction to use `expect` with a clear error message instead of `unwrap`.
- [x] Added support for null character (`'\0'`) handling in the `escape_css_string` function.
