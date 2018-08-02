/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Hello World

// The &'static here means the return type has a static lifetime.
// This is a Rust feature that you don't need to worry about now.
pub fn hello() -> &'static str {
    "Hello, World!"
}
