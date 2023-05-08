#### create a new rust project

```
cargo new rust-playground
```

================================================================================

#### add git ignore

```
touch .gitignore
```

```
# Ignore compiled binary files
target/
# Ignore compiled files
*.rs.bk
# Ignore Cargo.lock file
Cargo.lock

# Ignore editor backup files
*~
```

================================================================================

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

=============================================

#### create package (intutils is a package name)

````
cargo new --lib intutils
```
````
