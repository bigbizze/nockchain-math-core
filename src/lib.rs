#![feature(cold_path)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod belt;
pub mod bpoly;
pub mod crypto;
pub mod felt;
pub mod fpoly;
pub mod poly;
pub mod tip5;

pub mod math_error;

pub use belt::{based_check, Belt, FieldError, PRIME, PRIME_128};
pub use math_error::{MathError, MathResult};
