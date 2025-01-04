use hex;

#[allow(unused)]
fn read_version(transaction_hex: &str) -> /* Result<(), FromHexError> */ u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes = transaction_bytes[0..4].to_vec();
    print!("version bytes: {:?}", version_bytes);
    // Ok(())
    1
}

fn main() {
    let transaction_hex = "0000000003aeaefd44fde3f16b5d0a0dda1a9e8a1291df4505f12ed3d62d6e40";
    let version = read_version(&transaction_hex);

    print!("version: {}", version);
}
