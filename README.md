locount
---

Fast zero dependency lines of code counter.

## Example usage
First build
```bash
cargo build -r
```
then run
```bash
target/release/locount [--mode | -m] [--extension] [--exclude] [path/to/file]
```

`mode` is either `fast` (default) or `exact`. Fast mode may avoids per-character scans but may treat
very rare edge cases as comments instead of code, e.g. `/*block comment*/ return 42;` line is treated as
comment under `fast` mode vs. as code under `exact` mode. Exact mode is roughly 30% slower than fast mode.

`[exclude]` is an optional list of comma separated directories for exclusion (no whitespaces).

## Benchmarking
Running `locount` vs `tokei` against Servo codebase:
```text
time target/release/locount ~/code/rust/servo --extension rs
Files: 1750
           | Files      | Total      | Code       | Comments   | Empty      | Code %     | Comments % | Empty %
---------------------------------------------------------------------------------------------------------------
rs         | 1750       | 586123     | 446568     | 79793      | 59762      | 76.2       | 13.6       | 10.2

0.09s user 0.31s system 127% cpu 0.316 total
```

```
time tokei ~/code/rust/servo --compact --num-format commas --rsort lines --types=Rust
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Rust                  1,750      555,696      447,131       49,247       59,318

0.56s user 1.23s system 212% cpu 0.841 total
```

TODOs
- docstring counting
- proper arg parsing funcionality with help pages (without using `clap` or similar)
- better automatic directoy exclusion (e.g. `target` for Rust or `node_modules` for js)
- install shell scripts
- add file counts
