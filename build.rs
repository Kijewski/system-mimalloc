//! Tell `cargo` to link `libmimalloc` to your program.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=mimalloc");
}
