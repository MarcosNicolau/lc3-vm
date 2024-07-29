use std::io::{stdin, stdout, Read, Write};

use crate::{
    register::Register,
    vm::{VMError, VM},
};

pub enum Trap {
    GETC = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    OUT = 0x21,   /* output a character */
    PUTS = 0x22,  /* output a word string */
    IN = 0x23,    /* get character from keyboard, echoed onto the terminal */
    PUTSP = 0x24, /* output a byte string */
    HALT = 0x25,  /* halt the program */
}

impl Trap {
    pub fn from_raw_intr(raw_instr: u16) -> Trap {
        match raw_instr & 0xFF {
            0x20 => Trap::GETC,
            0x21 => Trap::OUT,
            0x22 => Trap::PUTS,
            0x23 => Trap::IN,
            0x24 => Trap::PUTSP,
            0x25 => Trap::HALT,
            _ => panic!("Invalid trap instruction"),
        }
    }

    pub fn execute(&self, vm: &mut VM) -> Result<(), VMError> {
        match self {
            Trap::GETC => {
                let mut buf = [0; 1];
                stdin().read_exact(&mut buf).unwrap();
                vm.set_register(Register::R0 as u16, buf[0] as u16);

                Ok(())
            }
            Trap::OUT => {
                let value = vm.get_register(Register::R0 as u16) as u8;
                print!("{}", value as char);

                Ok(())
            }
            Trap::IN => {
                print!("Enter a character: ");
                stdout().flush().expect("Could not flush stdout");
                let value = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u16)
                    .unwrap();

                vm.set_register(Register::R0 as u16, value as u16);

                Ok(())
            }
            Trap::PUTS => {
                let mut address = vm.get_register(Register::R0 as u16);
                let mut c = vm.read_from_memory(address);

                while c != 0x0 {
                    print!("{}", c as u8 as char);
                    address += 1;
                    c = vm.read_from_memory(address);
                }

                stdout().flush().expect("Could not flush stdout");

                Ok(())
            }
            Trap::PUTSP => {
                let mut address = vm.get_register(Register::R0 as u16);
                let mut c = vm.read_from_memory(address);

                while c != 0x0 {
                    let c1 = ((c & 0xFF) as u8) as char;
                    print!("{}", c1);
                    let c2 = ((c >> 8) as u8) as char;

                    if c2 != '\0' {
                        print!("{}", c2);
                    }

                    address += 1;
                    c = vm.read_from_memory(address);
                }

                stdout().flush().expect("Could not flush stdout");

                Ok(())
            }
            Trap::HALT => {
                vm.exit();

                Ok(())
            }
        }
    }
}
