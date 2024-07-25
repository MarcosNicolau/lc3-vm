use std::fs::File;
use std::io::prelude::*;

pub fn read_file_as_u16(
    file_path: &str,
    bytes_to_read: usize,
    buffer: &mut [u16],
    write_from: usize,
) {
    let mut file = File::open(file_path).expect("file does not exist!");
    let mut buff: Vec<u8> = vec![0; bytes_to_read];
    let bytes_read = file.read(&mut buff).expect("could not read file!");
    let mut idx = 0;

    while idx < bytes_read {
        /*
         * Imagine buff contains the bytes [0xAB, 0xCD, 0xEF, 0x12]. Here’s how the indexing and conversion work:
         * For idx = 0:
         * buff[0 * 2] is 0xAB (first byte of the first pair),
         * buff[0 * 2 + 1] is 0xCD (second byte of the first pair).
         *
         * Combined into u16: buffer[0] = (0xAB << 8) | 0xCD results in buffer[0] = 0xABCD.
         * For idx = 1:
         *         buff[1 * 2] is 0xEF (first byte of the second pair),
         * buff[1 * 2 + 1] is 0x12 (second byte of the second pair).
         *
         * Combined into u16: buffer[1] = (0xEF << 8) | 0x12 results in buffer[1] = 0xEF12.
         */
        let byte1 = buff[idx * 2] as u16;
        let byte2 = buff[idx * 2 + 1] as u16;
        buffer[write_from + idx] = (byte1 << 8) | byte2;
        idx += 1;
    }
}
