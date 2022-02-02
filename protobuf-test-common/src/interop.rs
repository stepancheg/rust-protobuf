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
