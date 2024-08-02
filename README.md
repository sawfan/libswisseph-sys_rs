# libswisseph-sys

Rust system wrapper for the swisseph C library

## Ephemeris files

Ephemeris files are excluded from this crate so that it fits on crates.io. 
There are standard standard ephemeris files located in the swisseph c library
and there are many more that can be included depending on the desired calculation.
For example: astroids, JPL, etc.

The ephemeris path with default to and can be set as described in the swisseph C library.
See https://github.com/aloistr/swisseph/ for further details.

## Example usage

```rust
use libswisseph_sys::*;
let mut xx: [f64; 6] = [0.0; 6];
let mut serr = [0; 256];

let y = 2002;
let m = 1;
let d = 1;
let h = 0.0;
let i_flag = SE_GREG_CAL as i32;

let tjd_ut = swe_julday(y, m, d, h, i_flag);
let ipl = SE_SUN as i32;
swe_calc_ut(
    tjd_ut, 
    ipl, 
    SEFLG_SPEED as i32, 
    xx.as_mut_ptr(), 
    serr.as_mut_ptr(),
);

let _lng = xx[0];
let _lat = xx[1];
let _speed = xx[3];
```

## TODO
* See if bindgen can generate more ergonomic integer types. There is some discrepancy 
  between signed and unsigned types for functions and constants.
* There are a couple of header files that are not included because of overlapping function
  names
 
## swisseph docs

https://www.astro.com/swisseph/swephprg.htm
https://www.astro.com/ftp/swisseph/doc/swisseph.pdf

## Credit

 * Astrodienst / @aloistr for swisseph
 * Stéphane Bressani / @stephaneworkspace for work on his wrapper and rust project

## Notes

### Added the swisseph c lib as submodule

git submodule add https://github.com/aloistr/swisseph.git libswisseph/

### u128 warnings

Rustc u128 is not ffi safe. Warnings may be thrown on compilation
