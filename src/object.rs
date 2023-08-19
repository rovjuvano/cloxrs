//> Strings object-c
use ::core::ptr::*;
use ::std::*;

use crate::memory::*;
//> Strings object-h
pub use crate::common::*;
//> Calls and Functions object-include-chunk
pub use crate::chunk::*;
//< Calls and Functions object-include-chunk
//> Classes and Instances object-include-table
pub use crate::table::*;
//< Classes and Instances object-include-table
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

//> Methods and Initializers is-bound-method
#[allow(unused_macros)]
macro_rules! IS_BOUND_METHOD {
    ($value:expr) => { isObjType($value.clone(), OBJ_BOUND_METHOD) };
}
#[allow(unused_imports)]
pub(crate) use IS_BOUND_METHOD;
//< Methods and Initializers is-bound-method
//> Classes and Instances is-class
/* Classes and Instances is-class < Superclasses inherit-non-class
#[allow(unused_macros)]
*/
macro_rules! IS_CLASS {
    ($value:expr) => { isObjType($value.clone(), OBJ_CLASS) };
}
/* Classes and Instances is-class < Superclasses inherit-non-class
#[allow(unused_imports)]
*/
pub(crate) use IS_CLASS;
//< Classes and Instances is-class
//> Closures is-closure
#[allow(unused_macros)]
macro_rules! IS_CLOSURE {
    ($value:expr) => { isObjType($value.clone(), OBJ_CLOSURE) };
}
//> Garbage Collection glob-imports-gone-wild
#[allow(unused_imports)]
//< Garbage Collection glob-imports-gone-wild
pub(crate) use IS_CLOSURE;
//< Closures is-closure
//> Calls and Functions is-function
#[allow(unused_macros)]
macro_rules! IS_FUNCTION {
    ($value:expr) => { isObjType($value.clone(), OBJ_FUNCTION) };
}
//> Garbage Collection glob-imports-gone-wild
#[allow(unused_imports)]
//< Garbage Collection glob-imports-gone-wild
pub(crate) use IS_FUNCTION;
//< Calls and Functions is-function
//> Classes and Instances is-instance
macro_rules! IS_INSTANCE {
    ($value:expr) => { isObjType($value.clone(), OBJ_INSTANCE) };
}
pub(crate) use IS_INSTANCE;
//< Classes and Instances is-instance
//> Calls and Functions is-native
#[allow(unused_macros)]
macro_rules! IS_NATIVE {
    ($value:expr) => { isObjType($value.clone(), OBJ_NATIVE) };
}
//> Garbage Collection glob-imports-gone-wild
#[allow(unused_imports)]
//< Garbage Collection glob-imports-gone-wild
pub(crate) use IS_NATIVE;
//< Calls and Functions is-native
macro_rules! IS_STRING {
    ($value:expr) => { isObjType($value.clone(), OBJ_STRING) };
}
pub(crate) use IS_STRING;
//< is-string
//> as-string

//> Methods and Initializers as-bound-method
macro_rules! AS_BOUND_METHOD {
    ($value:expr) => {{
        let value = $value;
        unsafe { AS_OBJ!(value) as *mut ObjBoundMethod }
    }};
}
pub(crate) use AS_BOUND_METHOD;
//< Methods and Initializers as-bound-method
//> Classes and Instances as-class
macro_rules! AS_CLASS {
    ($value:expr) => {{
        let value = $value;
        unsafe { AS_OBJ!(value) as *mut ObjClass }
     }};
}
pub(crate) use AS_CLASS;
//< Classes and Instances as-class
//> Closures as-closure
macro_rules! AS_CLOSURE {
    ($value:expr) => {{
        let value = $value;
        unsafe { AS_OBJ!(value) as *mut ObjClosure }
    }};
}
pub(crate) use AS_CLOSURE;
//< Closures as-closure
//> Calls and Functions as-function
macro_rules! AS_FUNCTION {
    ($value:expr) => {{
        let value = $value;
        unsafe { AS_OBJ!(value) as *mut ObjFunction }
    }};
}
pub(crate) use AS_FUNCTION;
//< Calls and Functions as-function
//> Classes and Instances as-instance
macro_rules! AS_INSTANCE {
    ($value:expr) => {{
        let value = $value;
        unsafe { AS_OBJ!(value) as *mut ObjInstance }
    }};
}
pub(crate) use AS_INSTANCE;
//< Classes and Instances as-instance
//> Calls and Functions as-native
macro_rules! AS_NATIVE {
    ($value:expr) => {{
        let value = $value;
        unsafe { (*(AS_OBJ!(value) as *mut ObjNative)).function }
    }};
}
pub(crate) use AS_NATIVE;
//< Calls and Functions as-native
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
//> Methods and Initializers obj-type-bound-method
    OBJ_BOUND_METHOD,
//< Methods and Initializers obj-type-bound-method
//> Classes and Instances obj-type-class
    OBJ_CLASS,
//< Classes and Instances obj-type-class
//> Closures obj-type-closure
    OBJ_CLOSURE,
//< Closures obj-type-closure
//> Calls and Functions obj-type-function
    OBJ_FUNCTION,
//< Calls and Functions obj-type-function
//> Classes and Instances obj-type-instance
    OBJ_INSTANCE,
//< Classes and Instances obj-type-instance
//> Calls and Functions obj-type-native
    OBJ_NATIVE,
//< Calls and Functions obj-type-native
    OBJ_STRING,
//> Closures obj-type-upvalue
    OBJ_UPVALUE,
//< Closures obj-type-upvalue
}
pub use ObjType::*;
//< obj-type

#[derive(Clone)] // Copy too but made explicit
#[repr(C)]
pub struct Obj {
    pub r#type: ObjType,
//> Garbage Collection is-marked-field
    pub isMarked: bool,
//< Garbage Collection is-marked-field
//> next-field
    pub next: *mut Obj,
//< next-field
}
//> Calls and Functions obj-function

#[derive(Clone)] // Copy too but made explicit
#[repr(C)]
pub struct ObjFunction {
    pub obj: Obj,
    pub arity: isize,
//> Closures upvalue-count
    pub upvalueCount: isize,
//< Closures upvalue-count
    pub chunk: Chunk,
    pub name: *mut ObjString,
}
//< Calls and Functions obj-function
//> Calls and Functions obj-native

pub type NativeFn = unsafe fn(argCount: isize, args: *mut Value) -> Value;

#[derive(Clone)] // Copy too but made explicit
#[repr(C)]
pub struct ObjNative {
    pub obj: Obj,
    pub function: NativeFn,
}
//< Calls and Functions obj-native
//> obj-string

#[derive(Clone)] // Copy too but made explicit
#[repr(C)]
pub struct ObjString {
    pub obj: Obj,
    pub length: isize,
    pub chars: *mut u8,
//> Hash Tables obj-string-hash
    pub hash: u32,
//< Hash Tables obj-string-hash
}
//< obj-string
//> Closures obj-upvalue
#[derive(Clone)] // Copy too but made explicit and as a self-referential struct, unsafely Unpin
#[repr(C)]
pub struct ObjUpvalue {
    pub obj: Obj,
    pub location: *mut Value,
//> closed-field
    pub closed: Value,
//< closed-field
//> next-field
    pub next: *mut ObjUpvalue,
//< next-field
}
//< Closures obj-upvalue
//> Closures obj-closure
#[derive(Clone)] // Copy too but made explicit
#[repr(C)]
pub struct ObjClosure {
    pub obj: Obj,
    pub function: *mut ObjFunction,
//> upvalue-fields
    pub upvalues: *mut *mut ObjUpvalue,
    pub upvalueCount: isize,
//< upvalue-fields
}
//< Closures obj-closure
//> Classes and Instances obj-class

#[derive(Clone)] // Copy too but made explicit
#[repr(C)]
pub struct ObjClass {
    pub obj: Obj,
    pub name: *mut ObjString,
//> Methods and Initializers class-methods
    pub methods: Table,
//< Methods and Initializers class-methods
}
//< Classes and Instances obj-class
//> Classes and Instances obj-instance

#[derive(Clone)] // Copy too but made explicit
#[repr(C)]
pub struct ObjInstance {
    pub obj: Obj,
    pub class: *mut ObjClass,
    pub fields: Table, // [fields]
}
//< Classes and Instances obj-instance

//> Methods and Initializers obj-bound-method
#[derive(Clone)] // Copy too but made explicit
#[repr(C)]
pub struct ObjBoundMethod {
    pub obj: Obj,
    pub receiver: Value,
    pub method: *mut ObjClosure,
}

//< Methods and Initializers obj-bound-method
//> Methods and Initializers new-bound-method-h
// no need to forward declare newBoundMethod
//< Methods and Initializers new-bound-method-h
//> Classes and Instances new-class-h
// no need to forward declare newClass
//< Classes and Instances new-class-h
//> Closures new-closure-h
// no need to forward declare newClosure
//< Closures new-closure-h
//> Calls and Functions new-function-h
// no need to forward declare newFunction
//< Calls and Functions new-function-h
//> Classes and Instances new-instance-h
// no need to forward declare newInstance
//< Classes and Instances new-instance-h
//> Calls and Functions new-native-h
// no need to forward declare newNative
//< Calls and Functions new-native-h
//> take-string-h
// no need to forward declare takeString
//< take-string-h
//> copy-string-h
// no need to forward declare copyString
//> Closures new-upvalue-h
// no need to forward declare newUpvalue
//< Closures new-upvalue-h
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
//> Hash Tables object-include-table
//> Classes and Instances object-include-table
#[allow(unused_imports)]
//< Classes and Instances object-include-table
use crate::table::*;
//< Hash Tables object-include-table
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
//> Garbage Collection init-is-marked
    unsafe { (*object).isMarked = false };
//< Garbage Collection init-is-marked
//> add-to-list

    unsafe { (*object).next = unsafe { vm.objects } };
    unsafe { vm.objects = object };
//< add-to-list
//> Garbage Collection debug-log-allocate

    #[cfg(DEBUG_LOG_GC)]
    print!("{:p} allocate {} for {}\n", object, size, r#type as u8);

//< Garbage Collection debug-log-allocate
    return object;
}
//< allocate-object
//> Methods and Initializers new-bound-method
pub unsafe fn newBoundMethod(mut receiver: Value,
        mut method: *mut ObjClosure) -> *mut ObjBoundMethod {
    let mut bound: *mut ObjBoundMethod =
        unsafe { ALLOCATE_OBJ!(ObjBoundMethod, OBJ_BOUND_METHOD) };
    unsafe { (*bound).receiver = receiver };
    unsafe { (*bound).method = method };
    return bound;
}
//< Methods and Initializers new-bound-method
//> Classes and Instances new-class
pub unsafe fn newClass(mut name: *mut ObjString) -> *mut ObjClass {
    let mut class: *mut ObjClass = unsafe { ALLOCATE_OBJ!(ObjClass, OBJ_CLASS) };
    unsafe { (*class).name = name }; // [klass]
//> Methods and Initializers init-methods
    unsafe { initTable(unsafe { &mut (*class).methods } as *mut Table) };
//< Methods and Initializers init-methods
    return class;
}
//< Classes and Instances new-class
//> Closures new-closure
pub unsafe fn newClosure(mut function: *mut ObjFunction) -> *mut ObjClosure {
//> allocate-upvalue-array
    let mut upvalues: *mut *mut ObjUpvalue =
        unsafe { ALLOCATE!(*mut ObjUpvalue, unsafe { (*function).upvalueCount } as usize) };
    for mut i in 0..unsafe { (*function).upvalueCount } {
        unsafe { *upvalues.offset(i) = null_mut() };
    }

//< allocate-upvalue-array
    let mut closure: *mut ObjClosure = unsafe { ALLOCATE_OBJ!(ObjClosure, OBJ_CLOSURE) };
    unsafe { (*closure).function = function };
//> init-upvalue-fields
    unsafe { (*closure).upvalues = upvalues };
    unsafe { (*closure).upvalueCount = unsafe { (*function).upvalueCount } };
//< init-upvalue-fields
    return closure;
}
//< Closures new-closure
//> Calls and Functions new-function
pub unsafe fn newFunction() -> *mut ObjFunction {
    let mut function: *mut ObjFunction = unsafe { ALLOCATE_OBJ!(ObjFunction, OBJ_FUNCTION) };
    unsafe { (*function).arity = 0 };
//> Closures init-upvalue-count
    unsafe { (*function).upvalueCount = 0 };
//< Closures init-upvalue-count
    unsafe { (*function).name = null_mut() };
    unsafe { initChunk(unsafe { &mut (*function).chunk } as *mut Chunk) };
    return function;
}
//< Calls and Functions new-function
//> Classes and Instances new-instance
pub unsafe fn newInstance(mut class: *mut ObjClass) -> *mut ObjInstance {
    let mut instance: *mut ObjInstance = unsafe { ALLOCATE_OBJ!(ObjInstance, OBJ_INSTANCE) };
    unsafe { (*instance).class = class };
    unsafe { initTable(unsafe { &mut (*instance).fields } as *mut Table) };
    return instance;
}
//< Classes and Instances new-instance
//> Calls and Functions new-native
pub fn newNative(mut function: NativeFn) -> *mut ObjNative {
    let mut native: *mut ObjNative = unsafe { ALLOCATE_OBJ!(ObjNative, OBJ_NATIVE) };
    unsafe { (*native).function = function };
    return native;
}
//< Calls and Functions new-native

/* Strings allocate-string < Hash Tables allocate-string
unsafe fn allocateString(mut chars: *mut u8, mut length: isize) -> *mut ObjString {
*/
//> allocate-string
//> Hash Tables allocate-string
unsafe fn allocateString(mut chars: *mut u8, mut length: isize,
        mut hash: u32) -> *mut ObjString {
//< Hash Tables allocate-string
    let mut string: *mut ObjString = unsafe { ALLOCATE_OBJ!(ObjString, OBJ_STRING) };
    unsafe { (*string).length = length };
    unsafe { (*string).chars = chars };
//> Hash Tables allocate-store-hash
    unsafe { (*string).hash = hash };
//< Hash Tables allocate-store-hash
//> Hash Tables allocate-store-string
//> Garbage Collection push-string

    unsafe { push(OBJ_VAL!(string)) };
//< Garbage Collection push-string
    let _ = unsafe { tableSet(unsafe { &mut vm.strings } as *mut Table, string, NIL_VAL!()) };
//> Garbage Collection pop-string
    let _ = unsafe { pop() };

//< Garbage Collection pop-string
//< Hash Tables allocate-store-string
    return string;
}
//< allocate-string
//> Hash Tables hash-string
unsafe fn hashString(mut key: *const u8, mut length: isize) -> u32 {
    let mut hash: u32 = 2166136261u32;
    for mut i in 0..length {
        hash ^= unsafe { *key.offset(i) as u32 };
        hash = u32::overflowing_mul(hash, 16777619).0;
    }
    return hash;
}
//< Hash Tables hash-string
//> take-string
pub unsafe fn takeString(mut chars: *mut u8, mut length: isize) -> *mut ObjString {
/* Strings take-string < Hash Tables take-string-hash
    return unsafe { allocateString(chars, length) };
*/
//> Hash Tables take-string-hash
    let mut hash: u32 = unsafe { hashString(chars as *const u8, length) };
//> take-string-intern
    let mut interned: *mut ObjString = unsafe { tableFindString(unsafe { &mut vm.strings } as *mut Table,
        chars, length, hash) };
    if !interned.is_null() {
        let _ = unsafe { FREE_ARRAY!(u8, chars, length + 1) };
        return interned;
    }

//< take-string-intern
    return unsafe { allocateString(chars, length, hash) };
//< Hash Tables take-string-hash
}
//< take-string
pub unsafe fn copyString(mut chars: *const u8, mut length: isize) -> *mut ObjString {
//> Hash Tables copy-string-hash
    let mut hash: u32 = unsafe { hashString(chars, length) };
//> copy-string-intern
    let mut interned: *mut ObjString = unsafe { tableFindString(unsafe { &mut vm.strings } as *mut Table,
        chars, length, hash) };
    if !interned.is_null() { return interned; }

//< copy-string-intern
//< Hash Tables copy-string-hash
    let mut heapChars: *mut u8 = unsafe { ALLOCATE!(u8, (length + 1) as usize) };
    unsafe { copy_nonoverlapping(chars, heapChars, length as usize) };
    unsafe { *heapChars.offset(length) = b'\0' };
/* Strings object-c < Hash Tables copy-string-allocate
    return unsafe { allocateString(heapChars, length) };
*/
//> Hash Tables copy-string-allocate
    return unsafe { allocateString(heapChars, length, hash) };
//< Hash Tables copy-string-allocate
}
//> Closures new-upvalue
pub unsafe fn newUpvalue(mut slot: *mut Value) -> *mut ObjUpvalue {
    let mut upvalue: *mut ObjUpvalue = unsafe { ALLOCATE_OBJ!(ObjUpvalue, OBJ_UPVALUE) };
//> init-closed
    unsafe { (*upvalue).closed = NIL_VAL!() };
//< init-closed
    unsafe { (*upvalue).location = slot };
//> init-next
    unsafe { (*upvalue).next = null_mut() };
//< init-next
    return upvalue;
}
//< Closures new-upvalue
//> Calls and Functions print-function-helper
unsafe fn printFunction(mut function: *mut ObjFunction) {
//> print-script
    if unsafe { (*function).name }.is_null() {
        print!("<script>");
        return;
    }
//< print-script
    print!("<fn {}>", unsafe { str_from_raw_parts!(unsafe { (*(*function).name).chars }, unsafe { (*(*function).name).length }) });
}
//< Calls and Functions print-function-helper
//> print-object
pub unsafe fn printObject(mut value: Value) {
    match unsafe { OBJ_TYPE!(value.clone()) } {
//> Methods and Initializers print-bound-method
        OBJ_BOUND_METHOD => {
            unsafe { printFunction(unsafe { (*(*unsafe { AS_BOUND_METHOD!(value) }).method).function }) };
        }
//< Methods and Initializers print-bound-method
//> Classes and Instances print-class
        OBJ_CLASS => {
            let mut name: *mut ObjString = unsafe { (*unsafe { AS_CLASS!(value) }).name };
            print!("{}", unsafe { str_from_raw_parts!(unsafe { (*name).chars }, unsafe { (*name).length }) });
        }
//< Classes and Instances print-class
//> Closures print-closure
        OBJ_CLOSURE => {
            unsafe { printFunction(unsafe { (*unsafe { AS_CLOSURE!(value) }).function }) };
        }
//< Closures print-closure
//> Calls and Functions print-function
        OBJ_FUNCTION => {
            unsafe { printFunction(unsafe { AS_FUNCTION!(value) }) };
        }
//< Calls and Functions print-function
//> Classes and Instances print-instance
        OBJ_INSTANCE => {
            let mut name: *mut ObjString = unsafe { (*(*unsafe { AS_INSTANCE!(value) }).class).name };
            print!("{} instance", unsafe { str_from_raw_parts!(unsafe { (*name).chars }, unsafe { (*name).length }) });
        }
//< Classes and Instances print-instance
//> Calls and Functions print-native
        OBJ_NATIVE => {
            print!("<native fn>");
        }
//< Calls and Functions print-native
        OBJ_STRING => {
            print!("{}", unsafe { AS_STR!(value) });
        }
//> Closures print-upvalue
        OBJ_UPVALUE => {
            print!("upvalue");
        }
//< Closures print-upvalue
    };
}
//< print-object
