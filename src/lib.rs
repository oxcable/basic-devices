//! Basic audio filters for making music with Rust.
//!
//! This is an extension to the
//! [`oxcable` framework](https://github.com/oxcable/oxcable). These devices are
//! designed to be used with the basic tools provided in the framework. See the
//! core crate for further details.

extern crate num;
extern crate oxcable;

pub mod adsr;
pub mod delay;
pub mod dynamics;
pub mod reverb;
pub mod tremolo;
