use crate::raw;
use std::ffi::CStr;
use crate::types::*;
use crate::util::*;

// IN THIS SECTION:
// * Hide C baggage, Use Primitive Rust types in calls
// * Create data inside of function
// * Return result of tuples of data
//
//hsys: ::std::os::raw::c_int,
//ObjectName: *mut ::std::os::raw::c_char,
//serr: *mut ::std::os::raw::c_char,
//

pub unsafe fn swe_heliacal_ut(
    tjdstart_ut: f64,
    geopos: *mut f64,
    datm: *mut f64,
    dobs: *mut f64,
    object_name: &str,
    type_event: i32,
    iflag: i32,
) -> Result<(i32, [f64;50]), (i32, String)> {
    let mut object_name_buffer = new_max_buffer_from_str(object_name);
    // result: array of at least 50 doubles, of which 3 are used at the moment
    let mut dret: [f64;50] = [0.;50];
    let mut serr = new_max_buffer();
    let ret_code = raw::swe_heliacal_ut(
        tjdstart_ut,
        geopos,
        datm,
        dobs,
        object_name_buffer.as_mut_ptr(),
        type_event,
        iflag,
        dret.as_mut_ptr(),
        serr.as_mut_ptr(),
    );

    if ret_code < RAW_OK {
        return Err((ret_code, buffer_to_string(serr)))
    } 

    Ok((ret_code, dret))
}

// //The return array has the following data:
// '0=AltO        [deg]     topocentric altitude of object (unrefracted)
// '1=AppAltO     [deg]     apparent altitude of object (refracted)
// '2=GeoAltO     [deg]     geocentric altitude of object
// '3=AziO        [deg]     azimuth of object
// '4=AltS        [deg]     topocentric altitude of Sun
// '5=AziS        [deg]     azimuth of Sun
// '6=TAVact      [deg]     actual topocentric arcus visionis
// '7=ARCVact     [deg]     actual (geocentric) arcus visionis
// '8=DAZact      [deg]     actual difference between object's and sun's azimuth
// '9=ARCLact     [deg]     actual longitude difference between object and sun
// '10=kact       [-]       extinction coefficient
// '11=minTAV     [deg]     smallest topocentric arcus visionis
// '12=TfistVR    [JDN]     first time object is visible, according to VR
// '13=TbVR       [JDN      optimum time the object is visible, according to VR
// '14=TlastVR    [JDN]     last time object is visible, according to VR
// '15=TbYallop   [JDN]     best time the object is visible, according to Yallop
// '16=WMoon      [deg]     crescent width of Moon
// '17=qYal       [-]            q-test value of Yallop
// '18=qCrit      [-]            q-test criterion of Yallop
// '19=ParO       [deg]     parallax of object
// '20 Magn       [-]            magnitude of object
// '21=RiseO      [JDN]     rise/set time of object
// '22=RiseS      [JDN]     rise/set time of Sun
// '23=Lag        [JDN]     rise/set time of object minus rise/set time of Sun
// '24=TvisVR     [JDN]     visibility duration
// '25=LMoon      [deg]     crescent length of Moon
// '26=CVAact     [deg]
// '27=Illum      [%]            new
// '28=CVAact     [deg]     new
// '29=MSk        [-]
pub unsafe fn swe_heliacal_pheno_ut(
    tjd_ut: f64,
    geopos: *mut f64,
    datm: *mut f64,
    dobs: *mut f64,
    object_name: &str,
    type_event: i32,
    helflag: i32,
) -> Result<(i32, [f64;50]), (i32, String)> {
    let mut serr = new_max_buffer();
    let mut object_name_buffer = new_max_buffer_from_str(object_name);
    // return array, declare array of 50 doubles 
    let mut darr: [f64;50] = [0.;50];
    let ret_code = raw::swe_heliacal_pheno_ut(
        tjd_ut,
        geopos,
        datm,
        dobs,
        object_name_buffer.as_mut_ptr(),
        type_event,
        helflag,
        darr.as_mut_ptr(),
        serr.as_mut_ptr(),
    );

    if ret_code < RAW_OK {
        return Err((ret_code, buffer_to_string(serr)))
    } 

    Ok((ret_code, darr))
}

//Details for return array dret[] (array of doubles):
//dret[0]: limiting visual magnitude (if dret[0] > magnitude of object, then the object is visible);
//dret[1]: altitude of object;
//dret[2]: azimuth of object;
//dret[3]: altitude of sun;
//dret[4]: azimuth of sun;
//dret[5]: altitude of moon;
//dret[6]: azimuth of moon;
//dret[7]: magnitude of object.
pub unsafe fn swe_vis_limit_mag(
    tjdut: f64,
    geopos: *mut f64,
    datm: *mut f64,
    dobs: *mut f64,
    object_name: &str,
    helflag: i32,
) -> Result<(i32, [f64;8]), (i32, String)> {
    let mut serr = new_max_buffer();
    let mut object_name_buffer = new_max_buffer_from_str(object_name);

    let mut dret: [f64;8] = [0.;8];

    // Function returns:
    //     -1    on error;
    //     -2    object is below horizon;
    //     0     OK, photopic vision;
    //     &1   OK, scotopic vision;
    //     &2   OK, near limit photopic/scotopic vision.
    let ret_code = raw::swe_vis_limit_mag(
        tjdut,
        geopos,
        datm,
        dobs,
        object_name_buffer.as_mut_ptr(),
        helflag,
        dret.as_mut_ptr(),
        serr.as_mut_ptr(),
    );

    if ret_code < RAW_OK {
        return Err((ret_code, buffer_to_string(serr)))
    }

    Ok((ret_code, dret))
}

// TODO:
pub unsafe fn swe_heliacal_angle(
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
) -> i32 {
    let mut serr = new_max_buffer();
    let ret_code = raw::swe_heliacal_angle(
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
        serr.as_mut_ptr(),
    );

    ret_code
}

// TODO:
pub unsafe fn swe_topo_arcus_visionis(
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
) -> (i32, i32) {
    let mut serr = new_max_buffer();
    let ret_code = raw::swe_topo_arcus_visionis(
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
        serr.as_mut_ptr(),
    );
    (ret_code, ret_code)
}

// TODO
pub unsafe fn swe_set_astro_models(samod: *mut ::std::os::raw::c_char, iflag: i32) {
    raw::swe_set_astro_models(samod, iflag)
}

// TODO:
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

pub unsafe fn swe_version() -> String {
    let mut buffer = new_max_buffer();
    let ret = raw::swe_version(buffer.as_mut_ptr());
    c_chars_to_string(ret)
}

pub unsafe fn swe_get_library_path() -> String {
    let mut buffer = new_max_buffer();
    let ret = raw::swe_get_library_path(buffer.as_mut_ptr());
    c_chars_to_string(ret)
}

pub unsafe fn swe_calc(
    tjd: f64,
    ipl: i32,
    iflag: i32,
) -> Result<(i32, [f64;6]), (i32, String)> {
    let mut serr = new_max_buffer();
    let mut xx: [f64;6] = [0.;6];
    let ret_code = raw::swe_calc(
        tjd,
        ipl,
        iflag,
        xx.as_mut_ptr(),
        serr.as_mut_ptr(),
    );

    if ret_code < RAW_OK {
        return Err((ret_code, buffer_to_string(serr)));
    }

    Ok((ret_code, xx))
}

pub unsafe fn swe_calc_ut(
    tjd_ut: f64,
    ipl: i32,
    iflag: i32,
) -> Result<(i32, [f64;6]), (i32, String)> {
    let mut serr = new_max_buffer();
    let mut xx: [f64;6] = [0.;6];
    let ret_code = raw::swe_calc_ut(
        tjd_ut,
        ipl,
        iflag,
        xx.as_mut_ptr(),
        serr.as_mut_ptr(),
    );

    if ret_code < RAW_OK {
        return Err((ret_code, buffer_to_string(serr)))
    }

    Ok((ret_code, xx))
}

//
//pub fn swe_calc_pctr(
//    tjd: f64,
//    ipl: i32,
//    iplctr: i32,
//    iflag: i32,
//) -> Result<[f64;6], (i32, String)> {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let mut xxret: [f64;6] = [0.;6];
//        let ret_code = raw::swe_calc_pctr(
//            tjd,
//            ipl,
//            iplctr,
//            iflag,
//            xxret.as_mut_ptr(),
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Ok(xxret)
//        }
//
//        Err((ret_code, buffer_to_string(serr)))
//
//    }
//}
//
//
//// Return value: double jx = time of next crossing, in Ephemeris Time or Universal Time.
//pub fn swe_solcross(
//    x2cross: f64,
//    jd_et: f64,
//    flag: i32,
//) -> Result<f64, (f64, String)> {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let jx = raw::swe_solcross(
//            x2cross,
//            jd_et,
//            flag,
//            serr.as_mut_ptr(),
//        );
//
//        if jx < jd_et {
//            return Err((jx, buffer_to_string(serr)))
//        }
//
//        Ok(jx)
//    }
//}
//
//pub fn swe_solcross_ut(
//    x2cross: f64,
//    jd_ut: f64,
//    flag: i32,
//) -> Result<f64, (f64, String)> {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let jx = raw::swe_solcross_ut(
//            x2cross,
//            jd_ut,
//            flag,
//            serr.as_mut_ptr(),
//        );
//
//        if jx < jd_ut {
//            return Err((jx, buffer_to_string(serr)))
//        }
//
//        Ok(jx)
//    }
//}
//
//pub fn swe_mooncross(
//    x2cross: f64,
//    jd_et: f64,
//    flag: i32,
//) -> Result<f64, (f64, String)> {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let jx = raw::swe_mooncross(
//            x2cross,
//            jd_et,
//            flag,
//            serr.as_mut_ptr(),
//        );
//
//        if jx < jd_et {
//            return Err((jx, buffer_to_string(serr)))
//        }
//
//        Ok(jx)
//    }
//}
//
//pub fn swe_mooncross_ut(
//    x2cross: f64,
//    jd_ut: f64,
//    flag: i32,
//) -> Result<f64, (f64, String)> {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let jx = raw::swe_mooncross_ut(
//            x2cross,
//            jd_ut,
//            flag,
//            serr.as_mut_ptr(),
//        );
//
//        if jx < jd_ut {
//            return Err((jx, buffer_to_string(serr)));
//        }
//
//        Ok(jx)
//    }
//}
//
//pub fn swe_mooncross_node(
//    jd_et: f64,
//    flag: i32,
//    xlon: *mut f64,
//    xlat: *mut f64,
//) -> Result<f64, (f64, String)>  {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let jx = raw::swe_mooncross_node(
//            jd_et,
//            flag,
//            xlon,
//            xlat,
//            serr.as_mut_ptr(),
//        );
//
//        if jx < jd_et {
//            return Err((jx, buffer_to_string(serr)));
//        }
//
//        Ok(jx)
//    }
//}
//
//pub fn swe_mooncross_node_ut(
//    jd_ut: f64,
//    flag: i32,
//    xlon: *mut f64,
//    xlat: *mut f64,
//) -> Result<f64, (f64, String)> {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let jx = raw::swe_mooncross_node_ut(
//            jd_ut,
//            flag,
//            xlon,
//            xlat,
//            serr.as_mut_ptr(),
//        );
//
//        if jx < jd_ut {
//            return Err((jx, buffer_to_string(serr)));
//        }
//
//        Ok(jx)
//    }
//}
//
//// Return value < 0 indicates an error, with error details in string serr 
//// (unless serr is a NULL pointer).
//// The crossing time is returned via parameter jx.
//pub fn swe_helio_cross(
//    ipl: i32,
//    x2cross: f64,
//    jd_et: f64,
//    iflag: i32,
//    dir: i32,
////    jd_cross: *mut f64,
//) -> Result<(i32, f64), (i32, String)> {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let mut jd_cross: f64 = 0.;
//        let ret_code = raw::swe_helio_cross(
//            ipl,
//            x2cross,
//            jd_et,
//            iflag,
//            dir,
//            &mut jd_cross,
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Err((ret_code, buffer_to_string(serr)));
//        }
//
//        Ok((ret_code, jd_cross))
//    }
//}
//
//pub fn swe_helio_cross_ut(
//    ipl: i32,
//    x2cross: f64,
//    jd_ut: f64,
//    iflag: i32,
//    dir: i32,
////    jd_cross: *mut f64,
//) -> Result<(i32, f64), (i32, String)> {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let mut jd_cross: f64 = 0.;
//        let ret_code = raw::swe_helio_cross_ut(
//            ipl,
//            x2cross,
//            jd_ut,
//            iflag,
//            dir,
//            &mut jd_cross,
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Err((ret_code, buffer_to_string(serr)));
//        }
//
//        Ok((ret_code, jd_cross))
//    }
//}
//
//pub fn swe_fixstar(
//    //star: *mut ::std::os::raw::c_char,
//    star: &str,
//    tjd: f64,
//    iflag: i32,
////    xx: *mut f64,
//) -> Result<(i32, String, [f64;6]), (i32, String)> {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let mut star_buffer = new_max_buffer_from_str(star);
//        let mut xx: [f64;6] = [0.;6];
//
//        let ret_code = raw::swe_fixstar(
//            star_buffer.as_mut_ptr(),
//            tjd,
//            iflag,
//            xx.as_mut_ptr(),
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Err((ret_code, buffer_to_string(serr)));
//        }
//
//        Ok((ret_code,buffer_to_string(star_buffer), xx))
//    }
//}
//
//pub fn swe_fixstar_ut(
//    //star: *mut ::std::os::raw::c_char,
//    star: &str,
//    tjd_ut: f64,
//    iflag: i32,
////    xx: *mut f64,
//) -> Result<(i32, String, [f64;6]), (i32, String)> {
//    unsafe {
//        let mut serr = new_max_buffer();
//        let mut star_buffer = new_max_buffer_from_str(star);
//        let mut xx: [f64;6] = [0.;6];
//        let ret_code = raw::swe_fixstar_ut(
//            star_buffer.as_mut_ptr(),
//            tjd_ut,
//            iflag,
//            xx.as_mut_ptr(),
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Err((ret_code, buffer_to_string(serr)));
//        }
//
//        Ok((ret_code, buffer_to_string(star_buffer), xx))
//    }
//}
//
//pub fn swe_fixstar_mag(
////    star: *mut ::std::os::raw::c_char,
//    star: &str,
////    mag: *mut f64,
//) -> Result<(i32, String), (i32, String)> {
//    unsafe {
//        let mut star_buffer = new_max_buffer_from_str(star);
//        let mut mag: f64 = 0.;
//        let mut serr = new_max_buffer();
//        let ret_code = raw::swe_fixstar_mag(
//            star_buffer.as_mut_ptr(),
//            &mut mag,
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Err((ret_code, buffer_to_string(serr)));
//        }
//
//        return Ok((ret_code, buffer_to_string(star_buffer)));
//    }
//}
//
//pub fn swe_fixstar2(
////    star: *mut ::std::os::raw::c_char,
//    star: &str,
//    tjd: f64,
//    iflag: i32,
////    xx: *mut f64,
//) -> Result<(i32, String, [f64;6]), (i32, String)>  {
//    unsafe {
//        let mut star_buffer = new_max_buffer_from_str(star);
//        let mut xx: [f64;6] = [0.;6];
//        let mut serr = new_max_buffer();
//        let ret_code = raw::swe_fixstar2(
//            star_buffer.as_mut_ptr(),
//            tjd,
//            iflag,
//            xx.as_mut_ptr(),
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Err((ret_code, buffer_to_string(serr)));
//        }
//
//        return Ok((ret_code, buffer_to_string(star_buffer), xx));
//    }
//}
//
//pub fn swe_fixstar2_ut(
// //   star: *mut ::std::os::raw::c_char,
//    star: &str,
//    tjd_ut: f64,
//    iflag: i32,
////    xx: *mut f64,
//) -> Result<(i32, String, [f64;6]), (i32, String)> {
//    unsafe {
//        let mut star_buffer = new_max_buffer_from_str(star);
//        let mut xx: [f64;6] = [0.;6];
//        let mut serr = new_max_buffer();
//        let ret_code = raw::swe_fixstar2_ut(
//            star_buffer.as_mut_ptr(),
//            tjd_ut,
//            iflag,
//            xx.as_mut_ptr(),
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Err((ret_code, buffer_to_string(serr)));
//        }
//
//        return Ok((ret_code, buffer_to_string(star_buffer), xx));
//    }
//}
//
//pub fn swe_fixstar2_mag(
//    //star: *mut ::std::os::raw::c_char,
//    star: &str,
////    mag: *mut f64,
//) -> Result<(i32, String, f64), (i32, String)> {
//    unsafe {
//        let mut star_buffer = new_max_buffer_from_str(star);
//        let mut mag: f64 = 0.;
//        let mut serr = new_max_buffer();
//        let ret_code = raw::swe_fixstar2_mag(
//            star_buffer.as_mut_ptr(),
//            &mut mag,
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Err((ret_code, buffer_to_string(serr)));
//        }
//
//        return Ok((ret_code, buffer_to_string(star_buffer), mag));
//    }
//}
//
//
pub unsafe fn swe_close() {
    raw::swe_close()
}
//
//pub fn swe_set_ephe_path(path: *const ::std::os::raw::c_char) {
pub unsafe fn swe_set_ephe_path(path: &str) {
    let mut path_buffer = new_max_buffer_from_str(path);
    raw::swe_set_ephe_path(path_buffer.as_mut_ptr()) 
}
//
////pub fn swe_set_jpl_file(fname: *const ::std::os::raw::c_char) {
//pub fn swe_set_jpl_file(fname: &str) {
//    unsafe {
//        let mut fname_buffer = new_max_buffer_from_str(fname);
//        raw::swe_set_jpl_file(fname_buffer.as_mut_ptr())
//    }
//}
//
//
//pub fn swe_get_planet_name(
//    ipl: ::std::os::raw::c_int,
////    spname: *mut ::std::os::raw::c_char,
////) -> *mut ::std::os::raw::c_char {
//) -> String {
//    unsafe {
//        let mut spname_buffer = new_max_buffer();
//        raw::swe_get_planet_name(
//            ipl,
//            spname_buffer.as_mut_ptr()
//        );
//
//        buffer_to_string(spname_buffer)
//    }
//}
//
//// The function returns either the ephemeris flag used or ERR (-1)
//pub fn swe_get_ayanamsa_ex(
//    tjd_et: f64,
//    iflag: i32,
////    daya: *mut f64,
//) -> Result<(i32, f64),(i32, String)> {
//    unsafe {
//        let mut daya: f64 = 0.;
//        let mut serr = new_max_buffer();
//        let ret_code = raw::swe_get_ayanamsa_ex(
//            tjd_et,
//            iflag,
//            &mut daya,
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Err((ret_code, buffer_to_string(serr)));
//        }
//
//        Ok((ret_code, daya))
//    }
//}
//
//pub fn swe_get_ayanamsa_ex_ut(
//    tjd_ut: f64,
//    iflag: i32,
// //   daya: *mut f64,
////) -> i32 {
//) -> Result<(i32, f64),(i32, String)> {
//    unsafe {
//        let mut daya: f64 = 0.;
//        let mut serr = new_max_buffer();
//        let ret_code = raw::swe_get_ayanamsa_ex_ut(
//            tjd_ut,
//            iflag,
//            &mut daya,
//            serr.as_mut_ptr(),
//        );
//
//        if ret_code < RAW_OK {
//            return Err((ret_code, buffer_to_string(serr)));
//        }
//
//        Ok((ret_code, daya))
//    }
//}
//
//
//// //pub fn swe_get_ayanamsa_name(isidmode: i32) -> *const ::std::os::raw::c_char {
//// pub fn swe_get_ayanamsa_name(isidmode: i32) -> String {
////     unsafe {
////         let name = raw::swe_get_ayanamsa_name(isidmode);
////        string_from_i8_array( name)
////     }
//// }
//
//// ifno = 0     planet file sepl_xxx, used for Sun .. Pluto, or jpl file
//// ifno = 1     moon file semo_xxx
//// ifno = 2     main asteroid file seas_xxx  if such an object was computed
//// ifno = 3     other asteroid or planetary moon file, if such object was computed
//// ifno = 4     star file
//// Return value: full file pathname, or NULL if no data
//// tfstart = start date of file,
//// tfend   = end data of fila,
//// denum   = jpl ephemeris number 406 or 431 from which file was derived
//// all three return values are zero for a jpl file or a star file.
//pub fn swe_get_current_file_data(
//    ifno: i32,
////    tfstart: *mut f64,
////    tfend: *mut f64,
////    denum: *mut ::std::os::raw::c_int,
//) -> *const ::std::os::raw::c_char {
//    unsafe {
//        let mut tfstart: f64 = 0.;
//        let mut tfend: f64 = 0.;
//        let mut denum: i32 = 0;
//
//        raw::swe_get_current_file_data(
//            ifno,
//            &mut tfstart,
//            &mut tfend,
//            &mut denum,
//        )
//    }
//}
//
//
pub unsafe fn swe_date_conversion(
    y: i32,
    m: i32,
    d: i32,
    utime: f64,
    c: ::std::os::raw::c_char,
) -> Result<(i32, f64), (i32, f64)> {
    let mut tjd: f64 = 0.;
    let ret_code = raw::swe_date_conversion(
        y,
        m,
        d,
        utime,
        c,
        &mut tjd,
    );

    if ret_code < RAW_OK {
        return Err((ret_code, tjd))
    }

    Ok((ret_code, tjd))
}

pub unsafe fn swe_revjul(
    jd: f64,
    gregflag: i32,
    //    jyear: *mut i32,
    //    jmon: *mut i32,
    //    jday: *mut i32,
    //    jut: *mut f64,
) -> (i32, i32, i32, f64) {
    let mut jyear: i32 = 0;
    let mut jmon: i32 = 0;
    let mut jday: i32 = 0;
    let mut jut: f64 = 0.;

    raw::swe_revjul(
        jd,
        gregflag,
        &mut jyear,
        &mut jmon,
        &mut jday,
        &mut jut,
    );

    (jyear, jmon, jday, jut)
}

pub unsafe fn swe_utc_to_jd(
    iyear: i32,
    imonth: i32,
    iday: i32,
    ihour: i32,
    imin: i32,
    dsec: f64,
    gregflag: i32,
    //    dret: *mut f64,
    //    serr: *mut ::std::os::raw::c_char,
) -> Result<(i32, [f64;2]), (i32, String)> {
    let mut dret: [f64; 2] = [0.;2];
    let mut serr = new_max_buffer();

    let ret_code = raw::swe_utc_to_jd(
        iyear,
        imonth,
        iday,
        ihour,
        imin,
        dsec,
        gregflag,
        dret.as_mut_ptr(),
        serr.as_mut_ptr(),
    );

    if ret_code < RAW_OK {
        return Err((ret_code, buffer_to_string(serr)));
    }

    Ok((ret_code, dret))
}

//pub fn swe_jdet_to_utc(
//    tjd_et: f64,
//    gregflag: i32,
//    iyear: *mut i32,
//    imonth: *mut i32,
//    iday: *mut i32,
//    ihour: *mut i32,
//    imin: *mut i32,
//    dsec: *mut f64,
//) {
//    unsafe {
//        raw::swe_jdet_to_utc(
//            tjd_et,
//            gregflag,
//            iyear,
//            imonth,
//            iday,
//            ihour,
//            imin,
//            dsec,
//        )
//    }
//}
//
//pub fn swe_jdut1_to_utc(
//    tjd_ut: f64,
//    gregflag: i32,
//    iyear: *mut i32,
//    imonth: *mut i32,
//    iday: *mut i32,
//    ihour: *mut i32,
//    imin: *mut i32,
//    dsec: *mut f64,
//) {
//    unsafe {
//        raw::swe_jdut1_to_utc(
//            tjd_ut,
//            gregflag,
//            iyear,
//            imonth,
//            iday,
//            ihour,
//            imin,
//            dsec,
//        )
//    }
//}
//
//pub fn swe_utc_time_zone(
//    iyear: i32,
//    imonth: i32,
//    iday: i32,
//    ihour: i32,
//    imin: i32,
//    dsec: f64,
//    d_timezone: f64,
//    iyear_out: *mut i32,
//    imonth_out: *mut i32,
//    iday_out: *mut i32,
//    ihour_out: *mut i32,
//    imin_out: *mut i32,
//    dsec_out: *mut f64,
//) {
//    unsafe {
//        raw::swe_utc_time_zone(
//            iyear,
//            imonth,
//            iday,
//            ihour,
//            imin,
//            dsec,
//            d_timezone,
//            iyear_out,
//            imonth_out,
//            iday_out,
//            ihour_out,
//            imin_out,
//            dsec_out,
//        )
//    }
//}
//

pub unsafe fn swe_houses(
    tjd_ut: f64,
    geolat: f64,
    geolon: f64,
    hsys: i32,
//    cusps: *mut f64,
//    ascmc: *mut f64,
) -> Result<(Option<(i32, [f64; 13], [f64;10])>, Option<(i32, [f64; 37], [f64;10])>), i32> {
    let mut ascmc: [f64; 10] = [0.; 10];
    if hsys != 'G' as i32 {
        let mut cusps: [f64; 13] = [0.; 13];
        let ret_code = raw::swe_houses(
            tjd_ut,
            geolat,
            geolon,
            hsys as i32,
            cusps.as_mut_ptr(),
            ascmc.as_mut_ptr(),
        );
        let houses = Some((ret_code, cusps, ascmc));
        return Ok((houses, None));
    } else {
        let mut cusps: [f64; 37] = [0.; 37];
        let ret_code = raw::swe_houses(
            tjd_ut,
            geolat,
            geolon,
            hsys as i32,
            cusps.as_mut_ptr(),
            ascmc.as_mut_ptr(),
        );

        let houses = Some((ret_code, cusps, ascmc));
        return Ok((None, houses));
    };
}

//pub fn swe_houses_ex(
//    tjd_ut: f64,
//    iflag: i32,
//    geolat: f64,
//    geolon: f64,
//    hsys: i32,
//    cusps: *mut f64,
//    ascmc: *mut f64,
//) -> i32 {
//    unsafe {
//        raw::swe_houses_ex(
//            tjd_ut,
//            iflag,
//            geolat,
//            geolon,
//            hsys,
//            cusps,
//            ascmc,
//        )
//    }
//}
//
//pub fn swe_houses_ex2(
//    tjd_ut: f64,
//    iflag: i32,
//    geolat: f64,
//    geolon: f64,
//    hsys: i32,
//    cusps: *mut f64,
//    ascmc: *mut f64,
//    cusp_speed: *mut f64,
//    ascmc_speed: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_houses_ex2(
//            tjd_ut,
//            iflag,
//            geolat,
//            geolon,
//            hsys,
//            cusps,
//            ascmc,
//            cusp_speed,
//            ascmc_speed,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_houses_armc(
//    armc: f64,
//    geolat: f64,
//    eps: f64,
//    hsys: ::std::os::raw::c_int,
//    cusps: *mut f64,
//    ascmc: *mut f64,
//) -> i32 {
//    unsafe {
//        raw::swe_houses_armc(
//            armc,
//            geolat,
//            eps,
//            hsys,
//            cusps,
//            ascmc,
//        )
//    }
//}
//
////return ::std::os::raw::c_int
//pub fn swe_houses_armc_ex2(
//    armc: f64,
//    geolat: f64,
//    eps: f64,
//    hsys: i32,
//    cusps: *mut f64,
//    ascmc: *mut f64,
//    cusp_speed: *mut f64,
//    ascmc_speed: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_houses_armc_ex2(
//            armc,
//            geolat,
//            eps,
//            hsys,
//            cusps,
//            ascmc,
//            cusp_speed,
//            ascmc_speed,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
pub unsafe fn swe_house_pos(
    armc: f64,
    geolat: f64,
    eps: f64,
    hsys: i32,
    //xpin: *mut f64,
    mut xpin: [f64;2],
) -> f64 {
    let mut serr = new_max_buffer();
    raw::swe_house_pos(
        armc,
        geolat,
        eps,
        hsys,
        xpin.as_mut_ptr(),
        serr.as_mut_ptr(),
    )
}

pub unsafe fn swe_house_name(hsys: i32) -> String {
    let house_name = raw::swe_house_name(hsys);
    CStr::from_ptr(house_name).to_str().unwrap().to_string()
}

//pub fn swe_gauquelin_sector(
//    t_ut: f64,
//    ipl: i32,
//    starname: *mut ::std::os::raw::c_char,
//    iflag: i32,
//    imeth: i32,
//    geopos: *mut f64,
//    atpress: f64,
//    attemp: f64,
//    dgsect: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_gauquelin_sector(
//            t_ut,
//            ipl,
//            starname,
//            iflag,
//            imeth,
//            geopos,
//            atpress,
//            attemp,
//            dgsect,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_sol_eclipse_where(
//    tjd: f64,
//    ifl: i32,
//    geopos: *mut f64,
//    attr: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_sol_eclipse_where(
//            tjd,
//            ifl,
//            geopos,
//            attr,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_lun_occult_where(
//    tjd: f64,
//    ipl: i32,
//    starname: *mut ::std::os::raw::c_char,
//    ifl: i32,
//    geopos: *mut f64,
//    attr: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_lun_occult_where(
//            tjd,
//            ipl,
//            starname,
//            ifl,
//            geopos,
//            attr,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_sol_eclipse_how(
//    tjd: f64,
//    ifl: i32,
//    geopos: *mut f64,
//    attr: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_sol_eclipse_how(
//            tjd,
//            ifl,
//            geopos,
//            attr,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_sol_eclipse_when_loc(
//    tjd_start: f64,
//    ifl: i32,
//    geopos: *mut f64,
//    tret: *mut f64,
//    attr: *mut f64,
//    backward: i32,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_sol_eclipse_when_loc(
//            tjd_start,
//            ifl,
//            geopos,
//            tret,
//            attr,
//            backward,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_lun_occult_when_loc(
//    tjd_start: f64,
//    ipl: i32,
//    starname: *mut ::std::os::raw::c_char,
//    ifl: i32,
//    geopos: *mut f64,
//    tret: *mut f64,
//    attr: *mut f64,
//    backward: i32,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_lun_occult_when_loc(
//            tjd_start,
//            ipl,
//            starname,
//            ifl,
//            geopos,
//            tret,
//            attr,
//            backward,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_sol_eclipse_when_glob(
//    tjd_start: f64,
//    ifl: i32,
//    ifltype: i32,
//    tret: *mut f64,
//    backward: i32,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_sol_eclipse_when_glob(
//            tjd_start,
//            ifl,
//            ifltype,
//            tret,
//            backward,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_lun_occult_when_glob(
//    tjd_start: f64,
//    ipl: i32,
//    starname: *mut ::std::os::raw::c_char,
//    ifl: i32,
//    ifltype: i32,
//    tret: *mut f64,
//    backward: i32,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_lun_occult_when_glob(
//            tjd_start,
//            ipl,
//            starname,
//            ifl,
//            ifltype,
//            tret,
//            backward,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_lun_eclipse_how(
//    tjd_ut: f64,
//    ifl: i32,
//    geopos: *mut f64,
//    attr: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_lun_eclipse_how(
//            tjd_ut,
//            ifl,
//            geopos,
//            attr,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_lun_eclipse_when(
//    tjd_start: f64,
//    ifl: i32,
//    ifltype: i32,
//    tret: *mut f64,
//    backward: i32,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_lun_eclipse_when(
//            tjd_start,
//            ifl,
//            ifltype,
//            tret,
//            backward,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_lun_eclipse_when_loc(
//    tjd_start: f64,
//    ifl: i32,
//    geopos: *mut f64,
//    tret: *mut f64,
//    attr: *mut f64,
//    backward: i32,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_lun_eclipse_when_loc(
//            tjd_start,
//            ifl,
//            geopos,
//            tret,
//            attr,
//            backward,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_pheno(
//    tjd: f64,
//    ipl: i32,
//    iflag: i32,
//    attr: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_pheno(
//            tjd,
//            ipl,
//            iflag,
//            attr,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_pheno_ut(
//    tjd_ut: f64,
//    ipl: i32,
//    iflag: i32,
//    attr: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_pheno_ut(
//            tjd_ut,
//            ipl,
//            iflag,
//            attr,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_refrac(inalt: f64, atpress: f64, attemp: f64, calc_flag: i32) -> f64 {
//    unsafe {
//        raw::swe_refrac(inalt, atpress, attemp, calc_flag)
//    }
//}
//
//pub fn swe_refrac_extended(
//    inalt: f64,
//    geoalt: f64,
//    atpress: f64,
//    attemp: f64,
//    lapse_rate: f64,
//    calc_flag: i32,
//    dret: *mut f64,
//) -> f64 {
//    unsafe {
//        raw::swe_refrac_extended(
//            inalt,
//            geoalt,
//            atpress,
//            attemp,
//            lapse_rate,
//            calc_flag,
//            dret,
//        )
//    }
//
//}
//
//pub fn swe_set_lapse_rate(lapse_rate: f64) {
//    unsafe {
//        raw::swe_set_lapse_rate(lapse_rate)
//    }
//}
//
//pub fn swe_azalt(
//    tjd_ut: f64,
//    calc_flag: i32,
//    geopos: *mut f64,
//    atpress: f64,
//    attemp: f64,
//    xin: *mut f64,
//    xaz: *mut f64,
//) {
//    unsafe {
//        raw::swe_azalt(
//            tjd_ut,
//            calc_flag,
//            geopos,
//            atpress,
//            attemp,
//            xin,
//            xaz,
//        )
//    }
//}
//
//pub fn swe_azalt_rev(
//    tjd_ut: f64,
//    calc_flag: i32,
//    geopos: *mut f64,
//    xin: *mut f64,
//    xout: *mut f64,
//) {
//    unsafe {
//        raw::swe_azalt_rev(
//            tjd_ut,
//            calc_flag,
//            geopos,
//            xin,
//            xout,
//        )
//    }
//}
//
//pub fn swe_rise_trans_true_hor(
//    tjd_ut: f64,
//    ipl: i32,
//    starname: *mut ::std::os::raw::c_char,
//    epheflag: i32,
//    rsmi: i32,
//    geopos: *mut f64,
//    atpress: f64,
//    attemp: f64,
//    horhgt: f64,
//    tret: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_rise_trans_true_hor(
//            tjd_ut,
//            ipl,
//            starname,
//            epheflag,
//            rsmi,
//            geopos,
//            atpress,
//            attemp,
//            horhgt,
//            tret,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_rise_trans(
//    tjd_ut: f64,
//    ipl: i32,
//    starname: *mut ::std::os::raw::c_char,
//    epheflag: i32,
//    rsmi: i32,
//    geopos: *mut f64,
//    atpress: f64,
//    attemp: f64,
//    tret: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_rise_trans(
//            tjd_ut,
//            ipl,
//            starname,
//            epheflag,
//            rsmi,
//            geopos,
//            atpress,
//            attemp,
//            tret,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_nod_aps(
//    tjd_et: f64,
//    ipl: i32,
//    iflag: i32,
//    method: i32,
//    xnasc: *mut f64,
//    xndsc: *mut f64,
//    xperi: *mut f64,
//    xaphe: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_nod_aps(
//            tjd_et,
//            ipl,
//            iflag,
//            method,
//            xnasc,
//            xndsc,
//            xperi,
//            xaphe,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_nod_aps_ut(
//    tjd_ut: f64,
//    ipl: i32,
//    iflag: i32,
//    method: i32,
//    xnasc: *mut f64,
//    xndsc: *mut f64,
//    xperi: *mut f64,
//    xaphe: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_nod_aps_ut(
//            tjd_ut,
//            ipl,
//            iflag,
//            method,
//            xnasc,
//            xndsc,
//            xperi,
//            xaphe,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_get_orbital_elements(
//    tjd_et: f64,
//    ipl: i32,
//    iflag: i32,
//    dret: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_get_orbital_elements(
//            tjd_et,
//            ipl,
//            iflag,
//            dret,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_orbit_max_min_true_distance(
//    tjd_et: f64,
//    ipl: i32,
//    iflag: i32,
//    dmax: *mut f64,
//    dmin: *mut f64,
//    dtrue: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_orbit_max_min_true_distance(
//            tjd_et,
//            ipl,
//            iflag,
//            dmax,
//            dmin,
//            dtrue,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_deltat_ex(tjd: f64, iflag: i32) -> f64 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_deltat_ex(tjd, iflag, serr.as_mut_ptr())
//    }
//}
//
//pub fn swe_time_equ(tjd: f64, te: *mut f64) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_time_equ(tjd, te, serr.as_mut_ptr())
//    }
//}
//
//pub fn swe_lmt_to_lat(
//    tjd_lmt: f64,
//    geolon: f64,
//    tjd_lat: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_lmt_to_lat(
//            tjd_lmt,
//            geolon,
//            tjd_lat,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_lat_to_lmt(
//    tjd_lat: f64,
//    geolon: f64,
//    tjd_lmt: *mut f64,
//) -> i32 {
//    unsafe {
//        let mut serr = new_max_buffer();
//        raw::swe_lat_to_lmt(
//            tjd_lat,
//            geolon,
//            tjd_lmt,
//            serr.as_mut_ptr(),
//        )
//    }
//}
//
//pub fn swe_cotrans(xpo: *mut f64, xpn: *mut f64, eps: f64) {
//    unsafe {
//        raw::swe_cotrans(xpo, xpn, eps)
//    }
//}
//
//pub fn swe_cotrans_sp(xpo: *mut f64, xpn: *mut f64, eps: f64) {
//    unsafe {
//        raw::swe_cotrans_sp(xpo, xpn, eps)
//    }
//}
//
pub unsafe fn swe_split_deg(
    ddeg: f64,
    roundflag: i32,
//    ideg: *mut i32,
//    imin: *mut i32,
//    isec: *mut i32,
//    dsecfr: *mut f64,
//    isgn: *mut i32,
) -> (i32, i32, i32, f64, i32) {
    let mut ideg: i32 = 0;
    let mut imin: i32 = 0;
    let mut isec: i32 = 0;
    let mut dsecfr: f64 = 0.;
    let mut isgn: i32 = 0;

        raw::swe_split_deg(
            ddeg,
            roundflag,
            &mut ideg,
            &mut imin,
            &mut isec,
            &mut dsecfr,
            &mut isgn,
        );

    (ideg, imin, isec, dsecfr, isgn)
}
//
//pub fn swe_cs2timestr(
//    t: centisec,
//    sep: i32,
//    suppress_zero: bool,
//    a: *mut ::std::os::raw::c_char,
//) -> *mut ::std::os::raw::c_char {
//    unsafe {
//        let suppress_zero = bool_to_as_bool(suppress_zero);
//        raw::swe_cs2timestr(
//            t,
//            sep,
//            suppress_zero,
//            a,
//        )
//    }
//}
//
//pub fn swe_cs2lonlatstr(
//    t: centisec,
//    pchar: ::std::os::raw::c_char,
//    mchar: ::std::os::raw::c_char,
//    s: *mut ::std::os::raw::c_char,
//) -> *mut ::std::os::raw::c_char {
//    unsafe {
//        raw::swe_cs2lonlatstr(
//            t,
//            pchar,
//            mchar,
//            s,
//        )
//    }
//}
//
//pub fn swe_cs2degstr(
//    t: centisec,
//    a: *mut ::std::os::raw::c_char,
//) -> *mut ::std::os::raw::c_char {
//    unsafe {
//        raw::swe_cs2degstr(
//            t,
//            a,
//        )
//    }
//}
//
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    //use std::ffi::CString;
//    //use std::os::raw::c_char;
//
//    #[test]
//    fn test_swe_house_name() {
//        let hn = swe_house_name(1);
//        assert_eq!(hn, "Placidus");
//    }
//
//}
//
