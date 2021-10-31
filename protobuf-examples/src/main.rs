mod protos;

use protobuf::Message;
// For demonstration we take `GetRequest` messages from a source generated
// with pure-rust codegen, and `GetResponse` with `protoc`-based codegen.
// This is not needed in practice, done here for demonstration purposes.
use protos::generated_with_native::example::get_response::Status;
use protos::generated_with_native::example::GetResponse;
use protos::generated_with_pure::example::GetRequest;

fn main() {
    // Encode example request
    let mut out_msg = GetRequest::new();
    out_msg.name = "John Smith".to_string();
    out_msg.age = 25;
    out_msg.features.push("one".to_string());
    out_msg.features.push("two".to_string());

    let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();

    // Decode example request
    let in_msg = GetRequest::parse_from_bytes(&out_bytes).unwrap();

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
    let in_resp = GetResponse::parse_from_bytes(&out_bytes).unwrap();

    assert_eq!(in_resp.status, out_resp.status);
    assert_eq!(in_resp.zipcode, out_resp.zipcode);
    assert_eq!(in_resp.address, out_resp.address);
}
