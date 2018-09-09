use std::fmt::Write as fmt_Write;
use std::fmt;

use Message;
use reflect::ReflectFieldRef;
use reflect::ReflectValueRef;
use reflect::ReflectRepeatedRef;
use json::float;
use std::f32;
use std::f64;
use reflect::ReflectMapRef;
use json::base64;

use well_known_types::NullValue;
use well_known_types::Value;
use well_known_types::Value_oneof_kind;
use well_known_types::DoubleValue;
use well_known_types::FloatValue;
use well_known_types::Int64Value;
use well_known_types::UInt64Value;
use well_known_types::Int32Value;
use well_known_types::UInt32Value;
use well_known_types::BoolValue;
use well_known_types::StringValue;
use well_known_types::BytesValue;
use well_known_types::ListValue;
use well_known_types::Struct;
use well_known_types::Duration;
use well_known_types::Timestamp;
use well_known_types::FieldMask;
use well_known_types::Any;

use json::well_known_wrapper::WellKnownWrapper;

use reflect::EnumValueDescriptor;
use json::rfc_3339::TmUtc;


#[derive(Debug)]
pub enum PrintError {
    Fmt(fmt::Error),
    AnyPrintingIsNotImplemented,
    TimestampNegativeNanos,
}

impl From<fmt::Error> for PrintError {
    fn from(e: fmt::Error) -> Self {
        PrintError::Fmt(e)
    }
}

pub type PrintResult<T> = Result<T, PrintError>;


struct Printer {
    buf: String,
    print_options: PrintOptions,
}

trait PrintableToJson {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()>;
}

trait JsonFloat : fmt::Display + fmt::Debug + PrintableToJson {
    fn is_nan(&self) -> bool;
    fn is_pos_infinity(&self) -> bool;
    fn is_neg_infinity(&self) -> bool;

    fn print_to_json_impl(&self, w: &mut String) -> PrintResult<()> {
        Ok(if self.is_nan() {
            write!(w, "\"{}\"", float::PROTOBUF_JSON_NAN)?
        } else if self.is_pos_infinity() {
            write!(w, "\"{}\"", float::PROTOBUF_JSON_INF)?
        } else if self.is_neg_infinity() {
            write!(w, "\"{}\"", float::PROTOBUF_JSON_MINUS_INF)?
        } else {
            write!(w, "{:?}", self)?
        })
    }
}

impl JsonFloat for f32 {
    fn is_nan(&self) -> bool {
        f32::is_nan(*self)
    }

    fn is_pos_infinity(&self) -> bool {
        f32::is_infinite(*self) && self > &0.0
    }

    fn is_neg_infinity(&self) -> bool {
        f32::is_infinite(*self) && self < &0.0
    }
}

impl PrintableToJson for f32 {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        Ok(self.print_to_json_impl(&mut w.buf)?)
    }
}

impl JsonFloat for f64 {
    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }

    fn is_pos_infinity(&self) -> bool {
        f64::is_infinite(*self) && self > &0.0
    }

    fn is_neg_infinity(&self) -> bool {
        f64::is_infinite(*self) && self < &0.0
    }
}

impl PrintableToJson for f64 {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        self.print_to_json_impl(&mut w.buf)
    }
}

impl PrintableToJson for u64 {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        Ok(write!(w.buf, "\"{}\"", self)?)
    }
}

impl PrintableToJson for i64 {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        Ok(write!(w.buf, "\"{}\"", self)?)
    }
}

impl PrintableToJson for u32 {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        Ok(write!(w.buf, "{}", self)?)
    }
}

impl PrintableToJson for i32 {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        Ok(write!(w.buf, "{}", self)?)
    }
}

impl PrintableToJson for bool {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        Ok(write!(w.buf, "{}", self)?)
    }
}

impl PrintableToJson for str {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        write!(w.buf, "\"")?;
        for c in self.chars() {
            match c {
                '"' => write!(w.buf, "\\\""),
                '\\' => write!(w.buf, "\\\\"),
                '\n' => write!(w.buf, "\\n"),
                '\r' => write!(w.buf, "\\r"),
                '\t' => write!(w.buf, "\\t"),
                c if c.is_control() => write!(w.buf, "\\u{:04x}", c as u32),
                c => write!(w.buf, "{}", c),
            }?;
        }
        write!(w.buf, "\"")?;
        Ok(())
    }
}

impl PrintableToJson for String {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        self.as_str().print_to_json(w)
    }
}

impl PrintableToJson for [u8] {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        let encoded = base64::encode(self);
        encoded.print_to_json(w)
    }
}

impl PrintableToJson for Vec<u8> {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        self.as_slice().print_to_json(w)
    }
}

impl<'a> PrintableToJson for ReflectValueRef<'a> {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        match self {
            ReflectValueRef::U32(v) => w.print_printable(v),
            ReflectValueRef::U64(v) => w.print_printable(v),
            ReflectValueRef::I32(v) => w.print_printable(v),
            ReflectValueRef::I64(v) => w.print_printable(v),
            ReflectValueRef::F32(v) => w.print_printable(v),
            ReflectValueRef::F64(v) => w.print_printable(v),
            ReflectValueRef::Bool(v) => w.print_printable(v),
            ReflectValueRef::String(v) => w.print_printable::<str>(v),
            ReflectValueRef::Bytes(v) => w.print_printable::<[u8]>(v),
            ReflectValueRef::Enum(v) => w.print_enum(v),
            ReflectValueRef::Message(v) => w.print_message(*v),
        }
    }
}

impl PrintableToJson for Duration {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        let sign = if self.seconds >= 0 { "" } else { "-" };
        Ok(write!(w.buf, "\"{}{}.{:09}s\"", sign, self.seconds.abs(), self.nanos.abs())?)
    }
}

impl PrintableToJson for Timestamp {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        if self.nanos < 0 {
            return Err(PrintError::TimestampNegativeNanos);
        }
        let tm_utc = TmUtc::from_protobuf_timestamp(self.seconds, self.nanos as u32);
        w.print_printable(&tm_utc.to_string())
    }
}

impl PrintableToJson for FieldMask {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        w.print_printable(&self.paths.join(","))
    }
}

impl PrintableToJson for Any {
    fn print_to_json(&self, _w: &mut Printer) -> PrintResult<()> {
        Err(PrintError::AnyPrintingIsNotImplemented)
    }
}

impl PrintableToJson for Value {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        match self.kind {
            // None should not be possible here, but it's better to print null than crash
            None => w.print_json_null(),
            Some(Value_oneof_kind::null_value(null_value)) => w.print_wk_null_value(&null_value),
            Some(Value_oneof_kind::bool_value(b)) => w.print_printable(&b),
            Some(Value_oneof_kind::number_value(n)) => w.print_printable(&n),
            Some(Value_oneof_kind::string_value(ref s)) => w.print_printable::<String>(&s),
            Some(Value_oneof_kind::struct_value(ref s)) => w.print_printable(&s),
            Some(Value_oneof_kind::list_value(ref l)) => w.print_printable(&l),
        }
    }
}

impl PrintableToJson for ListValue {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        w.print_list(&self.values)
    }
}

impl PrintableToJson for Struct {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        w.print_object(&self.fields)
    }
}

impl<'a, P : PrintableToJson> PrintableToJson for &'a P {
    fn print_to_json(&self, w: &mut Printer) -> PrintResult<()> {
        (*self).print_to_json(w)
    }
}


trait ObjectKey {
    fn print(&self, w: &mut Printer) -> PrintResult<()>;
}

impl<'a> ObjectKey for ReflectValueRef<'a> {
    fn print(&self, w: &mut Printer) -> PrintResult<()> {
        match self {
            ReflectValueRef::U32(v) => w.print_printable(v),
            ReflectValueRef::U64(v) => w.print_printable(v),
            ReflectValueRef::I32(v) => w.print_printable(v),
            ReflectValueRef::I64(v) => w.print_printable(v),
            ReflectValueRef::Bool(v) => w.print_printable(v),
            ReflectValueRef::String(v) => w.print_printable::<str>(v),
            ReflectValueRef::Bytes(v) => w.print_printable::<[u8]>(v),
            ReflectValueRef::Enum(v) => w.print_enum(v),
            ReflectValueRef::F32(_) |
            ReflectValueRef::F64(_) |
            ReflectValueRef::Message(_) => panic!("cannot be object key"),
        }
    }
}

impl ObjectKey for String {
    fn print(&self, w: &mut Printer) -> PrintResult<()> {
        w.print_printable(self)
    }
}

impl<'a, O : ObjectKey> ObjectKey for &'a O {
    fn print(&self, w: &mut Printer) -> PrintResult<()> {
        (*self).print(w)
    }
}


impl Printer {
    fn print_comma_but_first(&mut self, first: &mut bool) -> fmt::Result {
        if *first {
            *first = false;
            Ok(())
        } else {
            write!(self.buf, ", ")
        }
    }

    fn print_json_null(&mut self) -> PrintResult<()> {
        Ok(write!(self.buf, "null")?)
    }

    fn print_printable<F : PrintableToJson + ?Sized>(&mut self, f: &F) -> PrintResult<()> {
        f.print_to_json(self)
    }

    fn print_list<I>(&mut self, items: I) -> PrintResult<()>
        where
            I: IntoIterator,
            I::Item: PrintableToJson,
    {
        write!(self.buf, "[")?;
        for (i, item) in items.into_iter().enumerate() {
            if i != 0 {
                write!(self.buf, ", ")?;
            }
            self.print_printable(&item)?;
        }
        write!(self.buf, "]")?;
        Ok(())
    }

    fn print_repeated(&mut self, repeated: &ReflectRepeatedRef) -> PrintResult<()> {
        self.print_list(repeated)
    }

    fn print_object<I, K, V>(&mut self, items: I) -> PrintResult<()>
        where
            I: IntoIterator<Item=(K, V)>,
            K: ObjectKey,
            V: PrintableToJson,
    {
        write!(self.buf, "{{")?;
        for (i, (k, v)) in items.into_iter().enumerate() {
            if i != 0 {
                write!(self.buf, ", ")?;
            }
            k.print(self)?;
            write!(self.buf, ": ")?;
            self.print_printable(&v)?;
        }
        write!(self.buf, "}}")?;
        Ok(())
    }

    fn print_map(&mut self, map: &ReflectMapRef) -> PrintResult<()> {
        self.print_object(map.into_iter())
    }

    fn print_enum(&mut self, value: &EnumValueDescriptor) -> PrintResult<()> {
        if let Some(null_value) = value.cast() {
            self.print_wk_null_value(&null_value)
        } else {
            if self.print_options.enum_values_int {
                Ok(write!(self.buf, "{}", value.value())?)
            } else {
                Ok(write!(self.buf, "\"{}\"", value.name())?)
            }
        }
    }

    fn print_message(&mut self, message: &Message) -> PrintResult<()> {
        let descriptor = message.descriptor();

        if let Some(duration) = message.as_any().downcast_ref::<Duration>() {
            return self.print_printable(duration);
        } else if let Some(timestamp) = message.as_any().downcast_ref::<Timestamp>() {
            return self.print_printable(timestamp);
        } else if let Some(field_mask) = message.as_any().downcast_ref::<FieldMask>() {
            return self.print_printable(field_mask);
        } else if let Some(any) = message.as_any().downcast_ref::<Any>() {
            return self.print_printable(any);
        } else if let Some(value) = message.as_any().downcast_ref::<Value>() {
            return self.print_printable(value);
        } else if let Some(value) = message.as_any().downcast_ref::<DoubleValue>() {
            return self.print_wrapper(value);
        } else if let Some(value) = message.as_any().downcast_ref::<FloatValue>() {
            return self.print_wrapper(value);
        } else if let Some(value) = message.as_any().downcast_ref::<Int64Value>() {
            return self.print_wrapper(value);
        } else if let Some(value) = message.as_any().downcast_ref::<UInt64Value>() {
            return self.print_wrapper(value);
        } else if let Some(value) = message.as_any().downcast_ref::<Int32Value>() {
            return self.print_wrapper(value);
        } else if let Some(value) = message.as_any().downcast_ref::<UInt32Value>() {
            return self.print_wrapper(value);
        } else if let Some(value) = message.as_any().downcast_ref::<BoolValue>() {
            return self.print_wrapper(value);
        } else if let Some(value) = message.as_any().downcast_ref::<StringValue>() {
            return self.print_wrapper(value);
        } else if let Some(value) = message.as_any().downcast_ref::<BytesValue>() {
            return self.print_wrapper(value);
        } else if let Some(value) = message.as_any().downcast_ref::<ListValue>() {
            return self.print_printable(value);
        } else if let Some(value) = message.as_any().downcast_ref::<Struct>() {
            return self.print_printable(value);
        }

        write!(self.buf, "{{")?;

        let mut first = true;

        for field in descriptor.fields() {
            let json_field_name = if self.print_options.proto_field_name {
                field.name()
            } else {
                field.json_name()
            };
            match field.get_reflect(message) {
                ReflectFieldRef::Optional(None) => {}
                ReflectFieldRef::Optional(Some(v)) => {
                    self.print_comma_but_first(&mut first)?;
                    write!(self.buf, "{}: ", json_field_name)?;
                    self.print_printable(&v)?;
                }
                ReflectFieldRef::Repeated(v) => {
                    if !v.is_empty() {
                        write!(self.buf, "{}: ", json_field_name)?;
                        self.print_repeated(&v)?;
                    }
                }
                ReflectFieldRef::Map(v) => {
                    if !v.is_empty() {
                        write!(self.buf, "{}: ", json_field_name)?;
                        self.print_map(&v)?;
                    }
                }
            }
        }

        write!(self.buf, "}}")?;
        Ok(())
    }

    fn print_wk_null_value(&mut self, _null_value: &NullValue) -> PrintResult<()> {
        self.print_json_null()
    }

    fn print_wrapper<W>(&mut self, value: &W) -> PrintResult<()>
        where
            W : WellKnownWrapper,
            W::Underlying : PrintableToJson,
    {
        self.print_printable(value.get_ref())
    }
}

pub fn print_to_string(message: &Message) -> PrintResult<String> {
    print_to_string_with_options(message, &PrintOptions::default())
}

/// Options for printing JSON to string
#[derive(Default, Debug, Clone)]
pub struct PrintOptions {
    /// Use ints instead of strings for enums.
    /// Note both string or int can be parsed.
    pub enum_values_int: bool,
    /// Use protobuf field names instead of `lowerCamelCase` which is used by default.
    /// Note both names are supported when JSON is parsed.
    pub proto_field_name: bool,
}

pub fn print_to_string_with_options(message: &Message, print_options: &PrintOptions)
    -> PrintResult<String>
{
    let mut printer = Printer {
        buf: String::new(),
        print_options: print_options.clone(),
    };
    printer.print_message(message)?;
    Ok(printer.buf)
}
