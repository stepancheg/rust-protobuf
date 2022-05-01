use crate::gen::field::elem::FieldElem;
use crate::gen::field::option_kind::OptionKind;
use crate::gen::file_and_mod::FileAndMod;
use crate::gen::rust_types_values::RustType;
use crate::Customize;

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum SingularFieldFlag {
    // proto2 or proto3 message
    WithFlag {
        required: bool,
        option_kind: OptionKind,
    },
    // proto3
    WithoutFlag,
}

impl SingularFieldFlag {
    pub fn is_required(&self) -> bool {
        match *self {
            SingularFieldFlag::WithFlag { required, .. } => required,
            SingularFieldFlag::WithoutFlag => false,
        }
    }
}

#[derive(Clone)]
pub(crate) struct SingularField<'a> {
    pub flag: SingularFieldFlag,
    pub elem: FieldElem<'a>,
}

impl<'a> SingularField<'a> {
    pub(crate) fn rust_storage_type(&self, reference: &FileAndMod) -> RustType {
        match self.flag {
            SingularFieldFlag::WithFlag { option_kind, .. } => {
                option_kind.wrap_element(self.elem.rust_storage_elem_type(reference))
            }
            SingularFieldFlag::WithoutFlag => self.elem.rust_storage_elem_type(reference),
        }
    }

    pub(crate) fn default_value(
        &self,
        customize: &Customize,
        reference: &FileAndMod,
        const_expr: bool,
    ) -> String {
        self.rust_storage_type(reference)
            .default_value(customize, const_expr)
    }
}
