# manifestlint

Lints Cargo or Node.js manifests for missing required fields (like license or repository) across a monorepo. Fills the gap left by standard linters (eslint or clippy) which focus on code rather than manifest completeness.

## Status

**built, untested**: the recursive directory walking and TOML/JSON parsing logic is built. It has not been run against a real monorepo in this sandbox environment.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
manifestlint --dir . --require license,repository,description
```
