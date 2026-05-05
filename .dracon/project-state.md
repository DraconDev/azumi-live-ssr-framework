# Project State

## Current Focus
Added a test secret for release testing in CI pipeline

## Context
The change introduces a test secret for release builds in the CI pipeline to ensure proper environment handling during testing.

## Completed
- [x] Added `AZUMI_SECRET=ci-release-test-secret-do-not-use-in-prod` to release test command

## In Progress
- [x] CI pipeline configuration update

## Blockers
- None identified

## Next Steps
1. Verify the secret doesn't leak into production environments
2. Ensure all tests pass with the new secret configuration
