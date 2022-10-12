# FST Network Rust Template Repository

[![Build Status](https://drone.dev.fst.network/api/badges/fstnetwork/template-rust/status.svg)](https://drone.dev.fst.network/fstnetwork/template-rust)

A template for creating `Rust` project for `FST Network`.

- Make FST Network great!
- Сделайте FST Network отличным!

## Development

- [`.github`](./.github) contains [GitHub Actions](https://github.com/features/actions) related definition.
- [`dev-support`](./dev-support) contains development utilities
  - [`dev-support/bin`](./dev-support/bin) contains tools which will be used through development process
  - [`dev-support/ci-bin`](./dev-support/ci-bin) contains scripts used by [GitHub Actions](https://github.com/features/actions)
  - [`dev-support/containers`](./dev-support/containers) contains the container related definitions
  - [`dev-support/nix-overlay`](./dev-support/nix-overlay) contains overrides to Nix channel or custom derivations

### Git Hooks

It is suggested to install git hook for style linting before code committing. This project is configured with [pre-commit](https://pre-commit.com).

Installation steps:

```bash
pre-commit install --install-hooks -t commit-msg -t pre-commit
```

### Tools

There are some useful commands just like the one in `Rust` toolchain but with proper arguments:

- `cargo build-all`
- `cargo clippy-all`
- `cargo test-all`
- `cargo doc-all`
- `cargo miri-all`
- `cargo watch-all` (ex. `cargo watch-all clippy`)

Please perform the following steps before submitting Pull Request:

It is suggested to perform the following steps before submitting Pull Request:

- Run [codespell](https://github.com/codespell-project/codespell) to find out common misspellings
- Run [format-check](./dev-support/bin/format-check) to check format of `Rust`, `Shell`, `Nix`, `JavaScript`, `TypeScript`, `Markdown`, `JSON`, `YAML`
- Run [cargo clippy-all](./dev-support/bin/cargo-clippy-all) to lint common mistakes of `Rust`
- Run [cargo test-all](./dev-support/bin/cargo-test-all) to perform tests
- Run [udeps](./dev-support/bin/udeps) to check unused dependencies

Other tools:

- [generate-changelog](./dev-support/bin/generate-changelog): Generate from commit messages
- [install-deps](./dev-support/bin/install-deps): Install saffron dependencies
- [format-all](./dev-support/bin/format-all): format all files
- [format-rust](./dev-support/bin/format-rust): format `*.rs` files
- [format-sql](./dev-support/bin/format-sql): format `*.sql` files
- [lines-of-code](./dev-support/bin/lines-of-code): Count lines of code in this project
