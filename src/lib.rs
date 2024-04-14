//! #jpeglib
//! `jpeglib` is the interface of the C library https://github.com/kornelski/libjpeg/, created with bindgen. It is nessesary for libraw-rs-sys crate to convert RAW images to jpeg

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod bindings;