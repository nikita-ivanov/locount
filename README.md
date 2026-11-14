locount
---

Fast zero dependency lines of code counter in Rust.

## Example usage
First build
```bash
cargo build -r
```
then run
```bash
target/release/locount [path/to/file] [extension] [mode]
```

`mode` is either `fast` (default) or `exact`. Fast mode may avoids per-character scans but may treat
very rare edge cases as comments instead of code, e.g. `/*block comment*/ return 42;` line is treated as
comment under `fast` mode vs. as code under `exact` mode. Exact mode is roughly 30% slower than fast mode.

## Benchmarking
Running `locount` vs `tokei` against Zed codebase:
```text
time target/release/locount /Users/nivanov/code/rust/zed rs fast
Files: 1935
Chunks: 9
Counts: Counts { code: 1386997, comments: 71872, empty: 152579 }
Total lines of code: 1611448
0.11s user 0.10s system 318% cpu 0.066 total
```

```
time tokei -t=Rust path/to/zed
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
- docstring counting
- proper arg parsing funcionality with help pages (without using `clap` or similar)
- better automatic directoy exclusion (e.g. `target` for Rust or `.venv` for Python)
- exclusion of all hidden (`.`) directories
- tests
- install shell scripts
- counting of all extensions, not only 1 extension per run
