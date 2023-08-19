//> Chunks of Bytecode common-h
pub use ::core::clone::Clone;
//> main-chunk

use ::core::mem::MaybeUninit;
//> A Virtual Machine define-debug-trace

// rustflags = "--cfg DEBUG_TRACE_EXECUTION"
//< A Virtual Machine define-debug-trace

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
