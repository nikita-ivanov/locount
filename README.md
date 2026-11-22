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
Running `locount` vs `tokei` against Zed codebase:
```text
time target/release/locount --extension rs ~/code/rust/zed
Files: 2051
           | Files      | Total      | Code       | Comments   | Empty      | Code %     | Comments % | Empty %
---------------------------------------------------------------------------------------------------------------
rs         | 2051       | 1842144    | 1597733    | 78596      | 165815     | 86.7       | 4.3        | 9.0
0.12s user 0.11s system 326% cpu 0.072 total
```

```
time tokei -t=Rust ~/code/rust/zed
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
- better automatic directoy exclusion (e.g. `target` for Rust or `node_modules` for js)
- install shell scripts
- add file counts
