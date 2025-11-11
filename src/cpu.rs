//using this instruction set http://www.6502.org/users/obelisk/6502/instructions.html
//other resources used: https://www.nesdev.org/wiki/Nesdev_Wiki
//test roms: https://www.nesdev.org/wiki/Emulator_tests
//Made by Riley Woodruff

//this file is the cpu control flow in the 6502 processor


//mod

//TODO: uncomment and combine all into cpu
use crate::registers::{Registers, StatusFlags};
//use crate::memory::{};
//use crate:: registers::{};


pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    //TODO
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
        }
    }
}