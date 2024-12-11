// SPDX-License-Identifier: MIT OR MIT-0
//
// Copyright © 2024 René Kijewski <crates.io@k6i.de>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this
// software and associated documentation files (the "Software"), to deal in the Software
// without restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

//! # `system-mimalloc` – use the system's shared mimalloc library as allocator
//!
//! [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/system-mimalloc/ci.yml?branch=main&style=flat-square&logo=github&logoColor=white "GitHub Workflow Status")](https://github.com/Kijewski/system-mimalloc/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/system-mimalloc?logo=rust&style=flat-square "Crates.io")](https://crates.io/crates/system-mimalloc)
//! [![docs.rs](https://img.shields.io/docsrs/system-mimalloc?logo=docsdotrs&style=flat-square&logoColor=white "docs.rs")](https://docs.rs/system-mimalloc/)
//! ![Minimum supported Rust version: 1.38](https://img.shields.io/badge/rustc-1.38+-informational?logo=rust&style=flat-square "Minimum Supported Rust Version: 1.38")
//! [![License: MIT-0](https://img.shields.io/badge/license-MIT--0-informational?logo=apache&style=flat-square)](https://github.com/Kijewski/system-mimalloc/blob/v1.0.0-pre.0/LICENSE.md "License: MIT-0")
//!
//! A drop-in global allocator using the system's shared [mimalloc](https://github.com/microsoft/mimalloc) library.
//! Mimalloc is a general purpose, performance oriented allocator built by Microsoft.
//!
//! Probably only useful on Linux.
//! Use &lt;[crates.io/crates/mimalloc](https://crates.io/crates/mimalloc)&gt; if you want to hard-link
//! `mimalloc` to your program, to have more configuration options, and a higher platform compatibility.
//!
//! ## Usage
//!
//! Simply add this line to your `main.rs`:
//!
//! ```rust,ignore
//! system_mimalloc::use_mimalloc!();
//! ```
//!
//! # Requirements
//!
//! Make sure that `mimalloc` is installed, e.g.:
//!
//! ```sh
//! sudo apt install libmimalloc-dev
//! ```

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

/// A [global allocator](GlobalAlloc) using the system's shared mimalloc library.
///
/// To use the allocator in your program, add either of the two code snippets to your `main.rs`:
///
/// ```rust,ignore
/// system_mimalloc::use_mimalloc!();
/// ```
///
/// ```rust,ignore
/// use system_mimalloc::MiMalloc;
///
/// #[global_allocator]
/// static GLOBAL: MiMalloc = MiMalloc;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiMalloc;

/// Install [`MiMalloc`] as [global allocator](GlobalAlloc).
///
/// To use the allocator in your program, simply add this line to your `main.rs`:
///
/// ```rust,ignore
/// system_mimalloc::use_mimalloc!();
/// ```
#[macro_export]
macro_rules! use_mimalloc {
    ($(,)?) => {
        const _: () = {
            #[global_allocator]
            static GLOBAL: $crate::MiMalloc = $crate::MiMalloc;
        };
    };
}

#[allow(clippy::inline_always)]
unsafe impl GlobalAlloc for MiMalloc {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        mi_malloc_aligned(layout.size(), layout.align()).cast()
    }

    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mi_free(ptr.cast());
    }

    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        mi_zalloc_aligned(layout.size(), layout.align()).cast()
    }

    #[inline(always)]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        mi_realloc_aligned(ptr.cast(), new_size, layout.align()).cast()
    }
}

#[link(name = "mimalloc")]
extern "C" {
    fn mi_malloc_aligned(size: usize, alignment: usize) -> *mut c_void;
    fn mi_zalloc_aligned(size: usize, alignment: usize) -> *mut c_void;
    fn mi_realloc_aligned(p: *mut c_void, newsize: usize, alignment: usize) -> *mut c_void;
    fn mi_free(p: *mut c_void);
}
