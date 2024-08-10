use crate::raw;
use crate::raw::centisec;
use crate::util::*;
//use crate::types::RAW_OK;

// 
// These functions just pass simple data types and don't modify anything in place.
// They only return new values
//

pub unsafe fn swe_julday(
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
    gregflag: i32,
) -> f64 {
    raw::swe_julday(
        year,
        month,
        day,
        hour,
        gregflag,
    )
}


pub unsafe fn swe_set_topo(geolon: f64, geolat: f64, geoalt: f64) {
    raw::swe_set_topo(geolon, geolat, geoalt)   
}

pub unsafe fn swe_set_sid_mode(sid_mode: i32, t0: f64, ayan_t0: f64) {
    raw::swe_set_sid_mode(sid_mode, t0, ayan_t0)
}

pub unsafe fn swe_get_ayanamsa(tjd_et: f64) -> f64 {
    raw::swe_get_ayanamsa(tjd_et)
}

pub unsafe fn swe_get_ayanamsa_ut(tjd_ut: f64) -> f64 {
    raw::swe_get_ayanamsa_ut(tjd_ut)
}

pub unsafe fn swe_deltat(tjd: f64) -> f64 {
    raw::swe_deltat(tjd)
}

pub unsafe fn swe_sidtime0(tjd_ut: f64, eps: f64, nut: f64) -> f64 {
    raw::swe_sidtime0(tjd_ut, eps, nut)
}

pub unsafe fn swe_sidtime(tjd_ut: f64) -> f64 {
    raw::swe_sidtime(tjd_ut)
}

pub unsafe fn swe_set_interpolate_nut(do_interpolate: bool) {
    let b = bool_to_as_bool(do_interpolate);
    raw::swe_set_interpolate_nut(b)
}

pub unsafe fn swe_get_tid_acc() -> f64 {
    raw::swe_get_tid_acc()
}

pub unsafe fn swe_set_tid_acc(t_acc: f64) {
    raw::swe_set_tid_acc(t_acc) 
}

pub unsafe fn swe_set_delta_t_userdef(dt: f64) {
    raw::swe_set_delta_t_userdef(dt)
}

pub unsafe fn swe_degnorm(x: f64) -> f64 {
    raw::swe_degnorm(x)
}

pub unsafe fn swe_radnorm(x: f64) -> f64 {
    raw::swe_radnorm(x)
}

pub unsafe fn swe_rad_midp(x1: f64, x0: f64) -> f64 {
    raw::swe_rad_midp(x1, x0)
}

pub unsafe fn swe_deg_midp(x1: f64, x0: f64) -> f64 {
    raw::swe_deg_midp(x1, x0)
}

pub unsafe fn swe_difdeg2n(p1: f64, p2: f64) -> f64 {
    raw::swe_difdeg2n(p1, p2)
}

pub unsafe fn swe_difrad2n(p1: f64, p2: f64) -> f64 {
    raw::swe_difrad2n(p1, p2)
}

pub unsafe fn swe_csroundsec(x: centisec) -> centisec {
    raw::swe_csroundsec(x)
}

pub unsafe fn swe_d2l(x: f64) -> i32 {
    raw::swe_d2l(x)
}

pub unsafe fn swe_day_of_week(jd: f64) -> i32 {
    raw::swe_day_of_week(jd)
}

pub unsafe fn swe_csnorm(p: centisec) -> centisec {
    raw::swe_csnorm(p)
}

pub unsafe fn swe_difcsn(p1: centisec, p2: centisec) -> centisec {
    raw::swe_difcsn(p1, p2)
}

pub unsafe fn swe_difdegn(p1: f64, p2: f64) -> f64 {
    raw::swe_difdegn(p1, p2)
}

pub unsafe fn swe_difcs2n(p1: centisec, p2: centisec) -> centisec {
    raw::swe_difcs2n(p1, p2)
}

