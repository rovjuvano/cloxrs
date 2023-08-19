//> Chunks of Bytecode memory-c
use ::alloc::alloc::*;
use ::core::ptr::*;
use ::std::process::*;

//> Chunks of Bytecode memory-h
pub use crate::common::*;

macro_rules! GROW_CAPACITY {
    ($capacity:expr) => {{ if $capacity < 8 { 8 } else { 2 * $capacity } }};
}
pub(crate) use GROW_CAPACITY;
//> grow-array

macro_rules! GROW_ARRAY {
    ($type:ty, $pointer:expr, $oldCount:expr, $newCount:expr) => {{
        let size_of = ::core::mem::size_of::<$type>();
        let size_old = size_of * $oldCount as usize;
        let size_new = size_of * $newCount as usize;
        let ptr = $pointer;
        unsafe { reallocate(ptr, size_old, size_new) as *mut $type }
    }};
}
pub(crate) use GROW_ARRAY;
//> free-array

macro_rules! FREE_ARRAY {
    ($type:ty, $pointer:expr, $oldCount:expr) => {{
        let size_old = ::core::mem::size_of::<$type>() * $oldCount as usize;
        let ptr = $pointer;
        unsafe { reallocate(ptr, size_old, 0) }
    }};
}
pub(crate) use FREE_ARRAY;
//< free-array

// no need to forward declare reallocate
//< grow-array
//< Chunks of Bytecode memory-h

pub unsafe fn reallocate(mut pointer: *mut u8, mut oldSize: usize, mut newSize: usize) -> *mut u8 {
    let mut layout: Layout = Layout::array::<u8>(oldSize).unwrap();
    if newSize == 0 {
        unsafe { dealloc(pointer, layout) };
        return null_mut();
    }

    let mut result: *mut u8 = unsafe { realloc(pointer, layout, newSize) };
//> out-of-memory
    if result.is_null() { exit(1); }
//< out-of-memory
    return result;
}
