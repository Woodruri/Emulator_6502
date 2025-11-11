//using this instruction set http://www.6502.org/users/obelisk/6502/instructions.html
//other resources used: https://www.nesdev.org/wiki/Nesdev_Wiki
//test roms: https://www.nesdev.org/wiki/Emulator_tests
//Made by Riley Woodruff

//this file handles the registers and flags inside the 6502

//imports
use bitflags::bitflags;


//Consts
/*
6502 has the following flags, each one bit:
Bit 0: Carry flag: set if last instruction caused overflow from bit 7 or underflow from bit 0
Bit 1:Zero flag: set if result of last op was 0
Bit 2:Interrupt disable: set if SEI (set interrupt disable instruction). While set, will ignore 
    interrupts until CLI instruction is called
Bit 3:Decimal mode: set with SED (Set decimal flag) and disabled with CLD (Clear decimal flag) instructions
    while set, processor uses Binary Coded Decimal arithmatic during add or sub 
Bit 4:Break Command: Set when BRK instruction called and interrupt generated to process it
Bit 5: Always set 1, unused
Bit 6:Overflow flag: set if during arithmetic op, invalid 2s complement is returned
Bit 7:Negative flag: set if result of last op had a 7th bit set to 1
*/
//These are all the bitmasks for the flags, to be used inside of the status register

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusFlags: u8 {
        const CARRY_FLAG = 0b0000_0001;
        const ZERO_FLAG = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE = 0b0000_1000;
        const BREAK_COMMAND = 0b0001_0000;
        const UNUSED_FLAG = 0b0010_0000; //don't use this, it's there for accuracy
        const OVERFLOW_FLAG = 0b0100_0000;
        const NEGATIVE_FLAG = 0b1000_0000;
    }
}


/*6502 has the following registers:
    Program Counter: 16 bit
    Stack Pointer: 8 bit, supports stack from 0x0100 to 0x01FF and does not have overflow protection
    Accumulator: 8 bit
    Index X: 8 bit
    Index Y: 8 bit
    Processor Status: 8 bits for StatusFlags above
*/
pub struct Registers {
    prog_c: u16, //Program Counter
    stack_p: u8, //Stack Pointer
    a: u8, //Accumulator
    x: u8, 
    y: u8,
    status: StatusFlags, // Processor Status
}

//implempentation
impl Registers {

    pub fn new() -> Self {
        Registers {
            prog_c: 0,
            stack_p: 0xFD, //SP starts at 0xFD and grows down
            a: 0,
            x: 0,
            y: 0,
            status: StatusFlags::from_bits_truncate(0x0) //processor status is located at 0x24
        }
    }

    //flag instructions
    pub fn set_flag(&mut self, flag: StatusFlags, to_set: bool) {
        /*
        Sets status flags
        flag: StatusFlags is the flag to set
        to_set: bool is whether to set or unset the flag. false means unset, true means set
        */

        //making more verbose so I don't accidentally confuse myself
        if to_set == true{
            self.status.insert(flag);
        }
        else {
            self.status.remove(flag);
        }
    }

    pub fn get_flag(&self, flag: StatusFlags) -> bool {
        /*
        Returns if flag arg is set or not
        flag: The flag that we are checking the status of
         */
        self.status.contains(flag)
    }

    //Load/Store instructions
    
        
}
