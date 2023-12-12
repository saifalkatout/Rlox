use core::panic;
use std::os::raw::c_void;

use crate::chunk::{Chunk, OpCode};
use crate::chunk::OpCode::{OpReturn,OpConstant};
use crate::value::printValue;

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
    if (offset > 0 &&
        unsafe { *(c.GetLines().add(offset as usize)) == *(c.GetLines().add((offset - 1) as usize))}) {
            print!("   | ");
    } else {
        unsafe { print!("{:>4} ", *(c.GetLines().add(offset as usize))); } 
    }


    let inst: u8 = unsafe { *(c.getCode().add(offset as usize))};

    match inst {
        0 => constantInstruction("OP_CONSTANT", offset, c),
        1 => simpleInstruction("OP_RETURN",  offset),
        _ => {
            print!("Unknown opcode {}\n", inst);
            return offset + 1;
        }

    }

}

fn simpleInstruction(name: &str, offset: u128) -> u128 {
    print!("{}\n", name);
    return offset + 1;
}

fn constantInstruction(name: &str, offset: u128, c: &Chunk) -> u128 {
    let constant: usize = unsafe { *(c.getCode().add((offset + 1).try_into().unwrap())) as usize };
    print!("{} {:>16} ", name, constant);
    printValue(c.GetConstants().getValue(constant as usize));
    print!("`\n");
    return offset + 2;
}