//! Minimal WASI smoke test for Swiss Ephemeris.
//!
//! This example expects that the ephemeris directory from this repo is available
//! to the WASI module at runtime. With Wasmtime you can provide it with:
//!
//!   wasmtime run --dir libswisseph/ephe::/eph target/wasm32-wasip1/debug/examples/wasi_smoke.wasm
//!
//! (Or use wasm32-wasi output path if you compile for that target.)

use std::ffi::{CStr, CString};

use libswisseph_sys::raw;

fn main() {
    // Point Swiss Ephemeris at the directory where ephemeris files are located.
    // In WASI, this path must exist inside the WASI sandbox.
    let eph_path = CString::new("/eph").expect("CString");

    // Compute something that requires ephemeris file access.
    // J2000 = 2451545.0
    let tjd = 2451545.0_f64;
    let ipl = raw::SE_SUN as i32;
    let iflag = (raw::SEFLG_SWIEPH | raw::SEFLG_SPEED) as i32;

    let mut xx = [0.0_f64; 6];
    let mut serr = [0_i8; raw::AS_MAXCH as usize];

    let rc = unsafe {
        raw::swe_set_ephe_path(eph_path.as_ptr());
        raw::swe_calc(tjd, ipl, iflag, xx.as_mut_ptr(), serr.as_mut_ptr())
    };

    if rc < 0 {
        // serr is a C string, but we don't depend on it being valid UTF-8.
        let msg = unsafe { CStr::from_ptr(serr.as_ptr()) };
        eprintln!("swe_calc failed: {}", msg.to_string_lossy());
        std::process::exit(1);
    }

    // Print ecliptic longitude/latitude/distance.
    println!("rc={rc} lon={} lat={} dist={}", xx[0], xx[1], xx[2]);
}
