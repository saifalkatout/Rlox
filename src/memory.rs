use libc::realloc;
use std::{
    alloc::{dealloc, Layout},
    os::raw::c_void,
    ptr::{drop_in_place, null_mut},
};

pub fn grow_capacity(cap: usize) -> usize {
    if cap == 8 {
        8
    } else {
        cap * 2
    }
}

pub fn grow_array(T: &str, pointer: *mut c_void, oldSize: usize, newSize: usize) -> *mut c_void {
    match T {
        "u8" => reallocate(T, pointer, oldSize, newSize),
        "u16" => reallocate(T, pointer, 2 * (oldSize), 2 * (newSize)),
        "u32" => reallocate(T, pointer, 4 * (oldSize), 4 * (newSize)),
        "u64" => reallocate(T, pointer, 8 * (oldSize), 8 * (newSize)),
        _ => panic!("Type not found"),
    }
}
pub fn free_array(T: &str, pointer: *mut c_void, oldSize: usize) {
    //do we need oldsize ??
    reallocate(T, pointer, oldSize, 0);
} //write these 3 functions as macros ?? Also string comparisons are sloooow, but whatev I'll find type enum soon
fn reallocate(T: &str, pointer: *mut c_void, _oldSize: usize, newSize: usize) -> *mut c_void {
    if newSize == 0 {
        unsafe {
            drop_in_place(pointer);
            match T {
                "u8" => dealloc(pointer as *mut u8, Layout::new::<u8>()),
                "u16" => dealloc(pointer as *mut u8, Layout::new::<u16>()),
                "u32" => dealloc(pointer as *mut u8, Layout::new::<u32>()),
                "u64" => dealloc(pointer as *mut u8, Layout::new::<u64>()),
                _ => panic!("Type not found"),
            }
        }; //Very risky, now is working but super ugly/slow
        return null_mut();
    }
    let _lo = Layout::from_size_align(newSize, 8); //no need for layout in libc
    let result: *mut c_void = unsafe { realloc(pointer, newSize) }; //What about not using libc ?? I feel like I'm only translating...
    if result == null_mut() {
        std::process::exit(1);
    } //here used rust exit instead of C exit (how do they differ ??)

    result
}
