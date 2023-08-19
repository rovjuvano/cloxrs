//> Chunks of Bytecode value-c
use ::core::ptr::*;
use ::std::*;

use crate::memory::*;
//> Chunks of Bytecode value-h
pub use crate::common::*;

pub type Value = f64;
//> value-array

#[derive(Clone)] // Copy too but made explicit
pub struct ValueArray {
    pub capacity: isize,
    pub count: isize,
    pub values: *mut Value,
}
//< value-array
//> array-fns-h

// no need to forward declare initValueArray
// no need to forward declare writeValueArray
// no need to forward declare freeValueArray
//< array-fns-h
//> print-value-h
// no need to forward declare printValue
//< print-value-h
//< Chunks of Bytecode value-h

pub unsafe fn initValueArray(mut array: *mut ValueArray) {
    unsafe { (*array).values = null_mut() };
    unsafe { (*array).capacity = 0 };
    unsafe { (*array).count = 0 };
}
//> write-value-array
pub unsafe fn writeValueArray(mut array: *mut ValueArray, mut value: Value) {
    if unsafe { (*array).capacity } < unsafe { (*array).count } + 1 {
        let mut oldCapacity: isize = unsafe { (*array).capacity };
        unsafe { (*array).capacity = GROW_CAPACITY!(oldCapacity) };
        unsafe { (*array).values = GROW_ARRAY!(Value,
            unsafe { (*array).values } as *mut u8, oldCapacity, unsafe { (*array).capacity }) };
    }

    unsafe { *(*array).values.offset(unsafe { (*array).count }) = value };
    unsafe { (*array).count += 1 };
}
//< write-value-array
//> free-value-array
pub unsafe fn freeValueArray(mut array: *mut ValueArray) {
    let _ = unsafe { FREE_ARRAY!(Value, unsafe { (*array).values } as *mut u8, unsafe { (*array).capacity }) };
    unsafe { initValueArray(array) };
}
//< free-value-array
//> print-value
pub unsafe fn printValue(mut value: Value) {
    print!("{}", value);
}
//< print-value
