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
Counts: Counts { code: 1442359, comments: 16510, empty: 154514 }
Total lines of code: 1613383
0.07s user 0.11s system 300% cpu 0.063 total
```

```
# time tokei -t=Rust path/to/zed
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Rust                   1943      1572772      1392083        32122       148567
 |- Markdown            1166        38698          234        34115         4349
 (Total)                          1611470      1392317        66237       152916
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                  1943      1611470      1392317        66237       152916
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
0.39s user 0.17s system 465% cpu 0.120 total
```

TODOs
- proper comment (and docstring) counting using state machine (e.g. incl. `/*...*/ comment blocks)
- proper arg parsing funcionality with help pages (without using `clap` or similar)
- better automatic directoy exclusion (e.g. `target` for Rust or `.venv` for Python)
- exclusion of all hidden (`.`) directories
