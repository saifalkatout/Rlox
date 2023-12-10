use core::panic;
use std::os::raw::c_void;

use crate::chunk::{Chunk, OpCode};
use crate::chunk::OpCode::{OpReturn,OpSaif};

pub fn disassembleChunk(c: &mut Chunk, name: &str){ //changing &str to String will make c.code point to the first value of the string. NEVER USING STRING AGAIN
    print!("== {} ==\n", name);
    let mut offset: u128 = 0;
    while offset < c.getCount() {
        offset = disassembleInstruction(c, offset);
    }
}

fn disassembleInstruction(c: &Chunk, offset: u128) -> u128{
    print!("{:0>4} ", offset);
    //unsafe {panic!("{:?}", *(c.getCode()))};

    let inst: u8 = unsafe { *(c.getCode())};

    match inst {
        0 => simpleInstruction("OP_RETURN".to_string(), offset),
        1 => simpleInstruction("OP_SAIF".to_string(), offset),
        _ => {
            print!("Unknown opcode {}\n", inst);
            return offset + 1;
        }

    }

}

fn simpleInstruction(name: String, offset: u128) -> u128 {
    print!("{}\n", name);
    return offset + 1;
}