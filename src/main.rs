mod chunk;
mod common;
mod memory;
use std::env;


use crate::chunk::OpCode::OpReturn;
use chunk::freeChunk;
use chunk::Chunk;

use chunk::WriteToChunk;
fn main() {
    let _args: Vec<String> = env::args().collect();
    // let argc = &args[1];
    // let argv = &args[2];

    let mut chunk = Chunk::default();
    WriteToChunk(&mut chunk, OpReturn as u8);
    //freeChunk(&mut chunk); //what is mutable borrowing ?
}
