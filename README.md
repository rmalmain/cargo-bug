# Cargo Bug Reproducer

A dummy repo reproducing a cargo bug.


## How to Reproduce

From the top directory:
```
cargo build --features feature1,feature2
```
causes a compiler error since the dependency issues one if both features are enabled.


However:
```
cargo clippy --features feature1,feature2
```
also triggers a compiler error, which should not be the case since we are running with clippy.


Interestingly, the last command run from the `my-dep` directory works as expected.


This behavior has been observed with both `rustc 1.79.0 (129f3b996 2024-06-10)` (stable-x86_64-unknown-linux-gnu unchanged) and `rustc 1.81.0-nightly (d9284afea 2024-07-14)` (nightly-x86_64-unknown-linux-gnu unchanged).
