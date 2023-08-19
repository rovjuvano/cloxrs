//> Hash Tables table-c
use ::core::ptr::*;

use crate::memory::*;
#[allow(unused_imports)]
use crate::object::*;
//> Hash Tables table-h
pub use crate::common::*;
pub use crate::value::*;
//> entry

#[derive(Clone)] // Copy too but made explicit
pub struct Entry {
    pub key: *mut ObjString,
    pub value: Value,
}
//< entry

#[derive(Clone)] // Copy too but made explicit
pub struct Table {
    pub count: isize,
    pub capacity: isize,
    pub entries: *mut Entry,
}

//> init-table-h
// no need to forward declare initTable
//> free-table-h
// no need to forward declare freeTable
//< free-table-h
//> table-get-h
// no need to forward declare tableGet
//< table-get-h
//> table-set-h
// no need to forward declare tableSet
//< table-set-h
//> table-delete-h
// no need to forward declare tableDelete
//< table-delete-h
//> table-add-all-h
// no need to forward declare tableAddAll
//< table-add-all-h
//> table-find-string-h
// no need to forward declare tableFindString
//< table-find-string-h
//< init-table-h
//< Hash Tables table-h
#[allow(unused_imports)]
use crate::value::*;

//> max-load
const TABLE_MAX_LOAD: f64 = 0.75;

//< max-load
pub unsafe fn initTable(mut table: *mut Table) {
    unsafe { (*table).count = 0 };
    unsafe { (*table).capacity = 0 };
    unsafe { (*table).entries = null_mut() };
}
//> free-table
pub unsafe fn freeTable(mut table: *mut Table) {
    let _ = unsafe { FREE_ARRAY!(Entry, unsafe { (*table).entries } as *mut u8, unsafe { (*table).capacity }) };
    unsafe { initTable(table) };
}
//< free-table
//> find-entry
//> omit
// NOTE: The "Optimization" chapter has a manual copy of this function.
// If you change it here, make sure to update that copy.
//< omit
unsafe fn findEntry(mut entries: *mut Entry, mut capacity: isize,
        mut key: *mut ObjString) -> *mut Entry {
    let mut index: u32 = unsafe { (*key).hash } % capacity as u32;
//> find-entry-tombstone
    let mut tombstone: *mut Entry = null_mut();

//< find-entry-tombstone
    loop {
        let mut entry: *mut Entry = unsafe { entries.offset(index as isize) };
/* Hash Tables find-entry < Hash Tables find-tombstone
    if unsafe { (*entry).key } == key || unsafe { (*entry).key }.is_null() {
        return entry;
    }
*/
//> find-tombstone
        if unsafe { (*entry).key }.is_null() {
            if IS_NIL!(*unsafe { &(*entry).value }) {
                // Empty entry.
                return if !tombstone.is_null() { tombstone } else { entry };
            } else {
                // We found a tombstone.
                if tombstone.is_null() { tombstone = entry; }
            }
        } else if unsafe { (*entry).key } == key {
            // We found the key.
            return entry;
        }
//< find-tombstone

        index = (index + 1) % capacity as u32;
    }
}
//< find-entry
//> table-get
#[allow(dead_code)]
pub unsafe fn tableGet(mut table: *mut Table, mut key: *mut ObjString, mut value: *mut Value) -> bool {
    if unsafe { (*table).count } == 0 { return false; }

    let mut entry: *mut Entry = unsafe { findEntry(unsafe { (*table).entries }, unsafe { (*table).capacity }, key) };
    if unsafe { (*entry).key }.is_null() { return false; }

    unsafe { *value = unsafe { (*entry).value.clone() } };
    return true;
}
//< table-get
//> table-adjust-capacity
unsafe fn adjustCapacity(mut table: *mut Table, mut capacity: isize) {
    let mut entries: *mut Entry = unsafe { ALLOCATE!(Entry, capacity as usize) };
    for mut i in 0..capacity {
        unsafe { (*entries.offset(i)).key = null_mut() };
        unsafe { (*entries.offset(i)).value = NIL_VAL!() };
    }
//> re-hash

//> resize-init-count
    unsafe { (*table).count = 0 };
//< resize-init-count
    for mut i in 0..unsafe { (*table).capacity } {
        let mut entry: *mut Entry = unsafe { (*table).entries.offset(i) };
        if unsafe { (*entry).key }.is_null() { continue; }

        let mut dest: *mut Entry = unsafe { findEntry(entries, capacity, unsafe { (*entry).key }) };
        unsafe { (*dest).key = unsafe { (*entry).key } };
        unsafe { (*dest).value = unsafe { (*entry).value.clone() } };
//> resize-increment-count
        unsafe { (*table).count += 1 };
//< resize-increment-count
    }
//< re-hash

//> free-old-array
    let _ = unsafe { FREE_ARRAY!(Entry, unsafe { (*table).entries } as *mut u8, unsafe { (*table).capacity }) };
//< free-old-array
    unsafe { (*table).entries = entries };
    unsafe { (*table).capacity = capacity };
}
//< table-adjust-capacity
//> table-set
pub unsafe fn tableSet(mut table: *mut Table, mut key: *mut ObjString, mut value: Value) -> bool {
//> table-set-grow
    if unsafe { (*table).count + 1 } as f64 > unsafe { (*table).capacity } as f64 * TABLE_MAX_LOAD {
        let mut capacity: isize = GROW_CAPACITY!(unsafe { (*table).capacity });
        unsafe { adjustCapacity(table, capacity) };
    }

//< table-set-grow
    let mut entry: *mut Entry = unsafe { findEntry(unsafe { (*table).entries }, unsafe { (*table).capacity }, key) };
    let mut isNewKey: bool = unsafe { (*entry).key }.is_null();
/* Hash Tables table-set < Hash Tables set-increment-count
    if isNewKey { unsafe { (*table).count += 1 }; }
*/
//> set-increment-count
    if isNewKey && IS_NIL!(*unsafe { &(*entry).value }) { unsafe { (*table).count += 1 }; }
//< set-increment-count

    unsafe { (*entry).key = key };
    unsafe { (*entry).value = value };
    return isNewKey;
}
//< table-set
//> table-delete
#[allow(dead_code)]
pub unsafe fn tableDelete(mut table: *mut Table, mut key: *mut ObjString) -> bool {
    if unsafe { (*table).count } == 0 { return false; }

    // Find the entry.
    let mut entry: *mut Entry = unsafe { findEntry(unsafe { (*table).entries }, unsafe { (*table).capacity }, key) };
    if unsafe { (*entry).key }.is_null() { return false; }

    // Place a tombstone in the entry.
    unsafe { (*entry).key = null_mut() };
    unsafe { (*entry).value = BOOL_VAL!(true) };
    return true;
}
//< table-delete
//> table-add-all
#[allow(dead_code)]
pub unsafe fn tableAddAll(mut from: *mut Table, mut to: *mut Table) {
    for mut i in 0..unsafe { (*from).capacity } {
        let mut entry: *mut Entry = unsafe { (*from).entries.offset(i) };
        if !unsafe { (*entry).key }.is_null() {
            let _ = unsafe { tableSet(to, unsafe { (*entry).key }, unsafe { (*entry).value.clone() }) };
        }
    }
}
//< table-add-all
//> table-find-string
pub unsafe fn tableFindString(mut table: *mut Table, mut chars: *const u8,
        mut length: isize, mut hash: u32) -> *mut ObjString {
    if unsafe { (*table).count } == 0 { return null_mut(); }

    let mut index: u32 = hash % unsafe { (*table).capacity } as u32;
    loop {
        let mut entry: *mut Entry = unsafe { (*table).entries.offset(index as isize) };
        if unsafe { (*entry).key }.is_null() {
            // Stop if we find an empty non-tombstone entry.
            if IS_NIL!(*unsafe { &(*entry).value }) { return null_mut(); }
        } else if unsafe { (*(*entry).key).length } == length &&
                unsafe { (*(*entry).key).hash } == hash &&
                unsafe { memcmp(unsafe { (*(*entry).key).chars }, chars, length as usize) } == 0 {
            // We found it.
            return unsafe { (*entry).key };
        }

        index = (index + 1) % unsafe { (*table).capacity } as u32;
    }
}
//< table-find-string
