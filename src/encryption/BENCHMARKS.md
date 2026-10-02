# Encryption benchmarks

The `encryption` Cargo benchmark uses Criterion on stable Rust. Cipher
implementations remain std-only; Criterion is a development dependency.
The benchmark imports the existing module tree because this package has no
library target.

Run a correctness smoke check without timing:

```sh
cargo test --bench encryption
```

Save measurements before an implementation change, then compare afterward:

```sh
cargo bench --bench encryption -- --save-baseline before
cargo bench --bench encryption -- --baseline before
```

Criterion writes measurements under `target/criterion`. Keep that directory
between runs. For a shorter exploratory run, append:

```sh
--warm-up-time 0.1 --measurement-time 0.25 --sample-size 20
```

The 48 cases cover empty, small, and larger inputs; ASCII and mixed Unicode
text; Caesar; Vigenère encryption/decryption; and XOR/RC4 with 1-, 16-, and
256-byte keys. Byte inputs span all byte values. IDs identify input byte counts
and key lengths. Throughput measures input bytes; empty-input cases measure
call overhead. RC4 includes lowercase hexadecimal encoding in its timings.

Input generation and Vigenère decryption setup occur outside timed regions.
Each measured call includes key processing, output allocation, and output
destruction. `std::hint::black_box` prevents input-dependent optimization.

Use the same machine, toolchain, build flags, and benchmark settings for both
runs, with other workloads idle. Short runs are indicative; repeat the default
measurements before treating a small difference as a regression. Ciphertext
correctness is checked separately by unit tests, not inferred from timing.
