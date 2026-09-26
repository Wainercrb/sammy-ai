# Test Boundary Documentation Specification

## Purpose

Define accurate project documentation for the boundary between Cargo unit tests,
production compilation, and future integration testing without changing the
crate layout, runtime behavior, or testing architecture.

## Requirements

### Requirement: Document Test-Only Compilation Boundary

The project documentation MUST state that the unit-test module in
`src/tools/mod.rs`, guarded by `#[cfg(test)]`, is compiled by Cargo's test
harness and is excluded from a normal production build. The documentation MUST
clarify that physical co-location beneath `src/` does not cause the test module
to be included in the production binary.

#### Scenario: Explain co-located unit tests

- GIVEN a developer sees unit tests in `src/tools/mod.rs`
- WHEN the developer reads the project test guidance
- THEN the guidance identifies `#[cfg(test)]` as the condition that limits the module to Cargo test compilation
- AND the guidance states that the module is excluded from normal production compilation

#### Scenario: Avoid implying production leakage

- GIVEN a developer is concerned that a test module under `src/` ships with the application
- WHEN the developer reads the boundary documentation
- THEN the documentation explains that directory placement alone does not include the guarded module in the production binary
- AND the documentation does not claim that the current tests alter runtime behavior

### Requirement: Describe Current Offline Test Coverage

The project documentation MUST describe the current tests as offline unit tests
of local private helpers. It MUST state that these tests require neither an
OpenRouter credential nor network access, and it MUST NOT represent them as
coverage of the live CLI or provider boundary.

#### Scenario: Run the current unit-test suite offline

- GIVEN a developer has a Rust toolchain and no OpenRouter credential
- WHEN the developer follows the documented command to run the unit-test suite
- THEN the guidance identifies the suite as offline unit testing of local helpers
- AND the guidance states that provider credentials and network access are not required

#### Scenario: Distinguish live CLI execution from unit coverage

- GIVEN a developer needs confidence in provider-backed CLI behavior
- WHEN the developer reads the current test-coverage description
- THEN the documentation states that the offline unit tests do not execute the live CLI or OpenRouter boundary
- AND the documentation does not imply provider-backed, networked, or end-to-end coverage exists

### Requirement: Reserve Integration Tests for a Defined Future Need

The project documentation MUST distinguish the current unit-test suite from
black-box integration tests. It MUST state that `tests/` is currently absent
and reserved for a future explicitly defined requirement that exercises public
or CLI behavior. The documentation MUST NOT prescribe a library-target refactor,
test relocation, or a provider-backed testing approach as part of this boundary
description.

#### Scenario: Explain the absent integration-test directory

- GIVEN a developer looks for integration tests under `tests/`
- WHEN the developer reads the test-boundary documentation
- THEN the documentation states that no current integration-test suite is defined there
- AND the documentation reserves that location for a future approved black-box integration requirement

#### Scenario: Keep future test architecture undecided

- GIVEN a developer considers adding CLI or provider integration coverage
- WHEN the developer reads the boundary documentation
- THEN the documentation treats the need as future work requiring an explicit definition
- AND the documentation does not mandate a library target, move existing private-helper tests, or add provider-backed tests

### Requirement: Document Boundary Validation Commands

The project documentation MUST identify `cargo test` as validation for the
current offline unit-test suite and `cargo build` as validation of normal
production compilation. It MUST state the expected boundary: the former runs
the test harness, while the latter builds the production binary without the
`#[cfg(test)]` module.

#### Scenario: Validate the documented boundary

- GIVEN a developer has applied only the test-boundary documentation clarification
- WHEN the developer follows the documented validation guidance
- THEN the guidance instructs the developer to run `cargo test` for the offline unit suite and `cargo build` for the production binary
- AND the guidance describes the distinct compilation scope validated by each command

#### Scenario: Handle unavailable live credentials during validation

- GIVEN `OPENROUTER_API_KEY` is not set and network access is unavailable
- WHEN the developer follows the documented test-boundary validation guidance
- THEN the guidance allows the offline unit-test validation to proceed without either dependency
- AND the guidance does not require live CLI execution to validate the documented boundary
