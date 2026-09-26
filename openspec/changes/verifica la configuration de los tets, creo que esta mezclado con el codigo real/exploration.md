## Exploration: Verify Whether Test Configuration Is Mixed With Production Code

### Current State
**Verified evidence:** This is a Cargo binary-only crate (`Cargo.toml`, `cargo metadata`); it has no test-runner configuration, dev-dependencies, workspace overrides, or `tests/` directory. Cargo's built-in test harness compiles `src/main.rs` as the test target.

The five tests are an inline `#[cfg(test)] mod tests` in `src/tools/mod.rs` (lines 65-132). They exercise pure arithmetic helpers, argument deserialization, and the shared JSON schema. `#[cfg(test)]` excludes that module from normal production compilation, so test code is co-located with, but not shipped in, the production binary. `cargo test` completed successfully: 5 passed.

`src/main.rs` is the live CLI boundary: it loads `.env`, creates an OpenRouter client, reads stdin, and sends a provider-backed prompt. The existing tests do not initialize environment variables, call the provider, use the network, or execute the CLI. This matches `README.md` lines 35-46.

**Hypothesis:** The concern likely comes from the physical location of the tests under `src/`, not from test configuration leaking into runtime behavior. The verified evidence does not support a runtime mixing problem.

### Affected Areas
- `Cargo.toml` — defines one binary target and has no explicit test configuration or dev-dependencies.
- `src/tools/mod.rs` — contains production arithmetic helpers and their conditional unit tests.
- `src/main.rs` — production-only OpenRouter CLI boundary; currently not covered by automated tests.
- `README.md` — documents the offline unit-test boundary and the live provider-dependent CLI.
- `tests/` (absent) — would be the conventional location for future black-box integration tests.

### Approaches
1. **Keep focused unit tests co-located** — Retain the `#[cfg(test)]` module next to the private helper code and document this as the unit-test convention.
   - Pros: Idiomatic Cargo layout; no production binary inclusion; preserves private helper testing; no API or crate restructuring.
   - Cons: Test and production code remain physically adjacent; no integration-level coverage of the CLI boundary.
   - Effort: Low

2. **Introduce a library target and external integration tests** — Move reusable calculator logic into `src/lib.rs`, keep `main.rs` as composition, and add `tests/` for public behavior.
   - Pros: Separates executable composition from reusable logic; supports black-box tests and future provider abstraction.
   - Cons: Requires a public/internal API decision and module restructuring; external tests cannot directly exercise private helpers.
   - Effort: Medium

### Recommendation
Do not relocate the current unit tests merely because they live in `src/tools/mod.rs`; they are conditionally compiled and correctly isolated from the runtime binary. Run `sdd-propose` to decide whether the change should be limited to documenting and tightening the test boundary, or should deliberately introduce a library/integration-test architecture. Choose the second path only if testing the CLI composition or future non-provider behavior is a requirement.

### Risks
- Converting the binary-only crate to a library can broaden visibility and change module boundaries without delivering new behavioral coverage.
- Moving private-helper tests to `tests/` would either force unnecessary public APIs or require different test seams.
- The live CLI remains intentionally untested; any proposal claiming end-to-end coverage must define a provider seam or a separate authorized live-test strategy.

### Ready for Proposal
Yes — provided the proposal explicitly answers this decision gap: preserve the current unit-test layout and add only conventions/coverage, or restructure around a library target to enable integration tests. The expected scope is low for the first choice and medium for the second; either should remain within the 400-line single-PR review budget if kept focused.
