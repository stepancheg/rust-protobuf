## How to develop rust-protobuf itself

### ./rebuild.sh

Compile the project.

### ./clean.sh

Clean all stuff that shouldn't live in git.

### ./regenerate.sh

Generate .rs files from .proto files, that needed
internally in rust-protobuf.

### rust test lib/protobuf.rs

Execute the test suite.

### ./full-rebuild.sh

* build project
* regenerate .rs files
* build project again to ensure that generated files are correct
* run tests

This is primary script for development.

### ./checkout-generated.sh

Revert generated files to git version. Use if generated
files are incorrect.
