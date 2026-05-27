#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_FILE="$ROOT_DIR/src/bindings.rs"
TMP_FILE="$(mktemp "$ROOT_DIR/src/bindings.rs.tmp.XXXXXX")"
WRAPPER="$ROOT_DIR/wrapper.h"
LIB_DIR="$ROOT_DIR/libswisseph"

cleanup() {
    rm -f "$TMP_FILE"
}
trap cleanup EXIT

if ! command -v bindgen >/dev/null 2>&1; then
    echo "error: bindgen CLI not found"
    echo "install with: cargo install bindgen-cli"
    exit 1
fi

if [[ ! -f "$WRAPPER" ]]; then
    echo "error: wrapper.h not found: $WRAPPER"
    exit 1
fi

if [[ ! -d "$LIB_DIR" ]]; then
    echo "error: libswisseph directory not found: $LIB_DIR"
    echo "run: git submodule update --init --recursive"
    exit 1
fi

if [[ ! -f "$LIB_DIR/swephexp.h" ]]; then
    echo "error: libswisseph looks incomplete: missing $LIB_DIR/swephexp.h"
    echo "run: git submodule update --init --recursive"
    exit 1
fi

COMMON_ARGS=(
    "$WRAPPER"
    --output "$TMP_FILE"
    --no-layout-tests
)

BLOCKLIST_ARGS=(
    --blocklist-item "FP_NAN"
    --blocklist-item "FP_INFINITE"
    --blocklist-item "FP_ZERO"
    --blocklist-item "FP_SUBNORMAL"
    --blocklist-item "FP_NORMAL"

    --blocklist-function "__.*"
    --blocklist-function "scalbl"
    --blocklist-function "fmal"
    --blocklist-function "fminl"
    --blocklist-function "fmaxl"
    --blocklist-function "fdiml"
    --blocklist-function "llroundl"
    --blocklist-function "llrintl"
    --blocklist-function "remquol"
    --blocklist-function "truncl"
    --blocklist-function "roundl"
    --blocklist-function "nearbyintl"
    --blocklist-function "scalblnl"
    --blocklist-function "scalbnl"
    --blocklist-function "ilogbl"
    --blocklist-function "lroundl"
    --blocklist-function "lrintl"
    --blocklist-function "remainderl"
    --blocklist-function "nexttowardl"
    --blocklist-function "nextafterl"
    --blocklist-function "rintl"
    --blocklist-function "lgammal_r"
    --blocklist-function "gammal"
    --blocklist-function "tgammal"
    --blocklist-function "lgammal"
    --blocklist-function "erfcl"
    --blocklist-function "erfl"
    --blocklist-function "ynl"
    --blocklist-function "y1l"
    --blocklist-function "y0l"
    --blocklist-function "jnl"
    --blocklist-function "j1l"
    --blocklist-function "j0l"
    --blocklist-function "isnanl"
    --blocklist-function "nanl"
    --blocklist-function "copysignl"
    --blocklist-function "significandl"
    --blocklist-function "dreml"
    --blocklist-function "finitel"
    --blocklist-function "isinfl"
    --blocklist-function "fmodl"
    --blocklist-function "floorl"
    --blocklist-function "fabsl"
    --blocklist-function "ceill"
    --blocklist-function "cbrtl"
    --blocklist-function "hypotl"
    --blocklist-function "sqrtl"
    --blocklist-function "powl"
    --blocklist-function "log2l"
    --blocklist-function "exp2l"
    --blocklist-function "logbl"
    --blocklist-function "log1pl"
    --blocklist-function "expm1l"
    --blocklist-function "modfl"
    --blocklist-function "log10l"
    --blocklist-function "logl"
    --blocklist-function "ldexpl"
    --blocklist-function "frexpl"
    --blocklist-function "expl"
    --blocklist-function "atanhl"
    --blocklist-function "asinhl"
    --blocklist-function "acoshl"
    --blocklist-function "tanhl"
    --blocklist-function "sinhl"
    --blocklist-function "coshl"
    --blocklist-function "tanl"
    --blocklist-function "sinl"
    --blocklist-function "cosl"
    --blocklist-function "atan2l"
    --blocklist-function "atanl"
    --blocklist-function "acosl"
    --blocklist-function "nexttowardf"
    --blocklist-function "asinl"
    --blocklist-function "nexttoward"
    --blocklist-function "strtold"
    --blocklist-function "qecvt_r"
    --blocklist-function "qfcvt_r"
    --blocklist-function "qgcvt"
    --blocklist-function "ecvt_r"
    --blocklist-function "qfcvt"
    --blocklist-function "qecvt"
    --blocklist-function "fcvt_r"
    --blocklist-function "wctomb"
    --blocklist-function "mblen"
)

CLANG_ARGS=(
    "-I$LIB_DIR"
    "-I$ROOT_DIR"
    "-I$LIB_DIR/vfs"
    "-DUSECASE=2"
    "-D_FILE_OFFSET_BITS=64"
)

if [[ "${DEFINE_NO_SWE_GLP:-1}" == "1" ]]; then
    CLANG_ARGS+=("-DNO_SWE_GLP")
fi

# Optional target mode. For normal native binding generation, you may leave
# CLANG_TARGET unset. For checking wasm-facing headers, use:
#   CLANG_TARGET=wasm32-unknown-unknown ./scripts/regenerate-bindings.sh
if [[ -n "${CLANG_TARGET:-}" ]]; then
    CLANG_ARGS+=("--target=$CLANG_TARGET")
fi

# If using a WASI SDK sysroot, modern wasi-sdk may place libc headers under:
#   $WASI_SYSROOT/include/wasm32-wasip1
# rather than directly under:
#   $WASI_SYSROOT/include
if [[ -n "${WASI_SYSROOT:-}" ]]; then
    CLANG_ARGS+=("--sysroot=$WASI_SYSROOT")

    if [[ -d "$WASI_SYSROOT/include/wasm32-wasip1" ]]; then
        CLANG_ARGS+=("-I$WASI_SYSROOT/include/wasm32-wasip1")
    elif [[ -d "$WASI_SYSROOT/include/wasm32-wasi" ]]; then
        CLANG_ARGS+=("-I$WASI_SYSROOT/include/wasm32-wasi")
    elif [[ -f "$WASI_SYSROOT/include/math.h" ]]; then
        CLANG_ARGS+=("-I$WASI_SYSROOT/include")
    else
        echo "warning: WASI_SYSROOT is set, but no expected libc include directory was found:"
        echo "  $WASI_SYSROOT/include/wasm32-wasip1"
        echo "  $WASI_SYSROOT/include/wasm32-wasi"
        echo "  $WASI_SYSROOT/include/math.h"
    fi
fi

# Keep this optional because the shim is only relevant to the wasm/no-JPL build.
if [[ "${INCLUDE_NO_JPL_SHIM:-0}" == "1" ]]; then
    if [[ -f "$LIB_DIR/se_no_jpl_shim.h" ]]; then
        CLANG_ARGS+=("-include" "$LIB_DIR/se_no_jpl_shim.h")
    else
        echo "error: INCLUDE_NO_JPL_SHIM=1 but missing $LIB_DIR/se_no_jpl_shim.h"
        exit 1
    fi
fi

echo "Generating temporary bindings to $TMP_FILE"
echo "Wrapper: $WRAPPER"
echo "Clang args:"
printf '  %q\n' "${CLANG_ARGS[@]}"

bindgen \
    "${COMMON_ARGS[@]}" \
    "${BLOCKLIST_ARGS[@]}" \
    -- \
    "${CLANG_ARGS[@]}"

FUNCTION_COUNT="$(grep -c 'pub fn ' "$TMP_FILE" || true)"

if [[ "$FUNCTION_COUNT" -lt 20 ]]; then
    echo "error: generated bindings look incomplete ($FUNCTION_COUNT functions)"
    echo "keeping existing $OUT_FILE unchanged"
    exit 1
fi

if ! grep -q 'pub struct swe_vfs_api' "$TMP_FILE"; then
    echo "error: generated bindings are missing pub struct swe_vfs_api"
    echo "make sure wrapper.h includes: #include \"libswisseph/vfs/swevfs.h\""
    echo "keeping existing $OUT_FILE unchanged"
    exit 1
fi

if ! grep -q 'pub fn swe_set_vfs_api' "$TMP_FILE"; then
    echo "error: generated bindings are missing pub fn swe_set_vfs_api"
    echo "make sure wrapper.h includes: #include \"libswisseph/vfs/swevfs.h\""
    echo "keeping existing $OUT_FILE unchanged"
    exit 1
fi

mv "$TMP_FILE" "$OUT_FILE"
trap - EXIT

echo "Updated $OUT_FILE ($FUNCTION_COUNT functions)"
