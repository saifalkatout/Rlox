use std::{
    alloc::{Layout,realloc, dealloc},
    os::raw::c_void,
    ptr::{drop_in_place, null_mut},
};
pub fn grow_capacity(cap: u128) -> u128 {
    if cap == 1 {
        8
    } else {
        cap * 2
    }
}

pub fn grow_array(T: &str, pointer: *mut c_void, oldSize: u128, newSize: u128) -> *mut c_void {

    match T {
        "u8" => reallocate(T, pointer, oldSize, newSize),
        "u16" => reallocate(T, pointer, 2 * (oldSize), 2 * (newSize)),
        "u32" => reallocate(T, pointer, 4 * (oldSize), 4 * (newSize)),
        "u64" => reallocate(T, pointer, 8 * (oldSize), 8 * (newSize)),
        _ => panic!("Type not found"),
    }
}
pub fn free_array(T: &str, oldSize: u128, pointer: *mut c_void) {
    //do we need oldsize ??
    let _newsize = 0;
    reallocate(T, pointer, oldSize, _newsize);
} //write these 3 functions as macros ?? Also string comparisons are sloooow, but whatev I'll find type enum soon
fn reallocate(T: &str, pointer: *mut c_void, _oldSize: u128, newSize: u128) -> *mut c_void {

    if newSize == 0 {

        unsafe {
            drop_in_place(pointer);
            match T {
                "u8" => dealloc(pointer as *mut u8, Layout::new::<u8>()), //causing double free
                _ => panic!("Type not found"),
            }
        }; //Very risky, now is working but super ugly/slow

        return null_mut();
    }
    // let lo: Result<Layout, std::alloc::LayoutError> = Layout::from_size_align(newSize, 8); //no need for layout in libc
    let result: *mut u8 = unsafe { realloc(pointer as *mut u8, Layout::new::<u8>(), newSize as usize) }; //What about not using libc ?? I feel like I'm only translating... (urns out it doesn't make a difference both are challenging/ this is creating a seg fault)
    if result == null_mut() {
        std::process::exit(1);
    } //here used rust exit instead of C exit (how do they differ ??)

    result as *mut c_void
}
