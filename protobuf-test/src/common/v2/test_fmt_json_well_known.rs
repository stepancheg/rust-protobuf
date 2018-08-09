use protobuf::well_known_types::Duration;

use protobuf_test_common::*;

use super::test_fmt_json_well_known_pb::*;

#[test]
fn test_duration() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    let mut d = Duration::new();
    d.set_seconds(1);
    d.set_nanos(340012);
    m.set_duration(d);
    test_json_print_parse_message("{duration: \"1.000340012s\"}", &m);

    let mut m = TestFmtJsonWellKnownTypes::new();
    let mut d = Duration::new();
    d.set_seconds(1);
    m.set_duration(d);
    test_json_parse_message("{duration: \"1s\"}", &m);
}
