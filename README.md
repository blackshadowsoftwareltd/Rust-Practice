## -Rust-Practice

================================================================================

#### create a new rust project

```
cargo new rust-playground
```

#### add clippy

```
rustup component add clippy
```

#### add cargo-watch. This will watch for changes and run the project

```
cargo install cargo-watch
```

#### Run the project with quietly compile (qc)

```
cargo-watch -qc -x run -x clippy
```
