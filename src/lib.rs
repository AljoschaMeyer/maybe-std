#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(not(feature = "std"), feature = "unstable"),
            feature(core_intrinsics, core_panic, raw, unicode_internals))]
#![cfg_attr(all(not(feature = "std"), feature = "alloc", feature = "unstable"),
            feature(raw_vec_internals, wake_trait, asm, concat_idents, format_args_nl, global_asm,
                llvm_asm, log_syntax, trace_macros))]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
pub use std::*;

#[cfg(all(feature = "alloc", not(feature = "std")))]
mod alloc_stuff {
    pub use super::alloc::alloc;
    pub use super::alloc::borrow;
    pub use super::alloc::boxed;
    pub use super::alloc::collections;
    pub use super::alloc::fmt;
    pub use super::alloc::rc;
    pub use super::alloc::slice;
    pub use super::alloc::str;
    pub use super::alloc::string;
    pub use super::alloc::sync;
    pub use super::alloc::vec;

    pub use core::any;
    pub use core::arch;
    pub use core::array;
    pub use core::ascii;
    pub use core::cell;
    pub use core::char;
    pub use core::clone;
    pub use core::cmp;
    pub use core::convert;
    pub use core::default;
    pub use core::f32;
    pub use core::f64;
    pub use core::ffi;
    pub use core::future;
    pub use core::hash;
    pub use core::hint;
    pub use core::i8;
    pub use core::i16;
    pub use core::i32;
    pub use core::i64;
    pub use core::i128;
    pub use core::isize;
    pub use core::iter;
    pub use core::marker;
    pub use core::mem;
    pub use core::num;
    pub use core::ops;
    pub use core::option;
    pub use core::panic;
    pub use core::pin;
    pub use core::primitive;
    pub use core::ptr;
    pub use core::result;
    pub use core::time;
    pub use core::u8;
    pub use core::u16;
    pub use core::u32;
    pub use core::u64;
    pub use core::u128;
    pub use core::usize;

    pub mod prelude {
        pub mod v1 {
            pub use super::super::super::alloc::borrow::ToOwned;
            pub use super::super::super::alloc::boxed::Box;
            pub use super::super::super::alloc::string::String;
            pub use super::super::super::alloc::string::ToString;
            pub use super::super::super::alloc::vec::Vec;

            pub use super::super::super::alloc::format;
            pub use super::super::super::alloc::vec;

            pub use core::prelude::v1::*;
        }
    }

    pub mod task {
        pub use core::task::*;

        #[cfg(feature = "unstable")]
        pub use super::super::alloc::task::*;
    }

    pub use core::assert;
    pub use core::assert_eq;
    pub use core::assert_ne;
    pub use core::cfg;
    pub use core::column;
    pub use core::compile_error;
    pub use core::concat;
    pub use core::debug_assert;
    pub use core::debug_assert_eq;
    pub use core::debug_assert_ne;
    pub use core::env;
    pub use core::file;
    pub use core::format_args;
    pub use core::include;
    pub use core::include_bytes;
    pub use core::include_str;
    pub use core::line;
    pub use core::matches;
    pub use core::module_path;
    pub use core::option_env;
    pub use core::stringify;
    pub use core::todo;
    #[allow(deprecated)]
    pub use core::r#try;
    pub use core::unimplemented;
    pub use core::unreachable;
    pub use core::write;
    pub use core::writeln;

    #[cfg(feature = "unstable")]
    pub use core::asm;
    #[cfg(feature = "unstable")]
    pub use core::concat_idents;
    #[cfg(feature = "unstable")]
    pub use core::format_args_nl;
    #[cfg(feature = "unstable")]
    pub use core::global_asm;
    #[cfg(feature = "unstable")]
    pub use core::llvm_asm;
    #[cfg(feature = "unstable")]
    pub use core::log_syntax;
    #[cfg(feature = "unstable")]
    pub use core::trace_macros;
}
#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc_stuff::*;

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
pub use core::*;
