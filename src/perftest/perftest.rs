extern crate protobuf;
extern crate rand;
extern crate time;

use std::io::File;
use std::io::MemWriter;
use std::io::BufReader;
use std::default::Default;
use std::os;

use std::rand::Rng;
use std::rand::StdRng;
use std::rand::SeedableRng;

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

fn run_test<M : Message>(name: &str, data: &[M]) {
    let mut rng: StdRng = SeedableRng::from_seed([10, 20, 30, 40].as_slice());
    let mut random_data: Vec<M> = Vec::new();

    let mut total_size = 0;
    while total_size < 1000000 {
        let ref item = data[rng.gen_range(0, data.len())];
        random_data.push(item.clone());
        total_size += item.serialized_size();
    }

    let mut writer = MemWriter::new();
    measure_and_print(format!("{}: write", name).as_slice(), random_data.len() as u64, || {
        for m in random_data.iter() {
            m.write_length_delimited_to_writer(&mut writer).unwrap();
        }
    });

    let buf = writer.unwrap();

    let read_data = measure_and_print(format!("{}: read", name).as_slice(), random_data.len() as u64, || {
        let mut r = Vec::new();
        let mut reader = BufReader::new(buf.as_slice());
        let mut coded_input_stream = protobuf::CodedInputStream::new(&mut reader);
        while !coded_input_stream.eof().unwrap() {
            r.push(protobuf::parse_length_delimited_from::<M>(&mut coded_input_stream).unwrap());
        }
        r
    });

    assert_eq!(random_data, read_data);

    let merged = measure_and_print(format!("{}: read reuse", name).as_slice(), random_data.len() as u64, || {
        let mut reader = BufReader::new(buf.as_slice());
        let mut coded_input_stream = protobuf::CodedInputStream::new(&mut reader);
        let mut msg: M = Default::default();
        let mut count = 0;
        while !coded_input_stream.eof().unwrap() {
            msg.clear();
            coded_input_stream.merge_message(&mut msg).unwrap();
            count += 1;
        }
        count
    });

    assert_eq!(random_data.len(), merged);
}

struct TestRunner {
    selected: Option<String>,
    any_matched: bool,
}

impl TestRunner {
    fn test<M : Message>(&mut self, name: &str, data: &[M]) {
        if self.selected.is_none() || name == self.selected.as_ref().unwrap().as_slice() {
            run_test(name, data);
            self.any_matched = true;
        }
    }

    fn check(&self) {
        if !self.any_matched {
            let name = self.selected.as_ref().map(|s| s.as_slice()).unwrap_or("bug");
            fail!("no tests found with name {}", name);
        }
    }
}

fn main() {
    let selected = match os::args().as_slice() {
        [_] => None,
        [_, ref test] => Some(test.to_string()),
        v => fail!("usage: {} <test>", v[0]),
    };

    let mut runner = TestRunner { selected: selected, any_matched: false };

    let mut is = File::open(&Path::new("perftest_data.pbbin"));
    let test_data = protobuf::parse_from_reader::<PerftestData>(&mut is).unwrap();

    runner.test("test1", test_data.get_test1());
    runner.test("test_repeated_bool", test_data.get_test_repeated_bool());
    runner.test("test_repeated_packed_int32", test_data.get_test_repeated_packed_int32());
    runner.test("test_repeated_messages", test_data.get_test_repeated_messages());
    runner.test("test_optional_messages", test_data.get_test_optional_messages());
    runner.test("test_strings", test_data.get_test_strings());
    runner.check();
}
