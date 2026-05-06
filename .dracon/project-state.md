# Project State

## Current Focus
Added utility functions for schema processing in the macros crate

## Context
The changes prepare the schema processing infrastructure by adding helper functions that will be used to extract and validate schema attributes during code generation.

## Completed
- [x] Added `extract_schema_type` to extract schema type from attributes
- [x] Added `should_skip_field` to check if a field should be skipped
- [x] Added `extract_field_name` to extract custom field names
- [x] Added `is_option_type` to check for Option<T> types
- [x] Added `is_vec_type` to check for Vec<T> types

## In Progress
- [x] Implementation of schema processing utilities

## Blockers
- None identified

## Next Steps
1. Implement schema processing logic using these utilities
2. Integrate with existing schema derivation macros
