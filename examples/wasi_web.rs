//! WASI-in-the-browser Swiss Ephemeris demo.
//!
//! This example is intended to be compiled to `wasm32-wasip1` and loaded by the
//! `examples/wasi_web/web` Trunk runner, which uses a JavaScript WASI shim.
//!
//! The module exports a small C ABI surface that the JS host calls.

use std::cell::RefCell;
use std::ffi::{c_char, CStr, CString};

use libswisseph_sys::raw;

thread_local! {
    static LAST_ERROR: RefCell<[u8; 512]> = RefCell::new([0; 512]);
}

fn set_last_error(msg: &str) {
    LAST_ERROR.with(|buf| {
        let mut buf = buf.borrow_mut();
        buf.fill(0);

        let bytes = msg.as_bytes();
        let n = bytes.len().min(buf.len().saturating_sub(1));
        buf[..n].copy_from_slice(&bytes[..n]);
    });
}

/// Entry point.
///
/// The browser runner instantiates this module and calls the exported functions
/// directly. The WASI shim may still invoke `_start`, so keep it as a no-op.
fn main() {}

/// Allocate memory inside the wasm module and return a pointer.
///
/// JS can write input bytes into this buffer (e.g. UTF-8 strings).
#[no_mangle]
pub extern "C" fn swisseph_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Deallocate memory previously allocated by `swisseph_alloc`.
#[no_mangle]
pub extern "C" fn swisseph_dealloc(ptr: *mut u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }

    unsafe {
        drop(Vec::<u8>::from_raw_parts(ptr, 0, len));
    }
}

/// Allocate an `f64` buffer inside the wasm module and return a pointer.
///
/// This avoids alignment issues that can happen if JS allocates a `u8` buffer and
/// then reinterprets it as `f64`.
#[no_mangle]
pub extern "C" fn swisseph_alloc_f64(len: usize) -> *mut f64 {
    let mut buf = Vec::<f64>::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Deallocate memory previously allocated by `swisseph_alloc_f64`.
///
/// `len` is in number of `f64` elements (not bytes).
#[no_mangle]
pub extern "C" fn swisseph_dealloc_f64(ptr: *mut f64, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }

    unsafe {
        drop(Vec::<f64>::from_raw_parts(ptr, 0, len));
    }
}

/// Get a pointer to a thread-local, NUL-terminated last error message.
#[no_mangle]
pub extern "C" fn swisseph_last_error_ptr() -> *const u8 {
    LAST_ERROR.with(|buf| buf.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn swisseph_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn swisseph_version_ptr() -> *const u8 {
    static VERSION: &[u8] = b"libswisseph-sys wasi_web example\0";
    VERSION.as_ptr()
}

/// Set Swiss Ephemeris ephemeris path.
///
/// Note: When using `@bjorn3/browser_wasi_shim`, absolute paths are not supported.
/// Use a relative preopen name like "ephe" (not "/ephe").
///
/// Returns 0 on success, -1 on error.
#[no_mangle]
pub extern "C" fn swisseph_set_ephe_path_utf8(ptr: *const u8, len: usize) -> i32 {
    if ptr.is_null() {
        set_last_error("null pointer");
        return -1;
    }

    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let path = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => {
            set_last_error(&format!("invalid utf-8: {e}"));
            return -1;
        }
    };

    let c_path = match CString::new(path) {
        Ok(s) => s,
        Err(_) => {
            set_last_error("path contained NUL byte");
            return -1;
        }
    };

    unsafe {
        raw::swe_set_ephe_path(c_path.as_ptr());
    }

    0
}

/// Convert a UTC date/time into Julian Day (UT).
#[no_mangle]
pub extern "C" fn swisseph_julday_ut(year: i32, month: i32, day: i32, hour_ut: f64) -> f64 {
    unsafe { raw::swe_julday(year, month, day, hour_ut, raw::SE_GREG_CAL as i32) }
}

/// Calculate planetary position using `swe_calc_ut`.
///
/// - `jd_ut`: Julian day (UT)
/// - `ipl`: planet/body id (e.g. SE_SUN, SE_MOON, ...)
/// - `iflag`: Swiss Ephemeris flags (e.g. SEFLG_SWIEPH | SEFLG_SPEED)
/// - `out_ptr`: pointer to at least 6 f64 values
///
/// Returns 0 on success, -1 on error.
#[no_mangle]
pub extern "C" fn swisseph_calc_ut(jd_ut: f64, ipl: i32, iflag: i32, out_ptr: *mut f64) -> i32 {
    if out_ptr.is_null() {
        set_last_error("out_ptr is null");
        return -1;
    }

    let out = unsafe { std::slice::from_raw_parts_mut(out_ptr, 6) };
    let mut serr = [0 as c_char; 512];

    let rc = unsafe { raw::swe_calc_ut(jd_ut, ipl, iflag, out.as_mut_ptr(), serr.as_mut_ptr()) };

    if rc < 0 {
        let msg = unsafe { CStr::from_ptr(serr.as_ptr()) }
            .to_string_lossy()
            .to_string();
        set_last_error(&msg);
        -1
    } else {
        0
    }
}

// Convenience exports for common bodies (avoid JS needing constants).
#[no_mangle]
pub extern "C" fn swisseph_se_sun() -> i32 {
    raw::SE_SUN as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_moon() -> i32 {
    raw::SE_MOON as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_mercury() -> i32 {
    raw::SE_MERCURY as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_venus() -> i32 {
    raw::SE_VENUS as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_mars() -> i32 {
    raw::SE_MARS as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_jupiter() -> i32 {
    raw::SE_JUPITER as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_saturn() -> i32 {
    raw::SE_SATURN as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_uranus() -> i32 {
    raw::SE_URANUS as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_neptune() -> i32 {
    raw::SE_NEPTUNE as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_pluto() -> i32 {
    raw::SE_PLUTO as i32
}

#[no_mangle]
pub extern "C" fn swisseph_seflg_swieph() -> i32 {
    raw::SEFLG_SWIEPH as i32
}

#[no_mangle]
pub extern "C" fn swisseph_seflg_speed() -> i32 {
    raw::SEFLG_SPEED as i32
}
