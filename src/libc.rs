// We define our own libc types here
// and use #[allow(improper_ctypes)] on extern blocks
// to circumvent that we did not link the Rust libc

#[allow(non_camel_case_types)]
pub type c_int = int;
