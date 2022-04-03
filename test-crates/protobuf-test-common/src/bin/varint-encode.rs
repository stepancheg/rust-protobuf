use std::env;

use protobuf::CodedOutputStream;
use protobuf_test_common::hex::encode_hex;

fn parse_u64(s: &str) -> anyhow::Result<u64> {
    if s.starts_with("0x") {
        Ok(u64::from_str_radix(&s[2..], 16)?)
    } else {
        Ok(u64::from_str_radix(s, 10)?)
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    assert_eq!(1, args.len());
    let arg: u64 = parse_u64(&args[0])?;

    let mut varint = Vec::new();
    let mut os = CodedOutputStream::vec(&mut varint);
    os.write_raw_varint64(arg)?;
    os.flush()?;
    drop(os);

    println!("dec:        {}", arg);
    println!("hex:        0x{:x}", arg);
    println!("varint hex: {}", encode_hex(&varint));
    Ok(())
}
