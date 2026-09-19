# Sammy AI Calculator

Sammy AI is a single-prompt command-line calculator. It uses an OpenRouter-backed
agent to select one of three integer operations: addition, multiplication, or
division.

## Supported boundary

The calculator only supports arithmetic requests that use two signed 32-bit
integers. Division uses integer division. A zero denominator and calculations
outside the supported integer range return an error instead of a numeric result.

This project does not support filesystem actions, persistence, authentication,
sandboxing, multi-turn chat, or external capabilities beyond the OpenRouter
provider used for live calculator prompts.

## Prerequisites

- A current Rust toolchain with Cargo.
- An OpenRouter API key for live CLI execution.

## Run the calculator

Set `OPENROUTER_API_KEY` in your environment or in a local `.env` file. Do not
commit credentials to the repository.

```sh
export OPENROUTER_API_KEY="your-key"
cargo run
```

Enter a calculator request such as `multiply 7 by 6`. Live execution needs the
credential above and network access to OpenRouter.

## Develop and verify offline

The arithmetic unit tests run directly against local helpers. They do not need
an OpenRouter credential or network access.

```sh
cargo fmt --check
cargo test
```

The live CLI is intentionally not part of the offline test suite because it is
provider-dependent.
