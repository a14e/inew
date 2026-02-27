# INew Test Fail Crate

This private crate only contains INew integration tests that are meant to fail.
These are run using the `trybuild` harness found in [../tests/fail_crate.rs](../tests/fail_crate.rs).

It exists to provide editor syntax highlighting for the test files, which must be ignored by the `cargo test` command from the root directory.
You may have to open the [fail_crate](.) directory in another editor window, to make rust-analyzer recognize its contents.

All the tests must include an empty main function at the end:

```rust
fn main() {}
```
