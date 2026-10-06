# Twofish test data

The uppercase `.TXT` files and extensionless `README` are byte-for-byte copies
from the [designers' known-answer archive](https://www.schneier.com/wp-content/uploads/2015/12/twofish-kat.zip).
Git attributes preserve their original CRLF line endings. No expected value or
case has been edited to accommodate the Rust implementation.

The ordinary tests check every published case in `ECB_VK.TXT` (576),
`ECB_VT.TXT` (384) and `ECB_TBL.TXT` (147), in both directions. `ECB_IVAL.TXT`
provides the 40 subkeys and Reed–Solomon results for each supported key size.

`q_permutations.txt` contains the two 256-byte permutations from `TABLE.H` in
the [official C reference archive](https://www.schneier.com/wp-content/uploads/2015/12/twofish-reference-c.zip).
The C archive's SHA-256 is
`9b811147da65c3bbb8faace890c232df72b19ea14c82ff04fc556faf99cd38d7`.

`reference_vectors.txt` contains 778 fixed extra vectors generated using that
C reference, independently of the Rust implementation. Before generating them,
the reference was checked against all 1,107 published cases in both directions
and all three published key schedules. On the 64-bit little-endian host, only
the `DWORD` typedef (`uint32_t`, with a size assertion) and the `LittleEndian`
configuration were adapted. No algorithm or test expectation was changed.
Each generated row stores the hexadecimal key, plaintext and ciphertext.

The generation recipes are:

- For key lengths 16, 24 and 32, key byte `i` is `0x80 + i`. For each value
  `v` from 0 through 255, block byte `i` is `(v + 17 * i) mod 256`. These 768
  vectors cover every byte value at every block position for each key size.
- At each key length, zero, `0xff`, and alternating `0xaa`/`0x55` keys encrypt
  the block `00017F80FEFFAA551020304050607090` (nine vectors).
- The final vector encrypts `Twofish in Rust!` with the key `0123456789abcdef`.

The tests read the stored results; running them requires neither C nor network
access. The Rust implementation never generates its own expected ciphertext.

## Long Monte Carlo validation

`ECB_E_M.TXT` and `ECB_D_M.TXT` are also untouched files from the official KAT
archive. The two ignored tests each verify all 1,200 published outer cases:
400 per key size and 10,000 block operations per case. Together they execute
24 million operations. They also verify the input and key progression from
the final and penultimate outputs, as specified by `TST2FISH.C`.

Key expansion is reused within each outer case. The ordinary vectors test the
public functions that expand the key independently for each call. Run the
complete long suite explicitly with:

```sh
cargo test --release official_monte_carlo -- --ignored --nocapture
```

Both long tests must pass before delivery. Ignoring them in the ordinary suite
keeps routine development checks fast, without reducing their iteration counts
or modifying their scenarios.
