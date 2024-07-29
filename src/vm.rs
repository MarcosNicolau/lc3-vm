use std::io::{stdin, Read};

use crate::{
    cond_flag::CondFlag,
    instructions::OpCode,
    register::{MMRegister, Register},
    utils::*,
};

pub const PC_START: u16 = 0x3000;
// 1 << 16 = 1 * 2^16 = 65336
const MEMORY_SIZE: usize = 1 << 16;

#[derive(Debug)]
pub enum VMError {
    OPCodeDoesNotExist,
    TrapDoesNotExist,
}

pub struct VM {
    registers: [u16; 10],
    // memory is organized in frames of 16 bits or 2 bytes
    memory: [u16; MEMORY_SIZE],
    exited: bool,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            registers: [0; 10],
            memory: [0; 1 << 16],
            exited: false,
        }
    }
}

impl VM {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_register(&self, register: u16) -> u16 {
        self.registers[register as usize]
    }

    pub fn set_register(&mut self, register: u16, value: u16) {
        self.registers[register as usize] = value;
    }

    pub fn write_to_memory(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }

    pub fn read_from_memory(&mut self, address: u16) -> u16 {
        if address == MMRegister::KBSR as u16 {
            self.check_keyboard_status();
        }

        self.memory[address as usize]
    }

    fn check_keyboard_status(&mut self) {
        let mut buffer = [0; 1];
        stdin().read_exact(&mut buffer).unwrap();

        if buffer[0] != 0 {
            self.write_to_memory(MMRegister::KBSR as u16, 1 << 15);
            self.write_to_memory(MMRegister::KBDR as u16, buffer[0] as u16);
        } else {
            self.write_to_memory(MMRegister::KBSR as u16, 0)
        }
    }

    fn increment_pc(&mut self) {
        self.registers[Register::PC as usize] += 1;
    }

    /**
     * # Arguments
     *  * `file` - is an absolute path
     */
    fn load_assembly(&mut self, origin: usize, file_path: &str) {
        // we'll load the whole file into memory
        read_file_as_u16(file_path, &mut self.memory, origin);
    }

    fn fetch_and_decode(&mut self) -> (OpCode, u16) {
        let pc = self.get_register(Register::PC as u16);
        let instr = self.read_from_memory(pc);

        return (OpCode::from_raw_instr(instr), instr);
    }

    fn execute(&mut self, op_code: OpCode, raw_instr: u16) -> Result<(), VMError> {
        op_code.execute(raw_instr, self)
    }

    pub fn set_cond_flags(&mut self, register: u16) {
        let value = self.get_register(register);
        let cond_flag = match value {
            0 => CondFlag::ZRO,
            val if val >> 15 != 0 => CondFlag::NEG,
            _ => CondFlag::POS,
        };

        self.set_register(Register::COND as u16, cond_flag as u16);
    }

    pub fn exit(&mut self) {
        self.exited = true;
    }

    pub fn run(&mut self, file_path: &str) {
        self.set_register(Register::PC as u16, PC_START);
        self.set_cond_flags(Register::R0 as u16);
        self.load_assembly(self.get_register(Register::PC as u16) as usize, file_path);

        disable_input_buffering();

        loop {
            let (op_code, instr) = self.fetch_and_decode();
            self.increment_pc();
            self.execute(op_code, instr).unwrap();

            if self.exited {
                break;
            }
        }

        restore_input_buffering();
    }
}
