# Project State
This commit modifies the Azumi client component to centralize state management, enabling smoother transitions between client-side and server-side predictions.

## Completed
- Implemented state resolution logic using a WeakMap, az-ui, and az-scope attributes
- Added integration and fallback mechanisms for reading from multiple scales
- Enhanced data binding support with improved syntax handling
- Refactored the `readState` and `updateBindings` methods for cleaner logic and priority ordering
