use protobuf::reflect::MessageDescriptor;
use protobuf::MessageDyn;
include!(concat!(env!("OUT_DIR"), "/p/mod.rs"));

fn table_name(message: &MessageDescriptor) -> String {
    format!("{}s", message.name().to_lowercase())
}

pub fn generate_schema_for(message: MessageDescriptor) -> String {
    let table = table_name(&message);
    let mut columns = Vec::new();
    for field in message.fields() {
        if field.containing_oneof().is_some() {
            continue;
        }
        columns.push(format!("{} STRING", field.name()));
    }
    for oneof in message.oneofs() {
        columns.push(format!("{}_type STRING", oneof.name()));
        columns.push(format!("{}_value STRING", oneof.name()));
    }
    let columns = columns.join(", ");
    format!("CREATE TABLE {table} ({columns})")
}

pub fn generate_insert(message: &dyn MessageDyn) -> String {
    let descriptor = message.descriptor_dyn();
    let table = table_name(&descriptor);
    let mut column_names = Vec::new();
    let mut column_values = Vec::new();
    for field in descriptor.fields() {
        if let Some(v) = field.get_singular(message) {
            if let Some(oneof) = field.containing_oneof() {
                column_names.push(format!("{}_type", oneof.name()));
                column_values.push(format!("'{}'", field.name()));
                column_names.push(format!("{}_value", oneof.name()));
                column_values.push(format!("'{}'", v));
            } else {
                column_names.push(field.name().to_owned());
                column_values.push(format!("'{}'", v));
            }
        }
    }
    let column_names = column_names.join(", ");
    let column_values = column_values.join(", ");
    format!("INSERT INTO {table}({column_names}) VALUES ({column_values})")
}
