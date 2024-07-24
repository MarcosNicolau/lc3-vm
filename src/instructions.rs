use crate::vm::{VMError, VM};

pub enum OpCode {
    BR = 0, /* branch */
    ADD,    /* add  */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* unused */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump */
    RES,    /* reserved (unused) */
    LEA,    /* load effective address */
    TRAP,   /* execute trap */
}

impl OpCode {
    pub fn from_raw_instr(raw_instr: u16) -> OpCode {
        let instr = raw_instr & 0xF000;

        match instr {
            0 => OpCode::BR,
            1 => OpCode::ADD,
            2 => OpCode::LD,
            3 => OpCode::ST,
            4 => OpCode::JSR,
            5 => OpCode::AND,
            6 => OpCode::LDR,
            7 => OpCode::STR,
            8 => OpCode::RTI,
            9 => OpCode::NOT,
            10 => OpCode::LDI,
            11 => OpCode::STI,
            12 => OpCode::JMP,
            13 => OpCode::RES,
            14 => OpCode::LEA,
            15 => OpCode::TRAP,
            _ => panic!("instruction had invalid code"),
        }
    }
    /**
     * # Arguments
     * * `instr` - The full 16-bit instruction to be executed. Each operation will read and interpret
     *             relevant parts of this instruction.
     */
    pub fn execute(&self, raw_instr: u16, vm: &mut VM) -> Result<(), VMError> {
        match self {
            _ => Ok(()),
            // OpCode::BR => println!("Executing branch operation"),
            // OpCode::ADD => println!("Executing add operation"),
            // OpCode::LD => println!("Executing load operation"),
            // OpCode::ST => println!("Executing store operation"),
            // OpCode::JSR => println!("Executing jump register operation"),
            // OpCode::AND => println!("Executing bitwise and operation"),
            // OpCode::LDR => println!("Executing load register operation"),
            // OpCode::STR => println!("Executing store register operation"),
            // OpCode::RTI => println!("Executing unused operation"),
            // OpCode::NOT => println!("Executing bitwise not operation"),
            // OpCode::LDI => println!("Executing load indirect operation"),
            // OpCode::STI => println!("Executing store indirect operation"),
            // OpCode::JMP => println!("Executing jump operation"),
            // OpCode::RES => println!("Executing reserved operation"),
            // OpCode::LEA => println!("Executing load effective address operation"),
            // OpCode::TRAP => println!("Executing trap operation"),
        }
    }
}
