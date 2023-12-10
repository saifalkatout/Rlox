mod chunk;
mod common;
mod memory;
mod debug;
use std::env;


use crate::chunk::OpCode::{OpReturn,OpSaif};
use chunk::freeChunk;
use chunk::Chunk;

use chunk::WriteToChunk;
use debug::disassembleChunk;
fn main() {
    let _args: Vec<String> = env::args().collect();
    // let argc = &args[1];
    // let argv = &args[2];

    let mut chunk = Chunk::default();

    WriteToChunk(&mut chunk, OpSaif as u8);
    WriteToChunk(&mut chunk, OpSaif as u8);
    disassembleChunk(&mut chunk, "saif chunk");


    freeChunk(&mut chunk); //what is mutable borrowing ?

}
