# wasi_web (libswisseph-sys example)

This example builds a small WASI-compatible `.wasm` module from this repository (as a Cargo `--example wasi_web`) and includes a minimal web runner that loads it in the browser using a pure-JS WASI shim.

## Build the WASI wasm

You don’t need to manually copy the wasm into `web/`.

`web/Trunk.toml` includes build hooks that automatically:

- run a `pre_build` hook to compile the WASI wasm into `target/wasm32-wasip1/...`
- run a `post_build` hook to copy the resulting artifact into Trunk’s staging/dist directory as `swisseph_wasi.wasm`

This avoids a rebuild loop: if we were to copy the wasm into `web/` (the watched source dir), Trunk would detect the change and rebuild repeatedly.

If you want to build it manually anyway:

```bash
cargo build --target wasm32-wasip1 --example wasi_web
cp ../../target/wasm32-wasip1/debug/examples/wasi_web.wasm web/swisseph_wasi.wasm
```

Notes:

- This crate is currently configured as a `cdylib`. That means it may not export a WASI `_start` entrypoint (i.e. it is not necessarily a "command"). The web runner will still instantiate it and call exported functions (e.g. `swisseph_add`).

## Run the web page (Trunk)

Install Trunk if you don't have it:

```bash
cargo install trunk
```

Then serve the site:

```bash
trunk serve --config web/Trunk.toml --open
```

Trunk will build a static site under `web/dist` and serve it locally.

Note on the WebSocket URL you may see in the browser devtools:

- Trunk uses a WebSocket endpoint at `/.well-known/trunk/ws` for live-reload.
- If you see the WS request failing, the page can still work (it just won’t auto-reload). Usually a WS failure is due to a proxy, browser extension, or something else already bound to the port.

## Filesystem / WASI shims

The browser runner uses `@bjorn3/browser_wasi_shim` and configures:

- `stdin` as empty
- `stdout` / `stderr` forwarded to the page
- a preopened in-memory directory `.` containing `hello.txt`
- a preopened in-memory directory `tmp/`

This is enough for many WASI programs that expect basic WASI file APIs.

## Adding data files (virtual filesystem)

To provide files (e.g. Swiss Ephemeris `ephe` files) to the WASI module:

1. Put the files under `web/assets/`.
2. List them in `web/assets/manifest.json`.
3. Trunk will copy `web/assets/*` to `web/dist/assets/*`.
4. `web/main.js` fetches the listed files and mounts them into an in-memory preopened directory.

Default recommended ephemeris set (add these files under `web/assets/`):
- `seas_18.se1`
- `semo_18.se1`
- `sepl_18.se1`

The default `manifest.json` already lists these as `optional: true`, so the page still runs even if you haven’t added the binary files yet.

Note: In a browser, you generally **must ship these data files as web assets** (or download them at runtime), because the WASI shim can only read files that you provide (in-memory, OPFS, etc.). It cannot access arbitrary host filesystem paths.

By default the manifest mounts at `/ephe` (configurable via the `mount` property).

Example `web/assets/manifest.json`:

```json
{
  "mount": "ephe",
  "files": [
    { "path": "seas_18.se1" },
    { "path": "sepl_18.se1" }
  ]
}
```

