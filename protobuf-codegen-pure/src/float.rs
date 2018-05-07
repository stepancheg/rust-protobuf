use parser::ParserResult;
use parser::ParserError;

pub const PROTOBUF_NAN: &str = "nan";
pub const PROTOBUF_INF: &str = "inf";

pub fn format_protobuf_float(f: f64) -> String {
    if f.is_nan() {
        PROTOBUF_NAN.to_owned()
    } else if f.is_infinite() {
        if f > 0.0 {
            format!("{}", PROTOBUF_INF)
        } else {
            format!("-{}", PROTOBUF_INF)
        }
    } else {
        // TODO: make sure doesn't lose precision
        format!("{:?}", f)
    }
}

pub fn parse_protobuf_float(s: &str) -> ParserResult<f64> {
    match s.parse() {
        Ok(f) => Ok(f),
        Err(_) => Err(ParserError::IncorrectFloatLit),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_protobuf_float() {
        assert_eq!("10.0", format_protobuf_float(10.0));
    }
}
