# Project State

## Current Focus
Refactor Live macro to eliminate temporary predictions_entries collection and static __AZUMI_PREDICTIONS constant, simplifying code.

## Completed
- [x] Removed `predictions_entries.iter().collect::<Vec<_>>()` and associated variable
- [x] Dropped the `#(#predictions_const)*` expansion for `__AZUMI_PREDICTIONS` static const
