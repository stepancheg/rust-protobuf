mod protos;

use protobuf::parse_from_bytes;
use protobuf::Message;
use protos::example::get_response::Status;
use protos::example::GetRequest;
use protos::example::GetResponse;

fn main() {
    // Encode example request
    let mut out_msg = GetRequest::new();
    out_msg.name = "John Smith".to_string();
    out_msg.age = 25;
    out_msg.features.push("one".to_string());
    out_msg.features.push("two".to_string());

    let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();

    // Decode example request
    let in_msg = parse_from_bytes::<GetRequest>(&out_bytes).unwrap();

    let in_name = in_msg.name;

    assert_eq!(in_name, "John Smith");

    //////////////////////////////////

    // Encode example response
    let mut out_resp = GetResponse::new();
    out_resp.status = Status::OK.into();
    out_resp.address = "1243 main street".to_string();
    out_resp.city = "anytown".to_string();
    out_resp.zipcode = 54321;

    let out_bytes: Vec<u8> = out_resp.write_to_bytes().unwrap();

    // Decode example response
    let in_resp = parse_from_bytes::<GetResponse>(&out_bytes).unwrap();

    assert_eq!(in_resp.status, out_resp.status);
    assert_eq!(in_resp.zipcode, out_resp.zipcode);
    assert_eq!(in_resp.address, out_resp.address);
}
