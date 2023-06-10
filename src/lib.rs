#![no_std]
#![allow(clippy::missing_safety_doc)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod prelude;

pub mod masking;
pub mod sparse_masking;
pub mod subtracting;

pub mod capacity;
pub mod storage;
