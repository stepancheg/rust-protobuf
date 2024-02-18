use protobuf::{EnumOrUnknown, Message};

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use example::{get_response, GetRequest, GetResponse};

fn main() {
    // Encode example request
    let mut out_msg = GetRequest::new();
    out_msg.name = "John Smith".to_string();
    out_msg.age = 25;
    out_msg.features.push("one".to_string());
    out_msg.features.push("two".to_string());
    println!("Message request:\nout_msg {:#?}", out_msg);

    let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();
    println!("Message request in bytes:\nout_bytes {:?}", out_bytes);

    // Decode example request
    let in_msg = GetRequest::parse_from_bytes(&out_bytes).unwrap();

    assert_eq!(in_msg.name, out_msg.name);
    assert_eq!(in_msg.age, out_msg.age);
    assert_eq!(in_msg.features, out_msg.features);

    //////////////////////////////////

    // Encode example response
    let mut out_resp = GetResponse::new();
    out_resp.status = EnumOrUnknown::new(get_response::Status::OK);
    out_resp.address = "1243 main street".to_string();
    out_resp.city = "anytown".to_string();
    out_resp.zipcode = 54321;
    println!("\nMessage response:\nout_msg {:#?}", out_resp);

    let out_bytes: Vec<u8> = out_resp.write_to_bytes().unwrap();
    println!("Message response in bytes:\nout_bytes {:?}", out_bytes);

    // Decode example response
    let in_resp = GetResponse::parse_from_bytes(&out_bytes).unwrap();

    assert_eq!(in_resp.status, out_resp.status);
    assert_eq!(in_resp.address, out_resp.address);
    assert_eq!(in_resp.city, out_resp.city);
    assert_eq!(in_resp.zipcode, out_resp.zipcode);
}