#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_char;

    #[test]
    fn swe_calc_ut_works() {
        unsafe {
            let s = "/users/ephe";
            let c_str = CString::new(s).unwrap();
            let path: *const c_char = c_str.as_ptr() as *const c_char;
            swe_set_ephe_path(path);

            let mut xx: [f64; 6] = [0.0; 6];
            let mut serr = [0; AS_MAXCH as usize];

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

            swe_close();
        }
    }

    #[test]
    fn swe_julday_works() {
        unsafe {
            let s = "/users/ephe";
            let c_str = CString::new(s).unwrap();
            let path: *const c_char = c_str.as_ptr() as *const c_char;
            swe_set_ephe_path(path);

            let y = 2002;
            let m = 1;
            let d = 1;
            let h = 0.0;

            let _jd = swe_julday(y, m, d, h, SE_GREG_CAL as i32);

            swe_close();
        };
    }

//    #[test]
//    fn test_calc_mundane_aspects() {
//        unsafe {
//
//            let y = 2002;
//            let m = 1;
//            let d = 1;
//            let h = 0.0;
//            let i_flag = SE_GREG_CAL as i32;
//
//            let tjd = swe_julday(y, m, d, h, i_flag);
//
//            let mut stnama = [0; 40usize];
//            let mut stnamb = [0; 40usize];
//            let mut splan = [0; AS_MAXCH as usize];
//            let mut sasp = [0; AS_MAXCH as usize];
//            let mut serr = [0; AS_MAXCH as usize];
//
//            
//            let mut pev = event {
//                tjd,
//                evtype: 0,
//                ipla: 0,
//                iplb: 0,
//                stnama: stnama,
//                stnamb: stnamb,
//                iasp: 0,
//                bpind: 0,
//                dasp: 0.,
//                dang: 0.,
//                isign: 0,
//                backward: 0,
//                dorb: 8.,
//                dret: 0.,
//            };
//
//
//            let code = calc_mundane_aspects(
//                0, // iflag: ::std::os::raw::c_int,
//                0., //tjd0: f64, 
//                0., //tjde: f64, 
//                0., //tstep: f64, 
//                splan.as_mut_ptr(),
//                sasp.as_mut_ptr(),
//                &mut pev, //: *mut event, 
//                serr.as_mut_ptr(), //serr: *mut ::std::os::raw::c_char,
//            );
//
//            panic!("coded thing: {:#?}", code);
//        }
//
//    }

}


// This was experimental attempt to call aspect functions that aren't included in header.
// You can call the functions by just creating an .a library using:
//  `ar -crs libswevents.a swevents.o`
// The main() must be commented out
//
//    //int32 calc_mundane_aspects(int32 iflag, double tjd0, double tjde, double tstep, 
//    //  char *splan, char *sasp, EVENT *pev, char *serr) {}
//    //
//#[link(name = "swevents")]
//extern "C" {
//    pub fn calc_mundane_aspects(
//        iflag: ::std::os::raw::c_int,
//        tjd0: f64, 
//        tjde: f64, 
//        tstep: f64, 
//        splan: *mut ::std::os::raw::c_char,
//        sasp: *mut ::std::os::raw::c_char,
//        pev: *mut event, 
//        serr: *mut ::std::os::raw::c_char,
//    ) -> ::std::os::raw::c_int;
//}
