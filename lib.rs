#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swe_calc_ut_works() {
        unsafe {
            let mut xx: [f64; 6] = [0.0; 6];
            let mut serr = [0; 255];

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

            //panic!("YO: {:#?}", xx);
        }
    }

    #[test]
    fn swe_julday_works() {
        let y = 2002;
        let m = 1;
        let d = 1;
        let h = 0.0;

        let _jd = unsafe {
            swe_julday(y, m, d, h, SE_GREG_CAL as i32)
        };
        // TODO:

        //assert_eq!(result, 4);
    }
}

