#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate protobuf_fuzz;

fuzz_target!(|data: &[u8]| {
    protobuf_fuzz::fuzz_target_singular(data)
});
