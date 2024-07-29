use termios::*;

pub fn read_file_as_u16(file_path: &str, buffer: &mut [u16], write_from: usize) {
    let contents = std::fs::read(file_path).unwrap();

    let buff: Vec<u16> = contents
        .chunks(2)
        .map(|a| u16::from_be_bytes([a[0], a[1]]))
        .collect();

    buffer[write_from as usize..write_from as usize + buff.len() - 1].copy_from_slice(&buff[1..]);
}

pub fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }

    x
}

pub fn disable_input_buffering() {
    let mut termios: Termios = Termios::from_fd(0).unwrap();
    termios.c_lflag &= !ICANON & !ECHO;
    termios::tcsetattr(0, TCSANOW, &termios).unwrap();
}

pub fn restore_input_buffering() {
    let mut termios = Termios::from_fd(0).unwrap();
    termios.c_lflag |= ICANON | ECHO;
    termios::tcsetattr(0, TCSANOW, &termios).unwrap();
}
