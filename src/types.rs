use crate::raw::AS_MAXCH;

// bindgen made these different types because it assumed ok being 0 would be u32
// and that err being -1 would be i32
pub const RAW_OK: i32 = crate::raw::OK as i32;

pub const RAW_ERR: i32 = crate::raw::ERR;

pub const MAXCH: usize = AS_MAXCH as usize;
pub type MaxBuffer = [i8; MAXCH];

//pub type MaxBuffer2 = [u8; MAXCH];



//pub type BufferType = [i8; MAXCH];

//pub type CalcPrimRet = [f64; 6];
//pub type PrimitivePosition = [f64; 6];
