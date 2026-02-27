# INew Test Fail Crate

This private crate only contains INew integration tests that are meant to fail.
These are run using the `trybuild` harness found in [../tests/fail_crate.rs](../tests/fail_crate.rs).

It exists to keep the files that fail to compile separate, which must be ignored by the `cargo test` command from the root directory.
Also, to avoid publishing the contents of this crate to [crates.io](https://crates.io/).

It's added as a workspace member in the parent's [Cargo.toml](../Cargo.toml) to have autocompletion and syntax highlighting when using `rust-analyzer`.

All the tests must include an empty main function at the end:

```rust
fn main() {}
```
