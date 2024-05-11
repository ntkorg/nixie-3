use heapless::String;


const HEX: &'static [u8; 16] = b"0123456789abcdef";

// one row is 3 * 16 + 1
// 3 chars for hex and space, 1 for newline at start
fn print_string_hex<const N: usize>(str: &mut String<N>, buffer: &[u8]) {
  for i in 0..0x100 {
    if i % 16 == 0 {
      str.push('\n').unwrap();
    }
    let byte = buffer[i];
    str.push(HEX[((byte >> 4) & 0xF) as usize] as char).unwrap();
    str.push(HEX[((byte >> 0) & 0xF) as usize] as char).unwrap();
    str.push(' ').unwrap();
  }
}
