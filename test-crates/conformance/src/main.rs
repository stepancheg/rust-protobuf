use std::io::{self, Read, Write};

use anyhow::{anyhow, bail, Context, Error, Result};

mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

use protobuf::Message;
use protos::conformance::{
    conformance_request::Payload, ConformanceRequest, ConformanceResponse, WireFormat,
};

fn main() {
    loop {
        match serve_conformance_request() {
            Ok(false) => {}
            Ok(true) => break,
            Err(e) => {
                eprintln!("{:#}", e);
                std::process::exit(1)
            }
        }
    }
}

fn serve_conformance_request() -> Result<bool> {
    let mut stdin = io::stdin().lock();
    let mut len = [0; 4];
    match stdin.read_exact(&mut len) {
        Ok(()) => {}
        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
            return Ok(true);
        }
        Err(e) => return Err(Error::new(e).context("reading request length")),
    }
    let len = u32::from_ne_bytes(len);
    let mut buf = vec![0; len as usize];
    stdin.read_exact(&mut buf).context("reading request")?;

    let request = ConformanceRequest::parse_from_bytes(&buf).context("parsing request")?;

    let response = run_test(&request).context("running test")?;

    let response = response.write_to_bytes().context("serializing response")?;
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(&(response.len() as u32).to_ne_bytes())
        .context("writing response length")?;
    stdout.write_all(&response).context("writing response")?;
    stdout.flush()?;

    Ok(false)
}

fn run_test(request: &ConformanceRequest) -> Result<ConformanceResponse> {
    let files = [
        protos::test_messages_proto3::file_descriptor(),
        protos::test_messages_proto2::file_descriptor(),
        protos::conformance::file_descriptor(),
    ];
    let descriptor = files
        .into_iter()
        .flat_map(|file| file.message_by_full_name(&format!(".{}", request.message_type)))
        .next()
        .ok_or_else(|| anyhow!("couldn't find message type {}", request.message_type))?;

    let Some(ref payload) = request.payload else { bail!("missing or unsupported payload"); };

    let mut test_message = descriptor.new_instance();
    let mut response = ConformanceResponse::new();
    match *payload {
        Payload::ProtobufPayload(ref payload) => {
            if let Err(e) = test_message.merge_from_bytes_dyn(payload) {
                response.set_parse_error(e.to_string());
            }
        }
        Payload::JsonPayload(ref payload) => {
            if let Err(e) = protobuf_json_mapping::merge_from_str(&mut *test_message, payload) {
                response.set_parse_error(e.to_string());
            }
        }
        Payload::JspbPayload(_) => {
            response.set_skipped("JSPB input not supported".into());
        }
        Payload::TextPayload(ref payload) => {
            if let Err(e) = protobuf::text_format::merge_from_str(&mut *test_message, payload) {
                response.set_parse_error(e.to_string());
            }
        }
    }

    let requested_output_format = request
        .requested_output_format
        .enum_value()
        .map_err(|i| anyhow!("unknown wire format {}", i))?;
    match requested_output_format {
        WireFormat::UNSPECIFIED => bail!("unspecified output format"),
        WireFormat::PROTOBUF => response.set_protobuf_payload(
            test_message
                .write_to_bytes_dyn()
                .context("serializing test message")?,
        ),
        WireFormat::JSON => {
            response.set_json_payload(
                protobuf_json_mapping::print_to_string(&*test_message)
                    .context("serializing test message as JSON")?,
            );
        }
        WireFormat::JSPB => response.set_skipped("JSPB output not supported".into()),
        WireFormat::TEXT_FORMAT => {
            response.set_text_payload(protobuf::text_format::print_to_string(&*test_message))
        }
    }

    Ok(response)
}
