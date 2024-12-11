# `system-mimalloc` – use the system's shared mimalloc library as allocator

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/system-mimalloc/ci.yml?branch=main&style=flat-square&logo=github&logoColor=white "GitHub Workflow Status")](https://github.com/Kijewski/system-mimalloc/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/system-mimalloc?logo=rust&style=flat-square "Crates.io")](https://crates.io/crates/system-mimalloc)
[![docs.rs](https://img.shields.io/docsrs/system-mimalloc?logo=docsdotrs&style=flat-square&logoColor=white "docs.rs")](https://docs.rs/system-mimalloc/)
![Minimum supported Rust version: 1.38](https://img.shields.io/badge/rustc-1.38+-informational?logo=rust&style=flat-square "Minimum Supported Rust Version: 1.38")
[![License: MIT-0](https://img.shields.io/badge/license-MIT--0-informational?logo=apache&style=flat-square)](https://github.com/Kijewski/system-mimalloc/blob/v1.0.0-pre.0/LICENSE.md "License: MIT-0")

A drop-in global allocator using the system's shared [mimalloc](https://github.com/microsoft/mimalloc) library.
Mimalloc is a general purpose, performance oriented allocator built by Microsoft.

Probably only useful on Linux.
Use &lt;[crates.io/crates/mimalloc](https://crates.io/crates/mimalloc)&gt; if you want to hard-link
`mimalloc` to your program, to have more configuration options, and a higher platform compatibility.

## Usage

Simply add this line to your `main.rs`:

```rust
system_mimalloc::use_mimalloc!();
```

# Requirements

Make sure that `mimalloc` is installed, e.g.:

```sh
sudo apt install libmimalloc-dev
```
