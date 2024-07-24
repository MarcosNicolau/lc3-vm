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

    pub fn execute(&self) {
        match self {
            Trap::GETC => {
                println!("Executing GETC trap");
            }
            Trap::OUT => {
                println!("Executing OUT trap");
            }
            Trap::PUTS => {
                println!("Executing PUTS trap");
            }
            Trap::IN => {
                println!("Executing IN trap");
            }
            Trap::PUTSP => {
                println!("Executing PUTSP trap");
            }
            Trap::HALT => {
                println!("Executing HALT trap");
            }
        }
    }
}
