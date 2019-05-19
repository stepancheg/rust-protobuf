use crate::rust_name::RustRelativePath;
use Customize;

pub(crate) struct FileAndMod {
    pub file: String,
    pub relative_mod: RustRelativePath,
    pub customize: Customize,
}
