#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod util;
mod types;

pub mod raw;
// raw mod will be the main way to interact with this library. User will need to work 
// directly with cstring pointers and unsafe blocks
pub use raw::*;

// all functions from raw, but each one wrapped with unsafe blocks  
pub mod safe;

// Result will attempt to turn return codes into rusty Results and provide return types
// that fit the possible outcomes. These will be as verbose as possible to capture 
// everything for a consuming library to handle
pub mod tuple_result;
//pub use result::*;

