# Project State

## Current Focus
Refactor Live macro to use static prediction and field constant lookups

## Completed
- [x] Replace map+collect with direct `collect::<Vec<_>>()` for prediction entries
- [x] Switch `local_fields()` to return `Self::__AZUMI_LOCAL_FIELDS`
- [x] Switch `computed_fields()` to return `Self::__AZIMI_COMPUTED_FIELDS`
