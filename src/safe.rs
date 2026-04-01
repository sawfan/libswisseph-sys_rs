use crate::raw;
use raw::centisec;
use std::ffi::CStr;

use crate::util::*;

//hsys: ::std::os::raw::c_int,
//ObjectName: *mut ::std::os::raw::c_char,
//serr: *mut ::std::os::raw::c_char,
//

pub fn swe_heliacal_ut(
    tjdstart_ut: f64,
    geopos: *mut f64,
    datm: *mut f64,
    dobs: *mut f64,
    ObjectName: *mut ::std::os::raw::c_char,
    TypeEvent: i32,
    iflag: i32,
    dret: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_heliacal_ut(
            tjdstart_ut,
            geopos,
            datm,
            dobs,
            ObjectName,
            TypeEvent,
            iflag,
            dret,
            serr ,
        )
    }
}

pub fn swe_heliacal_pheno_ut(
    tjd_ut: f64,
    geopos: *mut f64,
    datm: *mut f64,
    dobs: *mut f64,
    ObjectName: *mut ::std::os::raw::c_char,
    TypeEvent: i32,
    helflag: i32,
    darr: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_heliacal_pheno_ut(
            tjd_ut,
            geopos,
            datm,
            dobs,
            ObjectName,
            TypeEvent,
            helflag,
            darr,
            serr,
        )
    }
}

pub fn swe_vis_limit_mag(
    tjdut: f64,
    geopos: *mut f64,
    datm: *mut f64,
    dobs: *mut f64,
    ObjectName: *mut ::std::os::raw::c_char,
    helflag: i32,
    dret: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_vis_limit_mag(
            tjdut,
            geopos,
            datm,
            dobs,
            ObjectName,
            helflag,
            dret,
            serr,
        )
    }
}

pub fn swe_heliacal_angle(
    tjdut: f64,
    dgeo: *mut f64,
    datm: *mut f64,
    dobs: *mut f64,
    helflag: i32,
    mag: f64,
    azi_obj: f64,
    azi_sun: f64,
    azi_moon: f64,
    alt_moon: f64,
    dret: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_heliacal_angle(
            tjdut,
            dgeo,
            datm,
            dobs,
            helflag,
            mag,
            azi_obj,
            azi_sun,
            azi_moon,
            alt_moon,
            dret,
            serr,
        )
    }
}

pub fn swe_topo_arcus_visionis(
    tjdut: f64,
    dgeo: *mut f64,
    datm: *mut f64,
    dobs: *mut f64,
    helflag: i32,
    mag: f64,
    azi_obj: f64,
    alt_obj: f64,
    azi_sun: f64,
    azi_moon: f64,
    alt_moon: f64,
    dret: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_topo_arcus_visionis(
            tjdut,
            dgeo,
            datm,
            dobs,
            helflag,
            mag,
            azi_obj,
            alt_obj,
            azi_sun,
            azi_moon,
            alt_moon,
            dret,
            serr,
        )
    }
}

pub fn swe_set_astro_models(samod: *mut ::std::os::raw::c_char, iflag: i32) {
    unsafe {
        raw::swe_set_astro_models(samod, iflag)
    }
}

pub fn swe_get_astro_models(
    samod: *mut ::std::os::raw::c_char,
    sdet: *mut ::std::os::raw::c_char,
    iflag: i32,
) {
    unsafe {
        raw::swe_get_astro_models(
            samod,
            sdet,
            iflag
        )
    }
}

pub fn swe_version(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char {
    unsafe {
        raw::swe_version(arg1)
    }
}

pub fn swe_get_library_path(
    arg1: *mut ::std::os::raw::c_char
) -> *mut ::std::os::raw::c_char {
    unsafe {
        raw::swe_get_library_path(arg1)
    }
}

pub fn swe_calc(
    tjd: f64,
    ipl: ::std::os::raw::c_int,
    iflag: i32,
    xx: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_calc(
            tjd,
            ipl,
            iflag,
            xx,
            serr,
        )
    }
}

pub fn swe_calc_ut(
    tjd_ut: f64,
    ipl: i32,
    iflag: i32,
    xx: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_calc_ut(
            tjd_ut,
            ipl,
            iflag,
            xx,
            serr
        )
    }
}

pub fn swe_calc_pctr(
    tjd: f64,
    ipl: i32,
    iplctr: i32,
    iflag: i32,
    xxret: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_calc_pctr(
            tjd,
            ipl,
            iplctr,
            iflag,
            xxret,
            serr,
        )
    }
}

pub fn swe_solcross(
    x2cross: f64,
    jd_et: f64,
    flag: i32,
    serr: *mut ::std::os::raw::c_char,
) -> f64 {
    unsafe {
        raw::swe_solcross(
            x2cross,
            jd_et,
            flag,
            serr
        )
    }
}

pub fn swe_solcross_ut(
    x2cross: f64,
    jd_ut: f64,
    flag: i32,
    serr: *mut ::std::os::raw::c_char,
) -> f64 {
    unsafe {
        raw::swe_solcross_ut(
            x2cross,
            jd_ut,
            flag,
            serr,
        )
    }
}

pub fn swe_mooncross(
    x2cross: f64,
    jd_et: f64,
    flag: i32,
    serr: *mut ::std::os::raw::c_char,
) -> f64 {
    unsafe {
        raw::swe_mooncross(
            x2cross,
            jd_et,
            flag,
            serr,
        )
    }
}

pub fn swe_mooncross_ut(
    x2cross: f64,
    jd_ut: f64,
    flag: i32,
    serr: *mut ::std::os::raw::c_char,
) -> f64 {
    unsafe {
        raw::swe_mooncross_ut(
            x2cross,
            jd_ut,
            flag,
            serr,
        )
    }
}

pub fn swe_mooncross_node(
    jd_et: f64,
    flag: i32,
    xlon: *mut f64,
    xlat: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> f64 {
    unsafe {
        raw::swe_mooncross_node(
            jd_et,
            flag,
            xlon,
            xlat,
            serr,
        )
    }
}

pub fn swe_mooncross_node_ut(
    jd_ut: f64,
    flag: i32,
    xlon: *mut f64,
    xlat: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> f64 {
    unsafe {
        raw::swe_mooncross_node_ut(
            jd_ut,
            flag,
            xlon,
            xlat,
            serr,
        )
    }
}

pub fn swe_helio_cross(
    ipl: i32,
    x2cross: f64,
    jd_et: f64,
    iflag: i32,
    dir: i32,
    jd_cross: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_helio_cross(
            ipl,
            x2cross,
            jd_et,
            iflag,
            dir,
            jd_cross,
            serr,
        )
    }
}

pub fn swe_helio_cross_ut(
    ipl: i32,
    x2cross: f64,
    jd_ut: f64,
    iflag: i32,
    dir: i32,
    jd_cross: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_helio_cross_ut(
            ipl,
            x2cross,
            jd_ut,
            iflag,
            dir,
            jd_cross,
            serr,

        )
    }
}

pub fn swe_fixstar(
    star: *mut ::std::os::raw::c_char,
    tjd: f64,
    iflag: i32,
    xx: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_fixstar(
            star,
            tjd,
            iflag,
            xx,
            serr,
        )
    }
}

pub fn swe_fixstar_ut(
    star: *mut ::std::os::raw::c_char,
    tjd_ut: f64,
    iflag: i32,
    xx: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_fixstar_ut(
            star,
            tjd_ut,
            iflag,
            xx,
            serr,
        )
    }
}

pub fn swe_fixstar_mag(
    star: *mut ::std::os::raw::c_char,
    mag: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_fixstar_mag(
            star,
            mag,
            serr,
        )
    }
}

pub fn swe_fixstar2(
    star: *mut ::std::os::raw::c_char,
    tjd: f64,
    iflag: i32,
    xx: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_fixstar2(
            star,
            tjd,
            iflag,
            xx,
            serr,
        )
    }
}

pub fn swe_fixstar2_ut(
    star: *mut ::std::os::raw::c_char,
    tjd_ut: f64,
    iflag: i32,
    xx: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_fixstar2_ut(
            star,
            tjd_ut,
            iflag,
            xx,
            serr,
        )
    }
}

pub fn swe_fixstar2_mag(
    star: *mut ::std::os::raw::c_char,
    mag: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_fixstar2_mag(
            star,
            mag,
            serr,
        )
    }
}


pub fn swe_close() {
    unsafe {
        raw::swe_close()
    }
}

pub fn swe_set_ephe_path(path: *const ::std::os::raw::c_char) {
    unsafe {
        raw::swe_set_ephe_path(path) 
    }
}


//    pub fn swe_set_jpl_file(fname: *const ::std::os::raw::c_char);


//    pub fn swe_get_planet_name(
//        ipl: ::std::os::raw::c_int,
//        spname: *mut ::std::os::raw::c_char,
//    ) -> *mut ::std::os::raw::c_char;


//    pub fn swe_set_topo(geolon: f64, geolat: f64, geoalt: f64);


//    pub fn swe_set_sid_mode(sid_mode: i32, t0: f64, ayan_t0: f64);


//    pub fn swe_get_ayanamsa_ex(
//        tjd_et: f64,
//        iflag: i32,
//        daya: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> i32;


//    pub fn swe_get_ayanamsa_ex_ut(
//        tjd_ut: f64,
//        iflag: i32,
//        daya: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> i32;


//    pub fn swe_get_ayanamsa(tjd_et: f64) -> f64;
//}

//    pub fn swe_get_ayanamsa_ut(tjd_ut: f64) -> f64;


//    pub fn swe_get_ayanamsa_name(isidmode: i32) -> *const ::std::os::raw::c_char;


pub fn swe_get_current_file_data(
    ifno: ::std::os::raw::c_int,
    tfstart: *mut f64,
    tfend: *mut f64,
    denum: *mut ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    unsafe {
        raw::swe_get_current_file_data(
            ifno,
            tfstart,
            tfend,
            denum,
        )
    }
}

pub fn swe_date_conversion(
    y: i32,
    m: i32,
    d: i32,
    utime: f64,
    c: ::std::os::raw::c_char,
    tjd: *mut f64,
) -> i32 {
    unsafe {
        raw::swe_date_conversion(
            y,
            m,
            d,
            utime,
            c,
            tjd,
        )
    }
}

pub fn swe_julday(
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
    gregflag: i32,
) -> f64 {
    unsafe {
        raw::swe_julday(
            year,
            month,
            day,
            hour,
            gregflag,
        )
    }
}

pub fn swe_revjul(
    jd: f64,
    gregflag: i32,
    jyear: *mut i32,
    jmon: *mut i32,
    jday: *mut i32,
    jut: *mut f64,
) {
    unsafe {
        raw::swe_revjul(
            jd,
            gregflag,
            jyear,
            jmon,
            jday,
            jut,
        )
    }
}

pub fn swe_utc_to_jd(
    iyear: i32,
    imonth: i32,
    iday: i32,
    ihour: i32,
    imin: i32,
    dsec: f64,
    gregflag: i32,
    dret: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_utc_to_jd(
            iyear,
            imonth,
            iday,
            ihour,
            imin,
            dsec,
            gregflag,
            dret,
            serr,
        )
    }
}

pub fn swe_jdet_to_utc(
    tjd_et: f64,
    gregflag: i32,
    iyear: *mut i32,
    imonth: *mut i32,
    iday: *mut i32,
    ihour: *mut i32,
    imin: *mut i32,
    dsec: *mut f64,
) {
    unsafe {
        raw::swe_jdet_to_utc(
            tjd_et,
            gregflag,
            iyear,
            imonth,
            iday,
            ihour,
            imin,
            dsec,
        )
    }
}

pub fn swe_jdut1_to_utc(
    tjd_ut: f64,
    gregflag: i32,
    iyear: *mut i32,
    imonth: *mut i32,
    iday: *mut i32,
    ihour: *mut i32,
    imin: *mut i32,
    dsec: *mut f64,
) {
    unsafe {
        raw::swe_jdut1_to_utc(
            tjd_ut,
            gregflag,
            iyear,
            imonth,
            iday,
            ihour,
            imin,
            dsec,
        )
    }
}

pub fn swe_utc_time_zone(
    iyear: i32,
    imonth: i32,
    iday: i32,
    ihour: i32,
    imin: i32,
    dsec: f64,
    d_timezone: f64,
    iyear_out: *mut i32,
    imonth_out: *mut i32,
    iday_out: *mut i32,
    ihour_out: *mut i32,
    imin_out: *mut i32,
    dsec_out: *mut f64,
) {
    unsafe {
        raw::swe_utc_time_zone(
            iyear,
            imonth,
            iday,
            ihour,
            imin,
            dsec,
            d_timezone,
            iyear_out,
            imonth_out,
            iday_out,
            ihour_out,
            imin_out,
            dsec_out,
        )
    }
}

pub fn swe_houses(
    tjd_ut: f64,
    geolat: f64,
    geolon: f64,
    hsys: i32,
    cusps: *mut f64,
    ascmc: *mut f64,
) -> ::std::os::raw::c_int {
    unsafe {
        raw::swe_houses(
            tjd_ut,
            geolat,
            geolon,
            hsys,
            cusps,
            ascmc,
        )
    }
}

pub fn swe_houses_ex(
    tjd_ut: f64,
    iflag: i32,
    geolat: f64,
    geolon: f64,
    hsys: i32,
    cusps: *mut f64,
    ascmc: *mut f64,
) -> i32 {
    unsafe {
        raw::swe_houses_ex(
            tjd_ut,
            iflag,
            geolat,
            geolon,
            hsys,
            cusps,
            ascmc,
        )
    }
}

pub fn swe_houses_ex2(
    tjd_ut: f64,
    iflag: i32,
    geolat: f64,
    geolon: f64,
    hsys: i32,
    cusps: *mut f64,
    ascmc: *mut f64,
    cusp_speed: *mut f64,
    ascmc_speed: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_houses_ex2(
            tjd_ut,
            iflag,
            geolat,
            geolon,
            hsys,
            cusps,
            ascmc,
            cusp_speed,
            ascmc_speed,
            serr,
        )
    }
}

pub fn swe_houses_armc(
    armc: f64,
    geolat: f64,
    eps: f64,
    hsys: ::std::os::raw::c_int,
    cusps: *mut f64,
    ascmc: *mut f64,
) -> i32 {
    unsafe {
        raw::swe_houses_armc(
            armc,
            geolat,
            eps,
            hsys,
            cusps,
            ascmc,
        )
    }
}

//return ::std::os::raw::c_int
pub fn swe_houses_armc_ex2(
    armc: f64,
    geolat: f64,
    eps: f64,
    hsys: i32,
    cusps: *mut f64,
    ascmc: *mut f64,
    cusp_speed: *mut f64,
    ascmc_speed: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_houses_armc_ex2(
            armc,
            geolat,
            eps,
            hsys,
            cusps,
            ascmc,
            cusp_speed,
            ascmc_speed,
            serr,
        )
    }
}

pub fn swe_house_pos(
    armc: f64,
    geolat: f64,
    eps: f64,
    hsys: i32,
    xpin: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> f64 {
    unsafe {
        raw::swe_house_pos(
            armc,
            geolat,
            eps,
            hsys,
            xpin,
            serr
        )
    }
}

pub fn swe_house_name(hsys: i32) -> String {
    unsafe {
        let house_name = raw::swe_house_name(hsys);
        CStr::from_ptr(house_name).to_str().unwrap().to_string()
    }
}

pub fn swe_gauquelin_sector(
    t_ut: f64,
    ipl: i32,
    starname: *mut ::std::os::raw::c_char,
    iflag: i32,
    imeth: i32,
    geopos: *mut f64,
    atpress: f64,
    attemp: f64,
    dgsect: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_gauquelin_sector(
            t_ut,
            ipl,
            starname,
            iflag,
            imeth,
            geopos,
            atpress,
            attemp,
            dgsect,
            serr,
        )
    }
}

pub fn swe_sol_eclipse_where(
    tjd: f64,
    ifl: i32,
    geopos: *mut f64,
    attr: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_sol_eclipse_where(
            tjd,
            ifl,
            geopos,
            attr,
            serr,
        )
    }
}

pub fn swe_lun_occult_where(
    tjd: f64,
    ipl: i32,
    starname: *mut ::std::os::raw::c_char,
    ifl: i32,
    geopos: *mut f64,
    attr: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_lun_occult_where(
            tjd,
            ipl,
            starname,
            ifl,
            geopos,
            attr,
            serr,
        )
    }
}

pub fn swe_sol_eclipse_how(
    tjd: f64,
    ifl: i32,
    geopos: *mut f64,
    attr: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_sol_eclipse_how(
            tjd,
            ifl,
            geopos,
            attr,
            serr,
        )
    }
}

pub fn swe_sol_eclipse_when_loc(
    tjd_start: f64,
    ifl: i32,
    geopos: *mut f64,
    tret: *mut f64,
    attr: *mut f64,
    backward: i32,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_sol_eclipse_when_loc(
            tjd_start,
            ifl,
            geopos,
            tret,
            attr,
            backward,
            serr,
        )
    }
}

pub fn swe_lun_occult_when_loc(
    tjd_start: f64,
    ipl: i32,
    starname: *mut ::std::os::raw::c_char,
    ifl: i32,
    geopos: *mut f64,
    tret: *mut f64,
    attr: *mut f64,
    backward: i32,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_lun_occult_when_loc(
            tjd_start,
            ipl,
            starname,
            ifl,
            geopos,
            tret,
            attr,
            backward,
            serr,
        )
    }
}

pub fn swe_sol_eclipse_when_glob(
    tjd_start: f64,
    ifl: i32,
    ifltype: i32,
    tret: *mut f64,
    backward: i32,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_sol_eclipse_when_glob(
            tjd_start,
            ifl,
            ifltype,
            tret,
            backward,
            serr,
        )
    }
}

pub fn swe_lun_occult_when_glob(
    tjd_start: f64,
    ipl: i32,
    starname: *mut ::std::os::raw::c_char,
    ifl: i32,
    ifltype: i32,
    tret: *mut f64,
    backward: i32,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_lun_occult_when_glob(
            tjd_start,
            ipl,
            starname,
            ifl,
            ifltype,
            tret,
            backward,
            serr,
        )
    }
}

pub fn swe_lun_eclipse_how(
    tjd_ut: f64,
    ifl: i32,
    geopos: *mut f64,
    attr: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_lun_eclipse_how(
            tjd_ut,
            ifl,
            geopos,
            attr,
            serr,
        )
    }
}

pub fn swe_lun_eclipse_when(
    tjd_start: f64,
    ifl: i32,
    ifltype: i32,
    tret: *mut f64,
    backward: i32,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_lun_eclipse_when(
            tjd_start,
            ifl,
            ifltype,
            tret,
            backward,
            serr,
        )
    }
}

pub fn swe_lun_eclipse_when_loc(
    tjd_start: f64,
    ifl: i32,
    geopos: *mut f64,
    tret: *mut f64,
    attr: *mut f64,
    backward: i32,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_lun_eclipse_when_loc(
            tjd_start,
            ifl,
            geopos,
            tret,
            attr,
            backward,
            serr,
        )
    }
}

pub fn swe_pheno(
    tjd: f64,
    ipl: i32,
    iflag: i32,
    attr: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_pheno(
            tjd,
            ipl,
            iflag,
            attr,
            serr,
        )
    }
}

pub fn swe_pheno_ut(
    tjd_ut: f64,
    ipl: i32,
    iflag: i32,
    attr: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_pheno_ut(
            tjd_ut,
            ipl,
            iflag,
            attr,
            serr,
        )
    }
}

pub fn swe_refrac(inalt: f64, atpress: f64, attemp: f64, calc_flag: i32) -> f64 {
    unsafe {
        raw::swe_refrac(inalt, atpress, attemp, calc_flag)
    }
}

pub fn swe_refrac_extended(
    inalt: f64,
    geoalt: f64,
    atpress: f64,
    attemp: f64,
    lapse_rate: f64,
    calc_flag: i32,
    dret: *mut f64,
) -> f64 {
    unsafe {
        raw::swe_refrac_extended(
            inalt,
            geoalt,
            atpress,
            attemp,
            lapse_rate,
            calc_flag,
            dret,
        )
    }

}

pub fn swe_set_lapse_rate(lapse_rate: f64) {
    unsafe {
        raw::swe_set_lapse_rate(lapse_rate)
    }
}

pub fn swe_azalt(
    tjd_ut: f64,
    calc_flag: i32,
    geopos: *mut f64,
    atpress: f64,
    attemp: f64,
    xin: *mut f64,
    xaz: *mut f64,
) {
    unsafe {
        raw::swe_azalt(
            tjd_ut,
            calc_flag,
            geopos,
            atpress,
            attemp,
            xin,
            xaz,
        )
    }
}

pub fn swe_azalt_rev(
    tjd_ut: f64,
    calc_flag: i32,
    geopos: *mut f64,
    xin: *mut f64,
    xout: *mut f64,
) {
    unsafe {
        raw::swe_azalt_rev(
            tjd_ut,
            calc_flag,
            geopos,
            xin,
            xout,
        )
    }
}

pub fn swe_rise_trans_true_hor(
    tjd_ut: f64,
    ipl: i32,
    starname: *mut ::std::os::raw::c_char,
    epheflag: i32,
    rsmi: i32,
    geopos: *mut f64,
    atpress: f64,
    attemp: f64,
    horhgt: f64,
    tret: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_rise_trans_true_hor(
            tjd_ut,
            ipl,
            starname,
            epheflag,
            rsmi,
            geopos,
            atpress,
            attemp,
            horhgt,
            tret,
            serr,
        )
    }
}

pub fn swe_rise_trans(
    tjd_ut: f64,
    ipl: i32,
    starname: *mut ::std::os::raw::c_char,
    epheflag: i32,
    rsmi: i32,
    geopos: *mut f64,
    atpress: f64,
    attemp: f64,
    tret: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_rise_trans(
            tjd_ut,
            ipl,
            starname,
            epheflag,
            rsmi,
            geopos,
            atpress,
            attemp,
            tret,
            serr,
        )
    }
}

pub fn swe_nod_aps(
    tjd_et: f64,
    ipl: i32,
    iflag: i32,
    method: i32,
    xnasc: *mut f64,
    xndsc: *mut f64,
    xperi: *mut f64,
    xaphe: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_nod_aps(
            tjd_et,
            ipl,
            iflag,
            method,
            xnasc,
            xndsc,
            xperi,
            xaphe,
            serr,
        )
    }
}

pub fn swe_nod_aps_ut(
    tjd_ut: f64,
    ipl: i32,
    iflag: i32,
    method: i32,
    xnasc: *mut f64,
    xndsc: *mut f64,
    xperi: *mut f64,
    xaphe: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_nod_aps_ut(
            tjd_ut,
            ipl,
            iflag,
            method,
            xnasc,
            xndsc,
            xperi,
            xaphe,
            serr,
        )
    }
}

pub fn swe_get_orbital_elements(
    tjd_et: f64,
    ipl: i32,
    iflag: i32,
    dret: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_get_orbital_elements(
            tjd_et,
            ipl,
            iflag,
            dret,
            serr)
    }
}

pub fn swe_orbit_max_min_true_distance(
    tjd_et: f64,
    ipl: i32,
    iflag: i32,
    dmax: *mut f64,
    dmin: *mut f64,
    dtrue: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_orbit_max_min_true_distance(
            tjd_et,
            ipl,
            iflag,
            dmax,
            dmin,
            dtrue,
            serr)
    }
}

pub fn swe_deltat(tjd: f64) -> f64 {
    unsafe {
        raw::swe_deltat(tjd)
    }
}

pub fn swe_deltat_ex(tjd: f64, iflag: i32, serr: *mut ::std::os::raw::c_char) -> f64 {
    unsafe {
        raw::swe_deltat_ex(tjd, iflag, serr)
    }
}

pub fn swe_time_equ(tjd: f64, te: *mut f64, serr: *mut ::std::os::raw::c_char) -> i32 {
    unsafe {
        raw::swe_time_equ(tjd, te, serr)
    }
}

pub fn swe_lmt_to_lat(
    tjd_lmt: f64,
    geolon: f64,
    tjd_lat: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {

    unsafe {
        raw::swe_lmt_to_lat(
            tjd_lmt,
            geolon,
            tjd_lat,
            serr,
        )
    }
}

pub fn swe_lat_to_lmt(
    tjd_lat: f64,
    geolon: f64,
    tjd_lmt: *mut f64,
    serr: *mut ::std::os::raw::c_char,
) -> i32 {
    unsafe {
        raw::swe_lat_to_lmt(
            tjd_lat,
            geolon,
            tjd_lmt,
            serr,
        )
    }
}

pub fn swe_sidtime0(tjd_ut: f64, eps: f64, nut: f64) -> f64 {
    unsafe {
        raw::swe_sidtime0(tjd_ut, eps, nut)
    }
}

pub fn swe_sidtime(tjd_ut: f64) -> f64 {
    unsafe {
        raw::swe_sidtime(tjd_ut)
    }
}

pub fn swe_set_interpolate_nut(do_interpolate: bool) {
    unsafe {
        let b = bool_to_as_bool(do_interpolate);
        raw::swe_set_interpolate_nut(b)
    }
}

pub fn swe_cotrans(xpo: *mut f64, xpn: *mut f64, eps: f64) {
    unsafe {
        raw::swe_cotrans(xpo, xpn, eps)
    }
}

pub fn swe_cotrans_sp(xpo: *mut f64, xpn: *mut f64, eps: f64) {
    unsafe {
        raw::swe_cotrans_sp(xpo, xpn, eps)
    }
}

pub fn swe_get_tid_acc() -> f64 {
    unsafe {
        raw::swe_get_tid_acc()
    }
}

pub fn swe_set_tid_acc(t_acc: f64) {
    unsafe {
        raw::swe_set_tid_acc(t_acc) 
    }
}

pub fn swe_set_delta_t_userdef(dt: f64) {
    unsafe {
        raw::swe_set_delta_t_userdef(dt)
    }
}

pub fn swe_degnorm(x: f64) -> f64 {
    unsafe {
        raw::swe_degnorm(x)
    }
}

pub fn swe_radnorm(x: f64) -> f64 {
    unsafe {
        raw::swe_radnorm(x)
    }
}

pub fn swe_rad_midp(x1: f64, x0: f64) -> f64 {
    unsafe {
        raw::swe_rad_midp(x1, x0)
    }
}

pub fn swe_deg_midp(x1: f64, x0: f64) -> f64 {
    unsafe {
        raw::swe_deg_midp(x1, x0)
    }
}

pub fn swe_split_deg(
    ddeg: f64,
    roundflag: i32,
    ideg: *mut i32,
    imin: *mut i32,
    isec: *mut i32,
    dsecfr: *mut f64,
    isgn: *mut i32,
) {
    unsafe {
        raw::swe_split_deg(
            ddeg,
            roundflag,
            ideg,
            imin,
            isec,
            dsecfr,
            isgn,
        )
    }
}

pub unsafe fn swe_csnorm(p: centisec) -> centisec {
    unsafe {
        raw::swe_csnorm(p)
    }
}

pub fn swe_difcsn(p1: centisec, p2: centisec) -> centisec {
    unsafe {
        raw::swe_difcsn(p1, p2)
    }
}

pub fn swe_difdegn(p1: f64, p2: f64) -> f64 {
    unsafe {
        raw::swe_difdegn(p1, p2)
    }
}

pub unsafe fn swe_difcs2n(p1: centisec, p2: centisec) -> centisec {
    unsafe {
        raw::swe_difcs2n(p1, p2)
    }
}

pub fn swe_difdeg2n(p1: f64, p2: f64) -> f64 {
    unsafe {
        raw::swe_difdeg2n(p1, p2)
    }
}

pub unsafe fn swe_difrad2n(p1: f64, p2: f64) -> f64 {
    unsafe {
        raw::swe_difrad2n(p1, p2)
    }
}

pub fn swe_csroundsec(x: centisec) -> centisec {
    unsafe {
        raw::swe_csroundsec(x)
    }
}

pub fn swe_d2l(x: f64) -> i32 {
    unsafe {
        raw::swe_d2l(x)
    }
}

pub fn swe_day_of_week(jd: f64) -> i32 {
    unsafe {
        raw::swe_day_of_week(jd)
    }
}

pub fn swe_cs2timestr(
    t: centisec,
    sep: i32,
    suppressZero: bool,
    a: *mut ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_char {
    unsafe {
        let suppressZero = bool_to_as_bool(suppressZero);
        raw::swe_cs2timestr(
            t,
            sep,
            suppressZero,
            a,
        )
    }
}

pub fn swe_cs2lonlatstr(
    t: centisec,
    pchar: ::std::os::raw::c_char,
    mchar: ::std::os::raw::c_char,
    s: *mut ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_char {
    unsafe {
        raw::swe_cs2lonlatstr(
            t,
            pchar,
            mchar,
            s,
        )
    }
}

pub fn swe_cs2degstr(
    t: centisec,
    a: *mut ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_char {
    unsafe {
        raw::swe_cs2degstr(
            t,
            a,
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    //use std::ffi::CString;
    //use std::os::raw::c_char;

    #[test]
    fn test_swe_house_name() {
        let hn = swe_house_name(1);
        assert_eq!(hn, "Placidus");
    }

}

