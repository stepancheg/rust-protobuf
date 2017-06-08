// v2 tests which must be compatible with v3 tests
mod v2;

// v3 tests are generated from v2 tests by copy&replace
#[cfg(proto3)]
mod v3;
