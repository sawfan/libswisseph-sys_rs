// Simple methods don't require any error handling, error strings, or modifing data in place
pub mod simple;
pub use simple::*;

// Creation methods will not require the user to init data before passing it in. All outputs
// will return new data. This is more ergonomic, but not as performant as structures cannot
// be reused.
pub mod create;
pub use create::*;

