extern crate protobuf;
extern crate rand;

use std::fs::File;
use std::path::Path;

use rand::Rng;
use rand::SeedableRng;
use rand::StdRng;

use perftest_data::PerftestData;
use protobuf::Message;
use std::time::Instant;

mod perftest_data;

fn measure_ns<R, F: FnMut() -> R>(mut f: F) -> (u64, R) {
    let start = Instant::now();
    let r = f();
    (start.elapsed().as_nanos() as u64, r)
}

fn measure_and_print<R, F: FnMut() -> R>(title: &str, iter: u64, f: F) -> R {
    let (ns, r) = measure_ns(f);
    let ns_per_iter = ns / iter;
    println!("{}: {} ns per iter", title, ns_per_iter);
    r
}

struct TestRunner {
    data_size: u32,
    selected: Option<String>,
    any_matched: bool,
}

impl TestRunner {
    fn run_test<M: Message + Clone + PartialEq>(&self, name: &str, data: &[M]) {
        assert!(data.len() > 0, "empty string for test: {}", name);

        let mut rng: StdRng = SeedableRng::from_seed([
            10, 20, 30, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ]);

        let mut random_data: Vec<M> = Vec::new();

        let mut total_size = 0;
        while total_size < self.data_size {
            let ref item = data[rng.gen_range(0, data.len())];
            random_data.push(item.clone());
            total_size += item.compute_size();
        }

        let mut buf = Vec::new();
        measure_and_print(
            &format!("{}: write", name),
            random_data.len() as u64,
            || {
                for m in &random_data {
                    m.write_length_delimited_to_vec(&mut buf).unwrap();
                }
            },
        );

        let read_data =
            measure_and_print(&format!("{}: read", name), random_data.len() as u64, || {
                let mut r = Vec::new();
                let mut coded_input_stream = protobuf::CodedInputStream::from_bytes(&buf);
                while !coded_input_stream.eof().unwrap() {
                    r.push(coded_input_stream.read_message().unwrap());
                }
                r
            });

        assert_eq!(random_data, read_data);

        let merged = measure_and_print(
            &format!("{}: read reuse", name),
            random_data.len() as u64,
            || {
                let mut coded_input_stream = protobuf::CodedInputStream::from_bytes(&buf);
                let mut msg: M = Message::new();
                let mut count = 0;
                while !coded_input_stream.eof().unwrap() {
                    msg.clear();
                    coded_input_stream.merge_message(&mut msg).unwrap();
                    count += 1;
                }
                count
            },
        );

        assert_eq!(random_data.len(), merged);
    }

    fn test<M: Message + Clone + PartialEq>(&mut self, name: &str, data: &[M]) {
        if self.selected.as_ref().map(|s| *s == name).unwrap_or(true) {
            self.run_test(name, data);
            self.any_matched = true;
        }
    }

    fn check(&self) {
        if !self.any_matched {
            let name = self.selected.as_ref().map(|s| &s[..]).unwrap_or("bug");
            panic!("no tests found with name {}", name);
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 3 {
        panic!("usage: {} [data_size] [test]", args[0])
    }
    let data_size = args
        .iter()
        .nth(1)
        .map(|x| x.parse().unwrap())
        .unwrap_or(1000000);
    let selected = args.iter().nth(2).cloned();

    let mut runner = TestRunner {
        selected: selected,
        any_matched: false,
        data_size: data_size,
    };

    let mut is = File::open(&Path::new("perftest_data.pbbin")).unwrap();
    let test_data = protobuf::parse_from_reader::<PerftestData>(&mut is).unwrap();

    runner.test("test1", &test_data.test1);
    runner.test("test_repeated_bool", &test_data.test_repeated_bool);
    runner.test(
        "test_repeated_packed_int32",
        &test_data.test_repeated_packed_int32,
    );
    runner.test("test_repeated_messages", &test_data.test_repeated_messages);
    runner.test("test_optional_messages", &test_data.test_optional_messages);
    runner.test("test_strings", &test_data.test_strings);
    runner.test("test_small_bytearrays", &test_data.test_small_bytearrays);
    runner.test("test_large_bytearrays", &test_data.test_large_bytearrays);
    runner.check();
}
