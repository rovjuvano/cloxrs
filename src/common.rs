//> Chunks of Bytecode common-h
pub use ::core::clone::Clone;
//> main-chunk

use ::core::mem::MaybeUninit;

pub const unsafe fn uninit<T>() -> T {
    unsafe { MaybeUninit::<T>::uninit().assume_init() }
}
//< main-chunk
