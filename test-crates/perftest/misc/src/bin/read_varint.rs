use std::env;
use std::process;
use std::time::Instant;

use protobuf::CodedInputStream;
use protobuf::CodedOutputStream;
use protobuf_perftest_misc::black_box;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 3 {
        eprintln!(
            "usage: {} <limit> <count> <iter>",
            env::args().next().unwrap()
        );
        process::exit(1);
    }

    let upper: u64 = args[0].parse().unwrap();
    let count = args[1].parse().unwrap();
    let iter = args[2].parse().unwrap();

    let rng = fastrand::Rng::with_seed(17);

    let mut data = Vec::new();
    let mut os = CodedOutputStream::vec(&mut data);
    for _ in 0..count {
        let v = match upper {
            0 => rng.u64(..),
            x => rng.u64(..x),
        };
        os.write_raw_varint64(v).unwrap();
    }
    os.flush().unwrap();
    drop(os);

    let br = iter / 10;

    let start = Instant::now();

    for i in 0..iter {
        if i % br == 0 {
            eprintln!("{}", i / br);
        }
        let mut is = CodedInputStream::from_bytes(black_box(&data));
        while !is.eof().unwrap() {
            black_box(is.read_raw_varint64().unwrap());
        }
    }

    let elapsed = start.elapsed();
    let per_read = elapsed.as_secs_f64() / (iter as f64) / (count as f64);
    eprintln!("{:.3}ns per iter", per_read * 1_000_000_000.0);
}
