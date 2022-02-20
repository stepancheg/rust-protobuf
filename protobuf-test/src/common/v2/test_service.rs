use super::test_service_pb::*;

#[test]
fn test_service() {
    // The request/response types should still
    // get generated, even though we ignore the
    // service definition in the same file.
    let _ = Request::new();
    let _ = Response::new();
}

#[test]
fn reflect() {
    let services = file_descriptor().services();
    assert_eq!(1, services.len());
}
