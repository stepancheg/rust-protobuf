/*!
An example of basic I/O with rust-protobuf. Takes a protocol (basic.proto), compiles it, and then
takes a message round-trip through either a file or a Vec<u8>.

First, we use ./build-examples.sh to generate 'basicproto/mod.rs' and compile this example.
*/

extern crate protobuf;

use protobuf::{Message,MessageStatic};
use std::error::Error;
use std::path::Path;

mod basicproto;
use basicproto::BasicMessage;

/// Put a message in a file
fn to_file<M: Message>(message: M, path: &Path) -> Result<(), Box<Error>> {
    let mut buf = try!(std::fs::File::create(path));
    Ok(try!(message.write_to_writer(&mut buf)))
}

/// Read a message from a file
fn from_file<M: MessageStatic>(path: &Path) -> Result<M, Box<Error>> {
    let mut buf = try!(std::fs::File::open(path));
    Ok(try!(protobuf::parse_from_reader::<M>(&mut buf)))
}

/// Put a message in a buffer (Vec<u8>)
fn to_vec<M: Message>(message: M) -> Result<Vec<u8>, Box<Error>> {
    let mut buf : Vec<u8> = Vec::new();
    try!(message.write_to_writer(&mut buf));
    Ok(buf)
}

/// Read a message in a buffer (Vec<u8>)
fn from_vec<M: MessageStatic>(buf: Vec<u8>) -> Result<M, Box<Error>> {
    let mut read_buf = std::io::Cursor::new(buf);
    Ok(try!(protobuf::parse_from_reader::<M>(&mut read_buf)))
}

fn main() {
    // Let's make a message, so that we can send it or not, as we please
    let mut message = BasicMessage::new();
    message.set_name("Name Field!".to_owned());
    message.set_data(vec![2.,3.,5.,7.,11.,13.,17.,19.]);
    
    println!("We start with the message:");
    println!("    {:?}", message);
    
    let mut args = std::env::args();
    let _ = args.next();
    let (path_str, result) = match args.next() {
        None => {
            // We were not given a path, so we dump the message to a Vec, and then read it
            // back from that Vec
            ("Vec<u8>".to_owned(), to_vec(message)
                .and_then(|vec| {from_vec::<BasicMessage>(vec)}))
            }
        Some(s) => {
            // We got a path on the command-line, so we dump the message to that file, and then
            // read it back from that file
            let s_copy = s.clone();
            let path = Path::new(&s_copy);
            (s, to_file(message, path)
                .and_then(|_| {from_file(path)}))
        }
    };
    
    match result {
        Ok(message) => {
            println!("After a round-trip to {}, we have the message:", path_str);
            println!("    {:?}", message);
        }
        Err(e) => {
            println!("We had an error on a round-trip to {}!", path_str);
            println!("{}", e);
        }
    }
}
