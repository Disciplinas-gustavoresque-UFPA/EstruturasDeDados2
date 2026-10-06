# Encryption unit tests

Keep encryption unit tests in this folder. Use one `.rs` file per algorithm
or shared utility; use an algorithm-specific subfolder when its tests need
fixtures, as Twofish does with `twofish/mod.rs` and `twofish/test_data`.

Each implementation attaches its test module with `#[cfg(test)]`,
`#[path = "../tests/..."]`, and `mod tests;`. The physical location changes,
but the tests remain children of the implementation module. This preserves
their existing names and access to private internals without changing the
public API. Keep Rustdoc examples beside the public functions.

Preserve existing test scenarios and expected results when relocating tests.
Keep fixture documentation and Git attributes with the data; the official
Twofish fixtures retain their original bytes and line endings.

Run the ordinary suite with `cargo test`. Run the two long Twofish tests with:

```sh
cargo test --release official_monte_carlo -- --ignored --nocapture
```
