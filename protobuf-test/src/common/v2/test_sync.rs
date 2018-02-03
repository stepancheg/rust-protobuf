use std::thread;
use std::sync::Arc;

use protobuf::CodedInputStream;
use protobuf::Message;

use super::test_sync_pb::*;

// test messages are sync
#[test]
fn test_sync() {
    let m = Arc::new({
        let mut r = TestSync::new();
        r.set_int32_field(23);
        r
    });

    let threads: Vec<_> = (0..4)
        .map(|_| {
            let m_copy = m.clone();
            thread::spawn(move || {
                let bytes = m_copy.write_to_bytes().unwrap();
                let mut is = CodedInputStream::from_bytes(&bytes);
                let mut read = TestSync::new();
                // API is not very convenient here
                read.merge_from(&mut is).unwrap();
                read.check_initialized().unwrap();
                read.get_int32_field()
            })
        })
        .collect();

    let results = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(&[23, 23, 23, 23], &results[..]);
}
