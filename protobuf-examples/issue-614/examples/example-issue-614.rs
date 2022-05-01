use protobuf::MessageFull;
use protobuf_examples_issue_614::foos;
use protobuf_examples_issue_614::generate_insert;
use protobuf_examples_issue_614::generate_schema_for;

fn main() {
    let descriptor = protobuf_examples_issue_614::foos::Foo::descriptor();
    println!("Schema: {}", generate_schema_for(descriptor));

    let mut message = foos::Foo::new();
    message.bar = 1;
    message.baz = Some(foos::foo::Baz::Qux("test".to_owned()));
    println!("Insert: {}", generate_insert(&message));
}
