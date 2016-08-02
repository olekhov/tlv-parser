#![crate_name = "tlv_parser"]
#![feature(question_mark)]
#![feature(test)]
#![feature(plugin)]
#![feature(exact_size_is_empty)]

#![plugin(clippy)]

//! A library to parse and emit [BER-TLV](https://en.wikipedia.org/wiki/X.690#BER_encoding) data.
//!
//! #Examples
//!
//! Parse TLV:
//!
//! ```
//! use tlv_parser::tlv::{Tlv, Value};
//!
//! let input: Vec<u8> = vec![0x21, 0x05, 0x22, 0x03, 0x03, 0x01, 0xaa];
//! let tlv = Tlv::from_vec( &input ).unwrap();
//!
//! if let Some(&Value::Val(ref val)) = tlv.find_val("21 / 22 / 03") {
//!     assert_eq!(*val, vec![0xaa]);
//! }
//! ```
//!
//! Emit constructed TLV incapsulated primitive TLV:
//!
//! ```
//! use tlv_parser::tlv::*;
//! 
//! let primitive_tlv = Tlv::new(0x01, Value::Nothing);
//! let constructed_tlv = Tlv::new(0x21, Value::TlvList(vec![primitive_tlv]));
//! 
//! assert_eq!(constructed_tlv.to_vec(), vec![0x21, 0x02, 0x01, 0x00]);
//! ```

#[macro_use]
extern crate error_chain;

extern crate byteorder;
extern crate rustc_serialize;
extern crate test;

pub mod tlv;
pub mod errors;

