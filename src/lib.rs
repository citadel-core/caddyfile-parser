#![warn(missing_docs)]

//! Caddyfile parser and formatter
//! This crate provides a Caddyfile parser and formatter for Rust.
//! It is a wrapper around the original Caddy, which is written in Go.


use crate::go::{GoSlice, GoString, ParseCaddyfile, FormatCaddyfile};

#[allow(unused)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(missing_docs)]
mod go {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Parse a Caddyfile and return a JSON representation of the parsed Caddyfile.
pub fn parse_caddyfile(filename: &str, contents: &str) -> String {
    let filename = GoString {
        p: filename.as_ptr() as *const std::os::raw::c_char,
        n: filename.len() as isize,
    };
    let contents = GoSlice {
        data: contents.as_ptr() as *mut std::ffi::c_void,
        len: contents.len() as i64,
        cap: contents.len() as i64,
    };
    let mut len: u16 = 0;
    let json = unsafe { ParseCaddyfile(filename, contents, &mut len) };
    let json = unsafe { std::slice::from_raw_parts(json as *const u8, len.into()) };
    String::from_utf8(json.to_vec()).unwrap()
}

/// Formats/prettifies a Caddyfile.
pub fn format_caddyfile(contents: &str) -> String {
    let contents = GoSlice {
        data: contents.as_ptr() as *mut std::ffi::c_void,
        len: contents.len() as i64,
        cap: contents.len() as i64,
    };
    let mut len: u16 = 0;
    let caddyfile: *mut ::std::os::raw::c_char = unsafe { FormatCaddyfile(contents, &mut len) };
    let caddyfile = unsafe { std::slice::from_raw_parts(caddyfile as *const u8, len.into()) };
    String::from_utf8(caddyfile.to_vec()).unwrap()
}
