//> Chunks of Bytecode memory-c
use ::alloc::alloc::*;
//> Garbage Collection add-to-gray-stack
use ::core::mem::*;
//< Garbage Collection add-to-gray-stack
use ::core::ptr::*;
use ::std::process::*;

//> Garbage Collection memory-include-compiler
use crate::compiler::*;
//< Garbage Collection memory-include-compiler
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
//> Garbage Collection mark-object-h
// no need to forward declare markObject
//< Garbage Collection mark-object-h
//> Garbage Collection mark-value-h
// no need to forward declare markValue
//< Garbage Collection mark-value-h
//> Garbage Collection collect-garbage-h
// no need to forward declare collectGarbage
//< Garbage Collection collect-garbage-h
//> Strings free-objects-h
// no need to forward declare freeObjects
//< Strings free-objects-h
//< Chunks of Bytecode memory-h
//> Strings memory-include-vm
//> Garbage Collection memory-include-compiler
#[allow(unused_imports)]
//< Garbage Collection memory-include-compiler
use crate::vm::*;
//< Strings memory-include-vm
//> Garbage Collection debug-log-includes

#[cfg(DEBUG_LOG_GC)]
use ::std::*;
#[cfg(DEBUG_LOG_GC)]
#[allow(unused_imports)]
use crate::debug::*;
//< Garbage Collection debug-log-includes
//> Garbage Collection heap-grow-factor

const GC_HEAP_GROW_FACTOR: usize = 2;
//< Garbage Collection heap-grow-factor

pub unsafe fn reallocate(mut pointer: *mut u8, mut oldSize: usize, mut newSize: usize) -> *mut u8 {
//> Garbage Collection updated-bytes-allocated
    unsafe {
        vm.bytesAllocated -= oldSize;
        vm.bytesAllocated += newSize;
    };
//< Garbage Collection updated-bytes-allocated
//> Garbage Collection call-collect
    if newSize > oldSize {
        #[cfg(DEBUG_STRESS_GC)]
        unsafe { collectGarbage() };
//> collect-on-next

        if unsafe { vm.bytesAllocated } > unsafe { vm.nextGC } {
            unsafe { collectGarbage() };
        }
//< collect-on-next
    }

//< Garbage Collection call-collect
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
//> Garbage Collection mark-object
pub unsafe fn markObject(mut object: *mut Obj) {
    if object.is_null() { return; }
//> check-is-marked
    if unsafe { (*object).isMarked } { return; }

//< check-is-marked
//> log-mark-object
    #[cfg(DEBUG_LOG_GC)]
    {
        print!("{:p} mark ", object);
        unsafe { printValue(OBJ_VAL!(object)) };
        print!("\n");
    }

//< log-mark-object
    unsafe { (*object).isMarked = true };
//> add-to-gray-stack

    if unsafe { vm.grayCapacity } < unsafe { vm.grayCount } + 1 {
        let mut layout: Layout = Layout::array::<*mut Obj>(unsafe { vm.grayCapacity } as usize).unwrap();
        unsafe { vm.grayCapacity = GROW_CAPACITY!(unsafe { vm.grayCapacity }) };
        let mut newSize: usize = size_of::<*mut Obj>() * unsafe { vm.grayCapacity } as usize;
        unsafe { vm.grayStack = realloc(unsafe { vm.grayStack } as *mut u8, layout, newSize) as *mut *mut Obj };
//> exit-gray-stack

        if unsafe { vm.grayStack }.is_null() { exit(1) };
//< exit-gray-stack
    }

    unsafe { *vm.grayStack.offset(unsafe { vm.grayCount }) = object };
    unsafe { vm.grayCount += 1; }
//< add-to-gray-stack
}
//< Garbage Collection mark-object
//> Garbage Collection mark-value
pub unsafe fn markValue(mut value: Value) {
    if IS_OBJ!(value) { unsafe { markObject(unsafe { AS_OBJ!(value) }) }; }
}
//< Garbage Collection mark-value
//> Garbage Collection mark-array
unsafe fn markArray(mut array: *mut ValueArray) {
    for mut i in 0..unsafe { (*array).count } {
        unsafe { markValue(unsafe { (*(*array).values.offset(i)).clone() }) };
    }
}
//< Garbage Collection mark-array
//> Garbage Collection blacken-object
unsafe fn blackenObject(mut object: *mut Obj) {
//> log-blacken-object
    #[cfg(DEBUG_LOG_GC)]
    {
        print!("{:p} blacken ", object);
        unsafe { printValue(OBJ_VAL!(object)) };
        print!("\n");
    }

//< log-blacken-object
    match unsafe { (*object).r#type.clone() } {
//> Methods and Initializers blacken-bound-method
        OBJ_BOUND_METHOD => {
            let mut bound: *mut ObjBoundMethod = object as *mut ObjBoundMethod;
            unsafe { markValue(unsafe { (*bound).receiver.clone() }) };
            unsafe { markObject(unsafe { (*bound).method } as *mut Obj) };
        }
//< Methods and Initializers blacken-bound-method
//> Classes and Instances blacken-class
        OBJ_CLASS => {
            let mut class: *mut ObjClass = object as *mut ObjClass;
            unsafe { markObject(unsafe { (*class).name } as *mut Obj) };
//> Methods and Initializers mark-methods
            unsafe { markTable(unsafe { &mut (*class).methods } as *mut Table) };
//< Methods and Initializers mark-methods
        }
//< Classes and Instances blacken-class
//> blacken-closure
        OBJ_CLOSURE => {
            let mut closure: *mut ObjClosure = object as *mut ObjClosure;
            unsafe { markObject(unsafe { (*closure).function } as *mut Obj) };
            for mut i in 0..unsafe { (*closure).upvalueCount } {
                unsafe { markObject(unsafe { *(*closure).upvalues.offset(i) } as *mut Obj) };
            }
        }
//< blacken-closure
//> blacken-function
        OBJ_FUNCTION => {
            let mut function: *mut ObjFunction = object as *mut ObjFunction;
            unsafe { markObject(unsafe { (*function).name } as *mut Obj) };
            unsafe { markArray(unsafe { &mut (*function).chunk.constants } as *mut ValueArray) };
        }
//< blacken-function
//> Classes and Instances blacken-instance
        OBJ_INSTANCE => {
            let mut instance: *mut ObjInstance = object as *mut ObjInstance;
            unsafe { markObject(unsafe { (*instance).class } as *mut Obj) };
            unsafe { markTable(unsafe { &mut (*instance).fields } as *mut Table) };
        }
//< Classes and Instances blacken-instance
//> blacken-upvalue
        OBJ_UPVALUE => {
            unsafe { markValue(unsafe { (*(object as *mut ObjUpvalue)).closed.clone() }) };
        }
//< blacken-upvalue
        OBJ_NATIVE => {}
        OBJ_STRING => {}
    }
}
//< Garbage Collection blacken-object
//> Strings free-object
unsafe fn freeObject(mut object: *mut Obj) {
//> Garbage Collection log-free-object
    #[cfg(DEBUG_LOG_GC)]
    print!("{:p} free type {}\n", object, unsafe { (*object).r#type.clone() } as u8);

//< Garbage Collection log-free-object
    match unsafe { (*object).r#type.clone() } {
//> Methods and Initializers free-bound-method
        OBJ_BOUND_METHOD => {
            let _ = unsafe { FREE!(ObjBoundMethod, object) };
        }
//< Methods and Initializers free-bound-method
//> Classes and Instances free-class
        OBJ_CLASS => {
//> Methods and Initializers free-methods
            let mut class: *mut ObjClass = object as *mut ObjClass;
            unsafe { freeTable(unsafe { &mut (*class).methods } as *mut Table) };
//< Methods and Initializers free-methods
            let _ = unsafe { FREE!(ObjClass, object) };
        } // [braces]
//< Classes and Instances free-class
//> Closures free-closure
        OBJ_CLOSURE => {
//> free-upvalues
            let mut closure: *mut ObjClosure = object as *mut ObjClosure;
            let _ = unsafe { FREE_ARRAY!(*mut ObjUpvalue, unsafe { (*closure).upvalues } as *mut u8,
                unsafe { (*closure).upvalueCount }) };
//< free-upvalues
            let _ = unsafe { FREE!(ObjClosure, object) };
        }
//< Closures free-closure
//> Calls and Functions free-function
        OBJ_FUNCTION => {
            let mut function: *mut ObjFunction = object as *mut ObjFunction;
            unsafe { freeChunk(unsafe { &mut (*function).chunk } as *mut Chunk) };
            let _ = unsafe { FREE!(ObjFunction, object) };
        }
//< Calls and Functions free-function
//> Classes and Instances free-instance
        OBJ_INSTANCE => {
            let mut instance: *mut ObjInstance = object as *mut ObjInstance;
            unsafe { freeTable(unsafe { &mut (*instance).fields } as *mut Table) };
            let _ = unsafe { FREE!(ObjInstance, object) };
        }
//< Classes and Instances free-instance
//> Calls and Functions free-native
        OBJ_NATIVE => {
            let _ = unsafe { FREE!(ObjNative, object) };
        }
//< Calls and Functions free-native
        OBJ_STRING => {
            let mut string: *mut ObjString = object as *mut ObjString;
            let _ = unsafe { FREE_ARRAY!(u8, unsafe { (*string).chars }, unsafe { (*string).length + 1 }) };
            let _ = unsafe { FREE!(ObjString, object) };
        }
//> Closures free-upvalue
        OBJ_UPVALUE => {
            let _ = unsafe { FREE!(ObjUpvalue, object) };
        }
//< Closures free-upvalue
    }
}
//< Strings free-object
//> Garbage Collection mark-roots
unsafe fn markRoots() {
    let mut slot: *mut Value = unsafe { &mut vm.stack } as *mut Value;
    while slot < unsafe { vm.stackTop } {
        unsafe { markValue(unsafe { (*slot).clone() }) };
        slot = unsafe { slot.offset(1) };
    }
//> mark-closures

    for mut i in 0..unsafe { vm.frameCount } {
        unsafe { markObject(unsafe { vm.frames[i as usize].closure } as *mut Obj) };
    }
//< mark-closures
//> mark-open-upvalues

    let mut upvalue: *mut ObjUpvalue = unsafe { vm.openUpvalues };
    while !upvalue.is_null() {
        unsafe { markObject(upvalue as *mut Obj) };
        upvalue = unsafe { (*upvalue).next };
    }
//< mark-open-upvalues
//> mark-globals

    unsafe { markTable(unsafe { &mut vm.globals } as *mut Table) };
//< mark-globals
//> call-mark-compiler-roots
    unsafe { markCompilerRoots() };
//< call-mark-compiler-roots
//> Methods and Initializers mark-init-string
    unsafe { markObject(unsafe { vm.initString } as *mut Obj) };
//< Methods and Initializers mark-init-string
}
//< Garbage Collection mark-roots
//> Garbage Collection trace-references
unsafe fn traceReferences() {
    while unsafe { vm.grayCount } > 0 {
        unsafe { vm.grayCount -= 1 };
        let mut object: *mut Obj = unsafe { *vm.grayStack.offset(unsafe { vm.grayCount }) };
        unsafe { blackenObject(object) };
    }
}
//< Garbage Collection trace-references
//> Garbage Collection sweep
unsafe fn sweep() {
    let mut previous: *mut Obj = null_mut();
    let mut object: *mut Obj = unsafe { vm.objects };
    while !object.is_null() {
        if unsafe { (*object).isMarked } {
//> unmark
            unsafe { (*object).isMarked = false };
//< unmark
            previous = object;
            object = unsafe { (*object).next };
        } else {
            let mut unreached: *mut Obj = object;
            object = unsafe { (*object).next };
            if !previous.is_null() {
                unsafe { (*previous).next = object };
            } else {
                unsafe { vm.objects = object };
            }

            unsafe { freeObject(unreached) };
        }
    }
}
//< Garbage Collection sweep
//> Garbage Collection collect-garbage
pub unsafe fn collectGarbage() {
//> log-before-collect
    #[cfg(DEBUG_LOG_GC)]
    print!("-- gc begin\n");
//> log-before-size
    #[cfg(DEBUG_LOG_GC)]
    let mut before: usize = unsafe { vm.bytesAllocated };
//< log-before-size
//< log-before-collect
//> call-mark-roots

    unsafe { markRoots() };
//< call-mark-roots
//> call-trace-references
    unsafe { traceReferences() };
//< call-trace-references
//> sweep-strings
    unsafe { tableRemoveWhite(unsafe { &mut vm.strings } as *mut Table) };
//< sweep-strings
//> call-sweep
    unsafe { sweep() };
//< call-sweep
//> update-next-gc

    unsafe { vm.nextGC = unsafe { vm.bytesAllocated } * GC_HEAP_GROW_FACTOR };
//< update-next-gc
//> log-after-collect

    #[cfg(DEBUG_LOG_GC)]
    print!("-- gc end\n");
//> log-collected-amount
    #[cfg(DEBUG_LOG_GC)]
    print!("   collected {} bytes (from {} to {}) next at {}\n",
        before - unsafe { vm.bytesAllocated }, before,
        unsafe { vm.bytesAllocated }, unsafe { vm.nextGC });
//< log-collected-amount
//< log-after-collect
}
//< Garbage Collection collect-garbage
//> Strings free-objects
pub unsafe fn freeObjects() {
    let mut object: *mut Obj = unsafe { vm.objects };
    while !object.is_null() {
        let mut next: *mut Obj = unsafe { (*object).next };
        unsafe { freeObject(object) };
        object = next;
    }
//> Garbage Collection free-gray-stack

    let mut layout: Layout = Layout::array::<*mut Obj>(unsafe { vm.grayCapacity as usize }).unwrap();
    unsafe { dealloc(unsafe { vm.grayStack } as *mut u8, layout) };
//< Garbage Collection free-gray-stack
}
//< Strings free-objects
