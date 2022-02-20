## Fast Floats
This project is forked from [bluss/fast-floats](https://github.com/bluss/fast-floats).

**Changes from original project:**
- Fast floats implement deref to their source float type

## Original docs
[Docs for `Fast` struct ](https://docs.rs/fast-floats/latest/fast_floats/struct.Fast.html)

Experimental (unstable) “fast-math” wrappers for f32, f64

These wrappers enable the [“fast-math”][1] flags for the operations
where there are intrinsics for this (add, sub, mul, div, rem).
The wrappers exist so that we have a quick & easy way **to experiment**
with fast math flags and further that feature in Rust.

Note that as of this writing, the Rust instrinsics use the “fast” flag
documented in the langref; this enables all the float flags.

[1]: https://llvm.org/docs/LangRef.html#fast-math-flags

## Rust Version

This crate is nightly only and experimental. Breaking changes can occur at
any time, if changes in Rust require it.

License: MIT OR Apache-2.0
