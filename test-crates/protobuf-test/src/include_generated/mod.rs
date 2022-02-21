// Include single mod.rs which references two mods: `v2` and `v3`
include!(concat!(env!("OUT_DIR"), "/include_generated/mod.rs"));

use v2::V2Message;
use v3::V3Message;

#[test]
fn test() {
    let _ = V2Message::new();
    let _ = V3Message::new();
}
