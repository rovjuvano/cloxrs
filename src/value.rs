//> Chunks of Bytecode value-c
use ::core::ptr::*;
use ::std::*;
//> Strings value-include-string
// no need for additional includes here
//< Strings value-include-string

//> Strings value-include-object
use crate::object::*;
//< Strings value-include-object
use crate::memory::*;
//> Chunks of Bytecode value-h
pub use crate::common::*;

//> Strings forward-declare-obj
// no need to forward declare Obj
//> forward-declare-obj-string
// no need to forward declare ObjString
//< forward-declare-obj-string

//< Strings forward-declare-obj
//> Types of Values value-type
#[derive(Clone)] // Copy, Eq, Ord too but made explicit
#[repr(u8)]
pub enum ValueType {
    VAL_BOOL,
    VAL_NIL, // [user-types]
    VAL_NUMBER,
//> Strings val-obj
    VAL_OBJ,
//< Strings val-obj
}
pub use ValueType::*;

//< Types of Values value-type
/* Chunks of Bytecode value-h < Types of Values value
pub type Value = f64;
*/
//> Types of Values value
#[derive(Clone)] // Copy too but made explicit
pub struct Value {
    pub r#type: ValueType,
    pub r#as: ValueUnion,
}
#[derive(Clone, Copy)] // Copy (or ManuallyDrop) required by union
pub union ValueUnion {
    pub boolean: bool,
    pub number: f64,
//> Strings union-object
    pub obj: *mut Obj,
//< Strings union-object
} // [as]
//< Types of Values value
//> Types of Values is-macros

macro_rules! IS_BOOL {
    ($value:expr) => { $value.r#type.clone() as u8 == VAL_BOOL as u8 };
}
pub(crate) use IS_BOOL;
macro_rules! IS_NIL {
    ($value:expr) => { $value.r#type.clone() as u8 == VAL_NIL as u8 };
}
pub(crate) use IS_NIL;
macro_rules! IS_NUMBER {
    ($value:expr) => { $value.r#type.clone() as u8 == VAL_NUMBER as u8 };
}
pub(crate) use IS_NUMBER;
//> Strings is-obj
macro_rules! IS_OBJ {
    ($value:expr) => { $value.r#type.clone() as u8 == VAL_OBJ as u8 };
}
pub(crate) use IS_OBJ;
//< Strings is-obj
//< Types of Values is-macros
//> Types of Values as-macros

//> Strings as-obj
macro_rules! AS_OBJ {
    ($value:expr) => { $value.r#as.obj };
}
pub(crate) use AS_OBJ;
//< Strings as-obj
macro_rules! AS_BOOL {
    ($value:expr) => { $value.r#as.boolean };
}
pub(crate) use AS_BOOL;
macro_rules! AS_NUMBER {
    ($value:expr) => { $value.r#as.number };
}
pub(crate) use AS_NUMBER;
//< Types of Values as-macros
//> Types of Values value-macros

macro_rules! BOOL_VAL {
    ($value:expr) => { Value { r#type: VAL_BOOL, r#as: ValueUnion { boolean: $value } } };
}
pub(crate) use BOOL_VAL;
macro_rules! NIL_VAL {
    () => { Value { r#type: VAL_NIL, r#as: ValueUnion { number: 0.0 } } };
}
pub(crate) use NIL_VAL;
macro_rules! NUMBER_VAL {
    ($value:expr) => { Value { r#type: VAL_NUMBER, r#as: ValueUnion { number: $value } } };
}
pub(crate) use NUMBER_VAL;
//> Strings obj-val
macro_rules! OBJ_VAL {
    ($object:expr) => { Value { r#type: VAL_OBJ, r#as: ValueUnion { obj: $object as *mut Obj } } };
}
pub(crate) use OBJ_VAL;
//< Strings obj-val
//< Types of Values value-macros
//> value-array

#[derive(Clone)] // Copy too but made explicit
pub struct ValueArray {
    pub capacity: isize,
    pub count: isize,
    pub values: *mut Value,
}
//< value-array
//> array-fns-h

//> Types of Values values-equal-h
// no need to forward declare valuesEqual
//< Types of Values values-equal-h
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
/* Chunks of Bytecode print-value < Types of Values print-number-value
    print!("{}", value);
*/
/* Types of Values print-number-value < Types of Values print-value
    print!("{}", unsafe { AS_NUMBER!(value) });
*/
//> Types of Values print-value
    match value.r#type {
        VAL_BOOL => {
            print!("{}", if unsafe { AS_BOOL!(value) } { "true" } else { "false" });
        }
        VAL_NIL => print!("nil"),
        VAL_NUMBER => print!("{}", unsafe { AS_NUMBER!(value) }),
//> Strings call-print-object
        VAL_OBJ => unsafe { printObject(value) },
//< Strings call-print-object
    }
//< Types of Values print-value
}
//< print-value
//> Types of Values values-equal
pub unsafe fn valuesEqual(mut a: Value, mut b: Value) -> bool {
    if a.r#type.clone() as u8 != b.r#type.clone() as u8 { return false; }
    return match a.r#type {
        VAL_BOOL   => (unsafe { AS_BOOL!(a) } == unsafe { AS_BOOL!(b) }),
        VAL_NIL    => true,
        VAL_NUMBER => (unsafe { AS_NUMBER!(a) } == unsafe { AS_NUMBER!(b) }),
//> Strings strings-equal
        VAL_OBJ => {
            let mut aString: *mut ObjString = unsafe { AS_STRING!(a) };
            let mut bString: *mut ObjString = unsafe { AS_STRING!(b) };
            (unsafe { (*aString).length } == unsafe { (*bString).length }) &&
                unsafe { memcmp(unsafe { (*aString).chars }, unsafe { (*bString).chars },
                    unsafe { (*aString).length } as usize) } == 0
        }
//< Strings strings-equal
        #[allow(unreachable_patterns)]
        _ => false, // Unreachable.
    };
}
//< Types of Values values-equal
