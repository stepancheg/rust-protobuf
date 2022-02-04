use crate::gen::code_writer::CodeWriter;
use crate::Customize;

pub(crate) fn write_custom_type_attr(w: &mut CodeWriter, customize: &Customize) {
    if let Some(attrs) = &customize.type_attrs {
        for attr in attrs {
            w.write_line(&attr.format());
        }
    }
}

pub(crate) fn write_custom_field_attr(w: &mut CodeWriter, customize: &Customize) {
    if let Some(attrs) = &customize.field_attrs {
        for attr in attrs {
            w.write_line(&attr.format());
        }
    }
}

pub(crate) fn write_special_field_attr(w: &mut CodeWriter, customize: &Customize) {
    if let Some(attrs) = &customize.special_field_attrs {
        for attr in attrs {
            w.write_line(&attr.format());
        }
    }
}
