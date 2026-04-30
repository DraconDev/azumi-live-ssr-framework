# ProjectState

## Current Focus
Add client-side UI state management via `az-ui` attribute with `set` command support for ephemeral state.

## Completed
- [x] Introduced `az-ui` attribute for client-side UI chrome (tabs, toggles) that stores JSON state locally without server round-trips
- [x] Implemented `set` command in `az-on` syntax for mutating `az-ui` state (e.g., `<button az-on="click set active_tab = 'rust'">`)
- [x] Added documentation comparing `az-scope` (persistent server data) vs `az-ui` (ephemeral UI chrome) with usage examples
- [x] Enhanced `az-bind` syntax to work with `az-ui` values for dynamic UI updates
