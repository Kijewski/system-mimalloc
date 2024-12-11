//! A simple program that uses `mimalloc` as its allocator.

system_mimalloc::use_mimalloc!();

fn main() {
    println!("{}", "Hello, world!".to_owned());
}
