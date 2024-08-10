use crate::raw;
use crate::types::*;
use std::ffi::CStr;
//use std::ffi::CString;
//use std::os::raw::c_char;

//util function
pub fn bool_to_as_bool(b: bool) -> i32 {
    let r = match b {
        true => raw::MY_TRUE,
        false => raw::MY_FALSE
    };

    r as i32
}

pub fn new_max_buffer() -> MaxBuffer {
    [0; MAXCH]
}

// Creates a new buffer and copies the contents of a &str into it.
// Mainly this is needed because &str only allocate the size of the current string
// whereas many of the calls in the swiss ephemeris modify the string passed in.
pub fn new_max_buffer_from_str(s: &str) -> MaxBuffer {
    let mut buff = [0; MAXCH];

    for (i,el) in s.as_bytes().iter().enumerate() {
        buff[i] = el.clone() as i8;
    }

    buff
}

// This should only be used for methods that do not modify the underlying str data.
// strings from rust may not be the right size or be null terminated properly.
//pub fn str_to_c_char_ptr(s: &str) -> *const c_char {
//    let c_str = CString::new(s).unwrap();
//    c_str.as_ptr() as *const c_char
//}

pub fn buffer_to_string(b: MaxBuffer) -> String {
    String::from_utf8(b.iter().map(|&c| c as u8).collect()).unwrap()
}

pub unsafe fn c_chars_to_string(s: *mut ::std::os::raw::c_char) -> String {
    serr_to_string(s)
}

pub unsafe fn serr_to_string(s: *mut ::std::os::raw::c_char) -> String {
    let rust_id = CStr::from_ptr(s);
    let rust_id = rust_id.to_owned();
    let str_slice: &str = rust_id.to_str().unwrap();
    let serr: String = str_slice.to_owned();
    serr
}

