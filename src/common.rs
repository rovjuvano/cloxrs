//> Chunks of Bytecode common-h
pub use ::core::clone::Clone;
//> main-chunk

use ::core::mem::MaybeUninit;
//> A Virtual Machine define-debug-trace

//> Compiling Expressions define-debug-print-code
// rustflags = "--cfg DEBUG_PRINT_CODE"
//< Compiling Expressions define-debug-print-code
// rustflags = "--cfg DEBUG_TRACE_EXECUTION"
//< A Virtual Machine define-debug-trace

//> Scanning on Demand run-file
pub(crate) use ::core::ffi::c_char;
extern "C" {
    // compiler builtins
//> check-keyword
    pub(crate) fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> ::core::ffi::c_int;
//< check-keyword
    pub(crate) fn strlen(s: *const ::core::ffi::c_char) -> usize;
}
//< Scanning on Demand run-file
//> Scanning on Demand dump-tokens
macro_rules! str_from_raw_parts {
    ($ptr:expr, $length:expr) => {{
        let ptr = $ptr;
        let length = $length as usize;
        ::core::str::from_utf8(
            unsafe { ::core::slice::from_raw_parts(ptr, length) }
        ).unwrap()
    }};
}
pub(crate) use str_from_raw_parts;
//< Scanning on Demand dump-tokens
pub const unsafe fn uninit<T>() -> T {
    unsafe { MaybeUninit::<T>::uninit().assume_init() }
}
//< main-chunk
//> A Virtual Machine vm.c
macro_rules! uninit_static {
    ($type:tt) => {{
        const S: usize = ::core::mem::size_of::<$type>();
        union U {
            none: [u8; S],
            some: ::core::mem::ManuallyDrop<$type>,
        }
        let x = U { none: [0; S] };
        ::core::mem::ManuallyDrop::into_inner(unsafe { x.some })
    }};
}
pub(crate) use uninit_static;
//< A Virtual Machine vm.c
