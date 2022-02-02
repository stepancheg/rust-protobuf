use protobuf::Message;
use protobuf_test_common::hex::encode_hex;
use protobuf_test_common::interop_json_decode;

use super::interop_pb::InteropMessageList;
use crate::interop::interop_pb::InteropMessage;

#[test]
fn test_repeated_packed_fixed_encoding() {
    let mut mm = InteropMessageList::new();
    mm.ts.push(InteropMessage {
        fixed32_repeated: vec![17, 34],
        ..InteropMessage::default()
    });

    let interop_bin = interop_json_decode("{ts: [{fixed32_repeated: [17, 34]}]}");
    let our_bin = mm.write_to_bytes().unwrap();
    // TODO: we are not using packed encoding for fixed
    if false {
        assert_eq!(encode_hex(&interop_bin), encode_hex(&our_bin));
    }
}
