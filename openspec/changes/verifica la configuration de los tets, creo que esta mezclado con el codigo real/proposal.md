# Proposal: Clarify the Test and Production Boundary

## Intent

Clarify that the inline `#[cfg(test)]` unit tests in `src/tools/mod.rs` are compiled only by Cargo's test harness and are excluded from the production binary. The concern is physical co-location under `src/`, not test configuration or runtime code leaking into production.

## Scope

### In Scope
- Document the Cargo unit-test boundary and the purpose of `#[cfg(test)]` in the project test guidance.
- Distinguish offline unit tests for private helpers from future black-box integration tests in `tests/`.
- Define validation that confirms the existing unit tests pass and the production binary still builds without test-only code.

### Out of Scope
- Converting the binary-only crate to a library target or restructuring production modules.
- Moving the existing private-helper unit tests out of `src/tools/mod.rs`.
- Adding provider-backed, networked, or end-to-end CLI tests without a defined provider seam and authorization strategy.

## Capabilities

> This section is the contract between proposal and specs phases.

### New Capabilities

None. This change documents an existing test boundary and adds no product capability.

### Modified Capabilities

None. No specified runtime or user-facing behavior changes.

## Approach

Keep the focused unit tests co-located with their private helpers. Update test documentation to state that `#[cfg(test)]` excludes the test module from normal production compilation, while reserving `tests/` for a future integration requirement that exercises public or CLI behavior. Validate with `cargo test` and a production `cargo build`; do not introduce a library target unless a concrete integration need is approved later.

## Affected Areas

| Area | Impact | Description |
|------|--------|-------------|
| `README.md` | Modified | Document the unit-test versus integration-test boundary and the offline/live CLI distinction. |
| `src/tools/mod.rs` | Unchanged | Remains the location of private-helper unit tests guarded by `#[cfg(test)]`. |
| `Cargo.toml` | Unchanged | Remains a binary-only crate without additional test-runner configuration. |
| `tests/` | Unchanged (absent) | Reserved for a future, explicitly defined integration-test requirement. |

## Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| Documentation could incorrectly imply that CLI or provider behavior has integration coverage. | Low | State explicitly that current tests are offline unit tests and do not execute the live CLI boundary. |
| A future test need could be constrained by the binary-only layout. | Medium | Revisit a library target only when a concrete black-box integration scenario is approved. |
| Test guidance could become stale as the CLI evolves. | Low | Keep the boundary description near the existing run/test instructions and update it with future test additions. |

## Rollback Plan

Revert the documentation-only changes if they prove inaccurate or conflict with an approved test architecture. No production code, Cargo target, or runtime behavior changes in this proposal require migration or operational rollback.

## Dependencies

- Cargo toolchain for `cargo test` and `cargo build` validation.
- No external services or provider credentials are required for the scoped validation.

## Success Criteria

- [ ] Project documentation accurately states that `#[cfg(test)]` unit tests are excluded from normal production compilation.
- [ ] Documentation distinguishes the current offline unit tests from future integration tests under `tests/` and from provider-backed CLI behavior.
- [ ] `cargo test` passes the existing five unit tests without provider or network access.
- [ ] `cargo build` succeeds for the production binary after the documentation clarification.
