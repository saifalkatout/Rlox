use std::{os::raw::c_void, ptr::drop_in_place, u128};

use crate::memory::{grow_capacity, grow_array};

pub type Value = f64;
#[derive(Debug, Clone, Copy)]
pub struct ValueArray {
    capacity: u128,
    count: u128,
    values: *mut Value
}

impl ValueArray {
    pub fn getCount(&self) -> u128 {
        self.count
    }

    pub fn getValue(&self, index: usize) -> Value {
        unsafe { *(self.values.add(index)) }
    }
}

impl Default for ValueArray {
    fn default() -> Self {
        Self {
            values: Vec::with_capacity(2).as_mut_ptr(), //Using Vac::new here is not suggested, as we need to allocate first then reallocate
            count: 0,
            capacity: 1,
        }
    }
}

pub fn WriteValueArray(v: &mut ValueArray, val: Value) {
    if v.capacity < v.count + 1 {
        let oldCapacity = v.capacity;
        v.capacity = grow_capacity(oldCapacity);
        v.values = grow_array("u8", v.values as *mut c_void, oldCapacity, v.capacity) as *mut f64;
        //This is all rust stuff
    }

    unsafe { *(v.values).add(v.count.try_into().unwrap()) = val };

    v.count += 1;
}

pub fn free_value_array(v: &mut ValueArray) {
    //free_array("u8",c.capacity, c.getCode() as *mut c_void); causes double free for the code pointer and cannot send object pointer because cant be converted to c_void
    unsafe { drop_in_place(v) }; //make sure this is deleting the chunk and the data pointer in sanitizer (it is not)
    *v = ValueArray::default(); 
}

pub fn printValue(val: Value) {
    print!("`{}", val);
  }
