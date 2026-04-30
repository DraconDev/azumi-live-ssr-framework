# Project State

## Current Focus
Added runtime auto‑detection of prediction values from the `az-predictions` attribute when `data-predict` is not set, with console logging and error handling.

## Completed
- [x] Added logic to parse `az-predictions` JSON and extract a prediction based on the `az-on` attribute in `client/azumi.js`
- [x] Added identical auto‑detection logic to the minified `src/client.min.js`
- [x] Updated `Cargo.lock` to reflect the latest dependency versions introduced by recent commits
