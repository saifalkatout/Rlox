mod chunk;
mod common;
mod memory;
mod debug;
mod value;
use std::env;


use crate::chunk::OpCode::{OpReturn,OpConstant};
use chunk::addConstant;
use chunk::freeChunk;
use chunk::Chunk;

use chunk::WriteToChunk;
use debug::disassembleChunk;
fn main() {
    let _args: Vec<String> = env::args().collect();
    // let argc = &args[1];
    // let argv = &args[2];

    let mut chunk = Chunk::default();

    let constant1 = addConstant(&mut chunk, 1.111);
    let constant2 = addConstant(&mut chunk, 3.145);
    WriteToChunk( OpConstant as u8, 123, &mut chunk);
    WriteToChunk( constant1 as u8, 123, &mut chunk);
    WriteToChunk( OpConstant as u8, 123, &mut chunk);
    WriteToChunk( constant2 as u8, 123, &mut chunk);
    WriteToChunk( OpReturn as u8, 0124, &mut chunk);

    disassembleChunk(&mut chunk, "saif chunk");


    //freeChunk(&mut chunk); //what is mutable borrowing ?

}
