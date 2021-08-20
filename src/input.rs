use std::io;

use crate::turns::{Turns};
use crate::board::Board;
use crate::utils::clear;

pub fn get_input(board:&Board) -> Option<u8>{
    let mut c_input = String::new();
    
    //Read from stdin to buffer: c_input
    io::stdin()
        .read_line(&mut c_input)
        .expect("Invalid input!");

    
    //trim whitespaces from the buffer(c_input) and try to parse it to u8
    let c_input:Option<u8> = match c_input.trim().parse(){
        
        //Floor value of n to 2 since Max row/col index of the board is 2
        Ok(n) => if n > 2 {clear(); board.display_board(); None} else {Some(n)},
        Err(_) => None,
    };

    c_input
}