// SPDX-FileCopyrightText: 2020-2021 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use crate::param::Parameter;
use std::fmt;

mod param;

/// Discrete SCPI parameter
///
/// Reference: IEEE 488.2
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Discrete(pub &'static str);

/// An arbitrary block of bytes
///
/// Reference: IEEE 488.2
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Block<'a>(pub &'a [u8]);

/// Special parameter that allows the instrument to select a numeric value.
///
/// Reference: SCPI 1999.0: 7.2.1.1 - DEFault
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DefaultValue;

/// Special parameter that refers to a numeric limit value.
///
/// Reference: SCPI 1999.0: 7.2.1.2 - MINimum|MAXimum
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Limit {
    Min,
    Max,
}

/// Special parameter that refers to a numeric step.
///
/// Reference: SCPI 1999.0: 7.2.1.3 - UP|DOWN
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Step {
    Up,
    Down,
}

/// Trait for types that can be used as SCPI parameters using their `fmt::Display` representation
pub trait ScpiDisplay: fmt::Display {}

impl ScpiDisplay for i8 {}
impl ScpiDisplay for i16 {}
impl ScpiDisplay for i32 {}
impl ScpiDisplay for i64 {}
impl ScpiDisplay for isize {}
impl ScpiDisplay for u8 {}
impl ScpiDisplay for u16 {}
impl ScpiDisplay for u32 {}
impl ScpiDisplay for u64 {}
impl ScpiDisplay for usize {}
