//! tb67s128ftg Driver
//!
//! Platform-agnostic driver library for the tb67s128ftg stepper motor driver.
//! This crate is a specialized facade for the [Stepper] library. Please
//! consider using Stepper directly, as it provides drivers for more stepper
//! motor drivers, as well as an interface to abstract over them.
//!
//! See [Stepper] for more documentation and usage examples.
//!
//! [Stepper]: https://crates.io/crates/stepper

#![no_std]
#![deny(missing_docs)]

pub use stepper::{drivers::tb67s128ftg::*, *};
