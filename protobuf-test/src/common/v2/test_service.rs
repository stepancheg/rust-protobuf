use super::test_service_pb::*;

#[test]
fn test_service() {
    // The request/response types should still
    // get generated, even though we ignore the
    // service definition in the same file.
    let _ = Request::new();
    let _ = Response::new();
}
