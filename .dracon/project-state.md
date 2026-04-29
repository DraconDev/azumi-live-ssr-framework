# Project State

## Current Focus
Add comprehensive unit tests for the accessibility validator functions.

## Completed
- [x] Added test helpers (`test_element`, `test_element_with_attrs`, `test_element_with_children`)
- [x] Added tests for `validate_img_alt` (missing alt fails, with alt passes, empty alt passes, non‑img ignored)
- [x] Added tests for `validate_input_type` (valid types pass, invalid types fail, missing type ignored)
- [x] Added tests for `validate_aria_roles` (valid roles pass, invalid role fails, missing role ignored)
- [x] Added tests for `validate_button_content` (text/aria-label/title passes, missing content fails, non‑button ignored)
