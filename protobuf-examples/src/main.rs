mod protos;
use protobuf::{parse_from_bytes, Message};
use protos::example::{GetRequest, GetResponse, GetResponse_Status};

fn main() {
    // Encode example request
    let mut out_msg = GetRequest::new();
    out_msg.set_name("John Smith".to_string());
    out_msg.set_age(25);
    out_msg.features.push("one".to_string());
    out_msg.features.push("two".to_string());

    let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();

    // Decode example request
    let in_msg = parse_from_bytes::<GetRequest>(&out_bytes).unwrap();

    let in_name = in_msg.get_name();

    assert_eq!(in_name, "John Smith");

    //////////////////////////////////

    // Encode example response
    let mut out_resp = GetResponse::new();
    out_resp.status = GetResponse_Status::OK;
    out_resp.set_address("1243 main street".to_string());
    out_resp.set_city("anytown".to_string());
    out_resp.set_zipcode(54321);

    let out_bytes: Vec<u8> = out_resp.write_to_bytes().unwrap();

    // Decode example response
    let in_resp = parse_from_bytes::<GetResponse>(&out_bytes).unwrap();

    assert_eq!(in_resp.status, out_resp.status);
    assert_eq!(in_resp.zipcode, out_resp.zipcode);
    assert_eq!(in_resp.get_address(), out_resp.get_address());
}
