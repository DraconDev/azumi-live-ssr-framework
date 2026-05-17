# Azumi Improvements Batch

Implement all 6 improvements:

1. **Unify live state error messages** — same text across lib.rs, live.rs, component.rs
2. **Add tests for new features** — page route const, action PATH const, devtools guard
3. **Cache AZUMI_ALLOW_DEVTOOLS_IN_RELEASE check** — OnceLock instead of env read every call
4. **Make MAX_STATE_AGE_SECS configurable** — AZUMI_STATE_MAX_AGE env var
5. **Add #[live_state] explicit attribute** — alternative to implicit `state` naming magic
6. **Update demo to showcase route constants** — dogfood the new features

Checklist:
- [ ] 1. Unify error messages
- [ ] 2. Tests for page route const
- [ ] 3. Tests for action PATH const
- [ ] 4. Tests for devtools guard
- [ ] 5. Cache devtools env check
- [ ] 6. Configurable MAX_STATE_AGE_SECS
- [ ] 7. #[live_state] attribute support
- [ ] 8. Demo showcase
- [ ] 9. All tests pass
- [ ] 10. AGENTS.md updated if needed