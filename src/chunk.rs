use std::{self, os::raw::c_void};

use crate::memory::{free_array, grow_array, grow_capacity};
pub enum OpCode {
    OpReturn,
    OpSaif,
}

pub struct Chunk {
    code: *mut u8,
    count: usize,
    capacity: usize,
}

impl Chunk {
    pub fn getCode(&self) -> *mut u8 {
        self.code
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            code: Vec::with_capacity(8).as_mut_ptr(), //Using Vac::new here is not suggested, as we need to allocate first then reallocate, check with darwish on this
            count: 0,
            capacity: 8,
        }
    }
}

pub fn CreateChunk(c: &mut Chunk) {
    //I think just mut is sufficient, not sure (turns out not enough)
    c.code = Vec::new().as_mut_ptr()
}

pub fn WriteToChunk(c: &mut Chunk, byte: u8) {
    
    if c.capacity < c.count + 1 {
        let oldCapacity = c.capacity;
        c.capacity = grow_capacity(oldCapacity);
        c.code = grow_array("u8", c.code as *mut c_void, oldCapacity, c.capacity) as *mut u8;
        //This is all rust stuff
    }

    unsafe { *(c.code).add(c.count) = byte };
    c.count += 1;
}

pub fn freeChunk(c: &mut Chunk) {
    
    free_array("u8", c.code as *mut c_void, c.capacity);
    *c = Chunk::default(); 
}
