# Project State

## Current Focus
Restore and harden client-side ephemeral UI state via az-ui + set commands with morph preservation.

## Completed
- [x] Preserve az-local-state and az-ui across Idiomorph morphs by saving/restoring attributes before and after morph.
- [x] Reintroduce set command in client runtime to mutate az-ui state without server round-trips, resolving assignment expressions against the closest [az-ui] element.
- [x] Update azumi_plus_demo and Lesson 10 to use az-ui for client-side counters, tabs, and accordions and clarify az-ui vs az-scope semantics.
- [x] Adjust asset manifest ordering and add nature.jpg entry with updated hashed filenames; lock updated dependencies.
