## Operations log

- Investigated Swiss Ephemeris C file I/O points and identified `swi_fopen()` in `libswisseph/sweph.c` as the primary file-open chokepoint.
- Added a WASI smoke-test example at `examples/wasi_smoke.rs` that uses `swe_set_ephe_path("/eph")` and calls `swe_calc()`.
- Added a placeholder Cargo feature `wasi`.
- Updated `build.rs` with a WASI cross-compilation helper that can pass `--sysroot` when `WASI_SYSROOT` or `WASI_SDK_PATH` is provided.
- Attempted `cargo build --target wasm32-wasip1 --examples`; build currently fails without a WASI sysroot/toolchain installed on the host.
- Fixed `build.rs` to check for the correct submodule directory (`libswisseph/`) and added `cargo:rerun-if-changed` / `cargo:rerun-if-env-changed` hints for more reliable bindgen regeneration.
- Confirmed a native `cargo build` prints the generated `$OUT_DIR/bindings.rs` path (evidence that bindgen output is being produced).
- Added the missing `add_c_files_from_list()` helper in `build.rs` so wasm targets can compile a curated subset of Swiss Ephemeris C sources.
- Verified `cargo build` succeeds natively after the build script changes.

