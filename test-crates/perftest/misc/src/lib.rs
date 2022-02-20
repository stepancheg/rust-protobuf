// Feature name changed
#![cfg_attr(rustc_nightly, feature(bench_black_box))]
#![cfg_attr(rustc_nightly, feature(test))]

#[cfg(not(rustc_nightly))]
#[inline(never)]
pub fn black_box<T>(v: T) -> T {
    v
}

#[cfg(rustc_nightly)]
#[inline(always)]
pub fn black_box<T>(v: T) -> T {
    std::hint::black_box(v)
}
