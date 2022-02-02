use std::io::Read;
use std::io::Write;
use std::process;

/// Invoke `interop` binary, pass given data as stdin, return stdout.
pub fn interop_command(command: &str, stdin: &[u8]) -> Vec<u8> {
    let mut interop = process::Command::new("../interop/cxx/interop")
        .args(&[command])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn()
        .expect("interop");

    interop
        .stdin
        .take()
        .unwrap()
        .write_all(stdin)
        .expect("write to process");

    let mut stdout = Vec::new();
    interop
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut stdout)
        .expect("read json");

    let exit_status = interop.wait().expect("wait_with_output");
    assert!(exit_status.success(), "{}", exit_status);

    stdout
}

/// Decode binary protobuf, encode as JSON.
pub fn interop_json_encode(bytes: &[u8]) -> String {
    let json = interop_command("json-encode", bytes);
    String::from_utf8(json).expect("UTF-8")
}

/// Decode JSON, encode as binary protobuf.
pub fn interop_json_decode(s: &str) -> Vec<u8> {
    interop_command("json-decode", s.as_bytes())
}
