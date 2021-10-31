use protobuf::descriptor::FieldDescriptorProto_Type;
use protobuf::Message;

use super::test_reflect_issue_564_pb::*;

fn visit_fields(mes: &dyn Message) {
    let mut seen_enum = false;
    let mut seen_message = false;

    let fields = mes.descriptor().fields();
    for field in fields {
        if field.has_field(mes) {
            match field.proto().get_field_type() {
                FieldDescriptorProto_Type::TYPE_ENUM => {
                    assert_eq!("TEST_ENUM_VALUE_B", field.get_enum(mes).name());
                    assert!(!seen_enum);
                    seen_enum = true;
                }
                FieldDescriptorProto_Type::TYPE_MESSAGE => {
                    let _ = field.get_message(mes);
                    assert!(!seen_message);
                    seen_message = true;
                }
                _ => {}
            }
        }
    }
    assert!(seen_enum);
    assert!(seen_message);
}

#[test]
fn test() {
    let mut mes = Issue564::default();
    mes.set_ee(Issue564_TestEnum::TEST_ENUM_VALUE_B);
    mes.set_mm(Default::default());
    visit_fields(&mes);
}
