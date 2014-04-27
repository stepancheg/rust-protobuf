extern crate protobuf;
extern crate rand;
extern crate time;

use std::io::File;
use std::io::MemWriter;
use std::io::BufReader;
use std::default::Default;

use rand::Rng;
use rand::StdRng;
use rand::SeedableRng;

use protobuf::Message;
use perftest_data::PerftestData;

mod perftest_data;

fn measure_ns<R>(f: || -> R) -> (u64, R) {
    let start = time::precise_time_ns();
    let r = f();
    (time::precise_time_ns() - start, r)
}

fn measure_and_print<R>(title: &str, iter: u64, f: || -> R) -> R {
    let (ns, r) = measure_ns(f);
    let ns_per_iter = ns / iter;
    println!("{}: {} ns per iter", title, ns_per_iter);
    r
}

fn test<M : Message>(name: &str, data: &[M]) {
    let mut rng: StdRng = SeedableRng::from_seed(&[10, 20, 30, 40]);
    let mut random_data: Vec<M> = Vec::new();

    let mut total_size = 0;
    while total_size < 1000000 {
        let ref item = data[rng.gen_range(0, data.len())];
        random_data.push(item.clone());
        total_size += item.serialized_size();
    }

    let mut writer = MemWriter::new();
    measure_and_print(format!("{}: write", name), random_data.len() as u64, || {
        for m in random_data.iter() {
            m.write_length_delimited_to_writer(&mut writer);
        }
    });

    let buf = writer.unwrap();

    let read_data = measure_and_print(format!("{}: read", name), random_data.len() as u64, || {
        let mut r = Vec::new();
        let mut reader = BufReader::new(buf.as_slice());
        let mut coded_input_stream = protobuf::CodedInputStream::new(&mut reader);
        while !coded_input_stream.eof() {
            r.push(protobuf::parse_length_delimited_from::<M>(&mut coded_input_stream));
        }
        r
    });

    assert_eq!(random_data, read_data);

    let merged = measure_and_print(format!("{}: read reuse", name), random_data.len() as u64, || {
        let mut reader = BufReader::new(buf.as_slice());
        let mut coded_input_stream = protobuf::CodedInputStream::new(&mut reader);
        let mut msg: M = Default::default();
        let mut count = 0;
        while !coded_input_stream.eof() {
            msg.clear();
            coded_input_stream.merge_message(&mut msg);
            count += 1;
        }
        count
    });

    assert_eq!(random_data.len(), merged);
}

fn main() {
    let mut is = File::open(&Path::new("perftest_data.pbbin"));
    let perftest_data = protobuf::parse_from_reader::<PerftestData>(&mut is);
    test("test1", perftest_data.get_test1());
    test("test_repeated_bool", perftest_data.get_test_repeated_bool());
    test("test_repeated_messages", perftest_data.get_test_repeated_messages());
    test("test_optional_messages", perftest_data.get_test_optional_messages());
    test("test_strings", perftest_data.get_test_strings());
}
