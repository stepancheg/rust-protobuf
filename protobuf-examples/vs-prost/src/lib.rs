//! # Compare generated code and API between rust-protobuf and prost

mod rust_protobuf_protos {
    include!(concat!(env!("OUT_DIR"), "/rust_protobuf_protos/mod.rs"));
}

mod prost_protos {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

#[cfg(test)]
mod test {
    use crate::prost_protos;
    use crate::rust_protobuf_protos;

    #[test]
    fn triangles() {
        let mut rp = rust_protobuf_protos::triangle::Triangle::new();
        let mut pr = prost_protos::Triangle::default();
        rp.color = "red".to_owned();
        pr.color = "red".to_owned();
    }
}
