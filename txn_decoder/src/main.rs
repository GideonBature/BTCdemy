use std::io::Read;

fn main() {
    let mut bytes_slice: &[u8] = [1, 0, 0, 0, 2].as_slice();
    let mut buffer = [0_u8; 4];
    bytes_slice.read(&mut buffer).unwrap();

    let version = u32::from_le_bytes(buffer);

    println!("Version: {}", version);
    println!("Bytes slice: {:?}", bytes_slice);

}
