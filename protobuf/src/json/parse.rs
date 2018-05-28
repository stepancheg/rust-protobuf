use super::base64;

use Message;

pub fn merge_into(_message: &mut Message, _json: &str) {

    // mute warnings for a while
    base64::encode(&b""[..]);
    base64::decode("").unwrap();
}
