locount
---

Fast zero dependency lones of code counter in Rust.

## Example usage
First build
```bash
cargo build -r
```
then run
```bash
target/release/locount [path/to/file] [extension]
```

## Benchmarking
Running `locount` vs `tokei` against Zed codebase:
```text
# time target/release/locount path/to/zed rs
0.062 seconds

# time tokei -t=Rust path/to/zed
0.120 seconds
```
