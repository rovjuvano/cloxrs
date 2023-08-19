//> Strings object-c
use ::core::ptr::*;
use ::std::*;

use crate::memory::*;
//> Strings object-h
pub use crate::common::*;
pub use crate::value::*;
//> obj-type-macro

macro_rules! OBJ_TYPE {
    ($value:expr) => {{
        let value = $value;
        unsafe { (*unsafe { AS_OBJ!(value) }).r#type.clone() }
    }};
}
pub(crate) use OBJ_TYPE;
//< obj-type-macro
//> is-string

macro_rules! IS_STRING {
    ($value:expr) => { isObjType($value.clone(), OBJ_STRING) };
}
pub(crate) use IS_STRING;
//< is-string
//> as-string

macro_rules! AS_STRING {
    ($value:expr) => {{
        let value = $value;
        unsafe { AS_OBJ!(value) as *mut ObjString }
    }};
}
pub(crate) use AS_STRING;
macro_rules! AS_STR {
    ($value:expr) => {{
        let value = $value;
        let s = unsafe { AS_STRING!(value) };
        str_from_raw_parts!(unsafe { (*s).chars }, unsafe { (*s).length })
    }};
}
pub(crate) use AS_STR;
//< as-string
//> obj-type

#[derive(Clone)] // Copy, Eq, Ord too but made explicit
#[repr(u8)]
pub enum ObjType {
    OBJ_STRING,
}
pub use ObjType::*;
//< obj-type

#[derive(Clone)] // Copy too but made explicit
#[repr(C)]
pub struct Obj {
    pub r#type: ObjType,
//> next-field
    pub next: *mut Obj,
//< next-field
}
//> obj-string

#[derive(Clone)] // Copy too but made explicit
#[repr(C)]
pub struct ObjString {
    pub obj: Obj,
    pub length: isize,
    pub chars: *mut u8,
}
//< obj-string

//> take-string-h
// no need to forward declare takeString
//< take-string-h
//> copy-string-h
// no need to forward declare copyString
//> print-object-h
// no need to forward declare printObject
//< print-object-h

//< copy-string-h
//> is-obj-type
#[inline]
pub fn isObjType(mut value: Value, mut r#type: ObjType) -> bool {
    return IS_OBJ!(value) && unsafe { (*unsafe { AS_OBJ!(value) }).r#type.clone() } as u8 == r#type as u8;
}
//< is-obj-type
//< Strings object-h
#[allow(unused_imports)]
use crate::value::*;
use crate::vm::*;
//> allocate-obj

macro_rules! ALLOCATE_OBJ {
    ($type:ty, $objectType:ident) => {{
        let size_of = ::core::mem::size_of::<$type>();
        unsafe { allocateObject(size_of, $objectType) as *mut $type }
    }};
}
//< allocate-obj
//> allocate-object

unsafe fn allocateObject(mut size: usize, mut r#type: ObjType) -> *mut Obj {
    let mut object: *mut Obj = unsafe { reallocate(null_mut(), 0, size) } as *mut Obj;
    unsafe { (*object).r#type = r#type.clone() };
//> add-to-list

    unsafe { (*object).next = unsafe { vm.objects } };
    unsafe { vm.objects = object };
//< add-to-list
    return object;
}
//< allocate-object

//> allocate-string
unsafe fn allocateString(mut chars: *mut u8, mut length: isize) -> *mut ObjString {
    let mut string: *mut ObjString = unsafe { ALLOCATE_OBJ!(ObjString, OBJ_STRING) };
    unsafe { (*string).length = length };
    unsafe { (*string).chars = chars };
    return string;
}
//< allocate-string
//> take-string
pub unsafe fn takeString(mut chars: *mut u8, mut length: isize) -> *mut ObjString {
    return unsafe { allocateString(chars, length) };
}
//< take-string
pub unsafe fn copyString(mut chars: *const u8, mut length: isize) -> *mut ObjString {
    let mut heapChars: *mut u8 = unsafe { ALLOCATE!(u8, (length + 1) as usize) };
    unsafe { copy_nonoverlapping(chars, heapChars, length as usize) };
    unsafe { *heapChars.offset(length) = b'\0' };
    return unsafe { allocateString(heapChars, length) };
}
//> print-object
pub unsafe fn printObject(mut value: Value) {
    match unsafe { OBJ_TYPE!(value.clone()) } {
        OBJ_STRING => {
            print!("{}", unsafe { AS_STR!(value) });
        }
    };
}
//< print-object
