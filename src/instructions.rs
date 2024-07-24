use crate::{
    register::Register,
    traps::Trap,
    vm::{VMError, VM},
};

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
            0x0000 => OpCode::BR,
            0x1000 => OpCode::ADD,
            0x2000 => OpCode::LD,
            0x3000 => OpCode::ST,
            0x4000 => OpCode::JSR,
            0x5000 => OpCode::AND,
            0x6000 => OpCode::LDR,
            0x7000 => OpCode::STR,
            0x8000 => OpCode::RTI,
            0x9000 => OpCode::NOT,
            0xA000 => OpCode::LDI,
            0xB000 => OpCode::STI,
            0xC000 => OpCode::JMP,
            0xD000 => OpCode::RES,
            0xE000 => OpCode::LEA,
            0xF000 => OpCode::TRAP,
            _ => panic!("Invalid instruction"),
        }
    }

    /**
     * # Arguments
     * * `raw_instr` - The full 16-bit instruction to be executed. Each operation will read and interpret
     *                 relevant parts of this instruction.
     * * `vm` - The virtual machine on which to execute the operation.
     *
     * # Returns
     * A Result indicating success or a VMError if an operation fails.
     */
    pub fn execute(&self, raw_instr: u16, vm: &mut VM) -> Result<(), VMError> {
        match self {
            OpCode::BR => {
                let n = (raw_instr >> 11) & 0x1;
                let z = (raw_instr >> 10) & 0x1;
                let p = (raw_instr >> 9) & 0x1;
                let pc_offset = raw_instr & 0x1FF;

                let cond_flag = vm.get_register(Register::COND as u16);

                if (cond_flag & 0x1) == n && (cond_flag & 0x2) == z && (cond_flag & 0x4) == p {
                    let pc = vm.get_register(Register::PC as u16);
                    vm.set_register(Register::PC as u16, pc + pc_offset);
                }

                Ok(())
            }
            OpCode::ADD => {
                let r0 = (raw_instr >> 9) & 0x7;
                let r1 = (raw_instr >> 6) & 0x7;
                let imm_flag = (raw_instr >> 5) & 0x1;

                if imm_flag == 1 {
                    let imm5 = raw_instr & 0x1F;
                    let r1_val = vm.get_register(r1);
                    vm.set_register(r0, r1_val.wrapping_add(imm5)); // Wrapping add to handle overflow
                } else {
                    let r2 = raw_instr & 0x7;
                    let r1_val = vm.get_register(r1);
                    let r2_val = vm.get_register(r2);
                    vm.set_register(r0, r1_val.wrapping_add(r2_val)); // Wrapping add to handle overflow
                }

                vm.set_cond_flags(vm.get_register(r0));

                Ok(())
            }
            OpCode::LD => {
                let r0 = (raw_instr >> 9) & 0x7;
                let pc_offset = raw_instr & 0x1FF;
                let mem_addr = vm.get_register(Register::PC as u16).wrapping_add(pc_offset);
                let mem_val = vm.read_from_memory(mem_addr);
                vm.set_register(r0, mem_val);
                vm.set_cond_flags(vm.get_register(r0));

                Ok(())
            }
            OpCode::ST => {
                let r0 = (raw_instr >> 9) & 0x7;
                let pc_offset = raw_instr & 0x1FF;
                let mem_addr = vm.get_register(Register::PC as u16).wrapping_add(pc_offset);
                let r0_val = vm.get_register(r0);
                vm.write_to_memory(mem_addr, r0_val);

                Ok(())
            }
            OpCode::JSR => {
                let long_pc_offset_flag = (raw_instr >> 11) & 0x1;
                let r1 = (raw_instr >> 6) & 0x7;
                let long_pc_offset = raw_instr & 0x7FF;

                let old_pc = vm.get_register(Register::PC as u16);
                vm.set_register(Register::R7 as u16, old_pc);

                if long_pc_offset_flag == 0 {
                    // JSRR
                    let r1_val = vm.get_register(r1);
                    vm.set_register(Register::PC as u16, r1_val);
                } else {
                    // JSR
                    let new_pc = old_pc.wrapping_add(long_pc_offset);
                    vm.set_register(Register::PC as u16, new_pc);
                }

                Ok(())
            }
            OpCode::AND => {
                let r0 = (raw_instr >> 9) & 0x7;
                let r1 = (raw_instr >> 6) & 0x7;
                let imm_flag = (raw_instr >> 5) & 0x1;

                if imm_flag == 1 {
                    let imm5 = raw_instr & 0x1F;
                    let r1_val = vm.get_register(r1);
                    vm.set_register(r0, r1_val & imm5);
                } else {
                    let r2 = raw_instr & 0x7;
                    let r1_val = vm.get_register(r1);
                    let r2_val = vm.get_register(r2);
                    vm.set_register(r0, r1_val & r2_val);
                }

                vm.set_cond_flags(vm.get_register(r0));

                Ok(())
            }
            OpCode::LDR => {
                let r0 = (raw_instr >> 9) & 0x7;
                let base_r = (raw_instr >> 6) & 0x7;
                let offset = raw_instr & 0x3F;

                let base_addr = vm.get_register(base_r);
                let mem_addr = base_addr.wrapping_add(offset);
                let mem_val = vm.read_from_memory(mem_addr);
                vm.set_register(r0, mem_val);
                vm.set_cond_flags(vm.get_register(r0));

                Ok(())
            }
            OpCode::STR => {
                let r0 = (raw_instr >> 9) & 0x7;
                let base_r = (raw_instr >> 6) & 0x7;
                let offset = raw_instr & 0x3F;

                let base_addr = vm.get_register(base_r);
                let r0_val = vm.get_register(r0);
                let mem_addr = base_addr.wrapping_add(offset);
                vm.write_to_memory(mem_addr, r0_val);

                Ok(())
            }
            OpCode::RTI => Ok(()),
            OpCode::NOT => {
                let r0 = (raw_instr >> 9) & 0x7;
                let r1 = (raw_instr >> 6) & 0x7;

                let r1_val = vm.get_register(r1);
                vm.set_register(r0, !r1_val);
                vm.set_cond_flags(vm.get_register(r0));

                Ok(())
            }
            OpCode::LDI => {
                let r0 = (raw_instr >> 9) & 0x7;
                let pc_offset = raw_instr & 0x1FF;
                let mem_addr = vm.get_register(Register::PC as u16).wrapping_add(pc_offset);
                let indirect_addr = vm.read_from_memory(mem_addr);
                let mem_val = vm.read_from_memory(indirect_addr);
                vm.set_register(r0, mem_val);
                vm.set_cond_flags(vm.get_register(r0));

                Ok(())
            }
            OpCode::STI => {
                let r0 = (raw_instr >> 9) & 0x7;
                let pc_offset = raw_instr & 0x1FF;
                let mem_addr = vm.get_register(Register::PC as u16).wrapping_add(pc_offset);
                let r0_val = vm.get_register(r0);
                let indirect_addr = vm.read_from_memory(mem_addr);
                vm.write_to_memory(indirect_addr, r0_val);

                Ok(())
            }
            OpCode::JMP => {
                let base_r = (raw_instr >> 6) & 0x7;
                let base_addr = vm.get_register(base_r);
                vm.set_register(Register::PC as u16, base_addr);

                Ok(())
            }
            OpCode::RES => Ok(()),
            OpCode::LEA => {
                let r0 = (raw_instr >> 9) & 0x7;
                let pc_offset = raw_instr & 0x1FF;
                let mem_addr = vm.get_register(Register::PC as u16).wrapping_add(pc_offset);
                vm.set_register(r0, mem_addr);
                vm.set_cond_flags(vm.get_register(r0));

                Ok(())
            }
            OpCode::TRAP => {
                let trap = Trap::from_raw_intr(raw_instr);
                trap.execute()
            }
            _ => Err(VMError::OPCodeDoesNotExist),
        }
    }
}
