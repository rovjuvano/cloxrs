//> Chunks of Bytecode memory-c
use ::alloc::alloc::*;
use ::core::ptr::*;
use ::std::process::*;

//> Chunks of Bytecode memory-h
pub use crate::common::*;
//> Strings memory-include-object
pub use crate::object::*;
//< Strings memory-include-object

//> Strings allocate
macro_rules! ALLOCATE {
    ($type:ty, $count:expr) => {{
        let size_new = ::core::mem::size_of::<$type>() * $count;
        unsafe { reallocate(::core::ptr::null_mut(), 0, size_new) as *mut $type }
    }};
}
pub(crate) use ALLOCATE;
//> free

macro_rules! FREE {
    ($type:tt, $pointer:expr) => {{
        let size_old = ::core::mem::size_of::<$type>();
        let ptr = $pointer as *mut u8;
        unsafe { reallocate(ptr, size_old, 0) }
    }};
}
pub(crate) use FREE;
//< free

//< Strings allocate
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
//> Strings free-objects-h
// no need to forward declare freeObjects
//< Strings free-objects-h
//< Chunks of Bytecode memory-h
//> Strings memory-include-vm
use crate::vm::*;
//< Strings memory-include-vm

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
//> Strings free-object
unsafe fn freeObject(mut object: *mut Obj) {
    match unsafe { (*object).r#type.clone() } {
        OBJ_STRING => {
            let mut string: *mut ObjString = object as *mut ObjString;
            let _ = unsafe { FREE_ARRAY!(u8, unsafe { (*string).chars }, unsafe { (*string).length + 1 }) };
            let _ = unsafe { FREE!(ObjString, object) };
        }
    }
}
//< Strings free-object
//> Strings free-objects
pub unsafe fn freeObjects() {
    let mut object: *mut Obj = unsafe { vm.objects };
    while !object.is_null() {
        let mut next: *mut Obj = unsafe { (*object).next };
        unsafe { freeObject(object) };
        object = next;
    }
}
//< Strings free-objects
