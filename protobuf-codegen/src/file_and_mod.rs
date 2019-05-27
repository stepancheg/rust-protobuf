use crate::customize::Customize;
use crate::rust_name::RustRelativePath;

pub(crate) struct FileAndMod {
    pub file: String,
    pub relative_mod: RustRelativePath,
    pub customize: Customize,
}
