A few interesting quine programs in Rust.

### Two programs:

- `quine.rs` is a quine that prints its own source code.
- `hashquine.rs` is a prints its own SHA-256 hash.

### Running

1. Clone the repo:
```bash
$ git clone https://github.com/mtraverso3/rust-quine.git
```

2. Run with cargo:

#### quine.rs
```bash
$ cargo run --bin quine
```
The output should match the source code of `quine.rs`.

#### hashquine.rs
```bash
$ cargo run --bin hashquine
```
The output should match the SHA-256 hash of `hashquine.rs`.
