#![no_main]
extern crate libfuzzer_sys;
extern crate regex_syntax;

use std::str;
use regex_syntax::Expr;

#[export_name="rust_fuzzer_test_input"]
pub extern fn go(data: &[u8]) {
    if let Ok(s) = str::from_utf8(data) {
        Expr::parse(s);
    }
}
