include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_char;


    #[test]
    fn raw_swe_calc_ut_works() {
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
    fn raw_swe_julday_works() {
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

}




