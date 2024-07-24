pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
}

/**
 * Memory mapped registers
 */
pub enum MMRegister {
    KBSR = 0xFE00, /* keyboard status */
    KBDR = 0xFE02, /* keyboard data */
}
