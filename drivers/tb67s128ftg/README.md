# Tb67s128ftg Driver [![crates.io](https://img.shields.io/crates/v/tb67s128ftg.svg)](https://crates.io/crates/tb67s128ftg) [![Documentation](https://docs.rs/tb67s128ftg/badge.svg)](https://docs.rs/tb67s128ftg) ![CI Build](workflows/CI%20Build/badge.svg)

## About

Rust driver crate for the [Tb67s128ftg] stepper motor driver. Carrier boards for this chip are [available from Pololu].

This crate is a specialized facade for the [Stepper] library. Please consider using Stepper directly, as it provides drivers for more stepper motor drivers, as well as an interface to abstract over them.

See [Stepper] for more documentation and usage examples.

## License

This project is open source software, licensed under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with the software, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[Tb67s128ftg]: https://toshiba.semicon-storage.com/us/semiconductor/product/motor-driver-ics/stepping-motor-driver-ics/detail.TB67S128FTG.html
[available from pololu]: https://www.pololu.com/product/2998
[Stepper]: https://crates.io/crates/stepper
[zero clause bsd license]: https://opensource.org/licenses/0BSD
[license.md]: LICENSE.md
