use core::panic;
use std::{self, os::raw::c_void, ptr::drop_in_place};

use crate::{memory::{free_array, grow_array, grow_capacity}, value::{ValueArray, free_value_array, Value, WriteValueArray}};
pub enum OpCode {
    OpConstant,
    OpReturn,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    code: *mut u8,
    count: u128,
    capacity: u128,
    constants: ValueArray, 
    lines: *mut i128,
}

impl Chunk {
    pub fn getCode(&self) -> *mut u8 {
        self.code
    }

    pub fn getCount(&self) -> u128 {
        self.count
    }

    pub fn GetConstants(&self) -> ValueArray {
        self.constants
    }

    pub fn GetLines(&self) -> *mut i128 {
        self.lines
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            code: Vec::with_capacity(1).as_mut_ptr(), //Using Vac::new here is not suggested, as we need to allocate first then reallocate
            count: 0,
            capacity: 1,
            constants: ValueArray::default(), 
            lines: Vec::with_capacity(2).as_mut_ptr() //starting with 2, I feel i128 is diff from vec::with_cap return type (might be byte)

        }
    }
}

pub fn CreateChunk(c: &mut Chunk) {
    //I think just mut is sufficient, not sure (turns out not enough)
    c.code = Vec::new().as_mut_ptr()
}

pub fn WriteToChunk( byte: u8, line: i128, c: &mut Chunk) {
    if c.capacity < c.count + 1 {
        let oldCapacity = c.capacity;
        c.capacity = grow_capacity(oldCapacity);
        c.code = grow_array("u8", c.code as *mut c_void, oldCapacity, c.capacity) as *mut u8;
        c.lines = grow_array("i256", c.lines as *mut c_void, oldCapacity, c.capacity) as *mut i128
        //This is all rust stuff
    }

    unsafe { *(c.code).add(c.count.try_into().unwrap()) = byte; *(c.lines.add(c.count as usize)) = line };

    c.count += 1;
}

pub fn freeChunk(c: &mut Chunk) {
    //free_array("u8",c.capacity, c.getCode() as *mut c_void); causes double free for the code pointer 
    free_value_array(&mut c.constants);
    c.lines = Vec::new().as_mut_ptr();
    //unsafe { drop_in_place(c) }; // learn how to free memory from object pointer (this is not working)
    *c = Chunk::default(); 
}

pub fn addConstant(c: &mut Chunk, value: Value) -> u128 {
    WriteValueArray(&mut c.constants, value);
    return c.constants.getCount() - 1;
  }
