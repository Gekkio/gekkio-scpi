// SPDX-FileCopyrightText: 2020-2021 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::io::{self, Write};

use crate::{Block, DefaultValue, Discrete, Limit, ScpiDisplay, Step};

/// Trait for types that can be used as SCPI command/query parameters
pub trait Parameter {
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()>;
}

impl Parameter for Discrete {
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        // TODO: return error instead
        debug_assert!(self
            .0
            .chars()
            .all(|ch| { ch.is_ascii() && !(ch.is_ascii_control() && !ch.is_ascii_whitespace()) }));
        w.write_all(self.0.as_bytes())
    }
}

#[test]
fn test_discrete_parameter() {
    let mut buf = Vec::new();
    Discrete("TEST").encode(&mut buf).unwrap();
    assert_eq!(buf, b"TEST");
}

impl Parameter for &str {
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        w.write_all(b"\"")?;
        for ch in self.chars() {
            match ch {
                // Double quotes are escaped by duplicating them
                '"' => w.write_all(b"\"\"")?,
                // Only ASCII is allowed
                ch if ch.is_ascii() && !(ch.is_ascii_control() && !ch.is_ascii_whitespace()) => {
                    w.write_all(&[ch as u8])?
                }
                // TODO: return error instead
                _ => w.write_all(b"*")?,
            }
        }
        w.write_all(b"\"")
    }
}

#[test]
fn test_str_parameter() {
    let mut buf = Vec::new();
    "foo".encode(&mut buf).unwrap();
    assert_eq!(buf, b"\"foo\"");
}

#[test]
fn test_str_parameter_escape() {
    let mut buf = Vec::new();
    r#"what if "quotes" break 'stuff'?"#.encode(&mut buf).unwrap();
    assert_eq!(buf, br#""what if ""quotes"" break 'stuff'?""#);
}

impl<'a> Parameter for Block<'a> {
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        w.write_all(b"#")?;
        let mut buf = [0; 64];
        let remaining = {
            let mut buf_slice = &mut buf[..];
            write!(buf_slice, "{}", self.0.len())?;
            buf_slice.len()
        };
        let digits = buf.len() - remaining;
        w.write_all(&[b'0' + (digits as u8)])?;
        w.write_all(&buf[..digits])?;
        w.write_all(self.0)
    }
}

#[test]
fn test_block_parameter() {
    let mut buf = Vec::new();
    Block(&[0x11, 0x22, 0x33]).encode(&mut buf).unwrap();
    assert_eq!(buf, b"#13\x11\x22\x33");
}

impl Parameter for f32 {
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        // SCPI 1999.0: 7.2 - Decimal Numeric Program Data
        // TODO: return error instead
        debug_assert!(!(self > 9.9E37 || self < -9.9E37));
        if self.is_finite() {
            write!(w, "{:E}", self)
        } else if self.is_nan() {
            // SCPI 1999.0: 7.2.1.5 - Not A Number (NAN)
            w.write_all(b"NAN")
        } else {
            // SCPI 1999.0: 7.2.1.4 - INFinity and Negative INFinity (NINF)
            if self.is_sign_positive() {
                w.write_all(b"INF")
            } else {
                w.write_all(b"NINF")
            }
        }
    }
}

#[test]
fn test_f32_parameter_positive() {
    let mut buf = Vec::new();
    1.234567E11.encode(&mut buf).unwrap();
    assert_eq!(buf, b"1.234567E11");
}

#[test]
fn test_f32_parameter_negative() {
    let mut buf = Vec::new();
    (-1.234567E-11).encode(&mut buf).unwrap();
    assert_eq!(buf, b"-1.234567E-11");
}

impl Parameter for DefaultValue {
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        w.write_all(b"DEF")
    }
}

impl Parameter for Limit {
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        w.write_all(match self {
            Limit::Min => b"MIN",
            Limit::Max => b"MAX",
        })
    }
}

impl Parameter for Step {
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        w.write_all(match self {
            Step::Up => b"UP",
            Step::Down => b"DOWN",
        })
    }
}

impl Parameter for bool {
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        // SCPI 1999.0: 7.3 - Boolean Program Data
        w.write_all(match self {
            true => b"1",
            false => b"0",
        })
    }
}

impl<T: ScpiDisplay> Parameter for T {
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        write!(w, "{}", self)
    }
}

impl Parameter for () {
    fn encode<W>(self, _w: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl<A, B> Parameter for (A, B)
where
    A: Parameter,
    B: Parameter,
{
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        self.0.encode(w)?;
        w.write_all(b",")?;
        self.1.encode(w)
    }
}

#[test]
fn test_tuple2() {
    let mut buf = Vec::new();
    ("mixed", Discrete("BAG")).encode(&mut buf).unwrap();
    assert_eq!(buf, br#""mixed",BAG"#);
}

impl<A, B, C> Parameter for (A, B, C)
where
    A: Parameter,
    B: Parameter,
    C: Parameter,
{
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        self.0.encode(w)?;
        w.write_all(b",")?;
        self.1.encode(w)?;
        w.write_all(b",")?;
        self.2.encode(w)
    }
}

#[test]
fn test_tuple3() {
    let mut buf = Vec::new();
    (1u8, -1i8, -420000f32).encode(&mut buf).unwrap();
    assert_eq!(buf, b"1,-1,-4.2E5");
}

impl<A, B, C, D> Parameter for (A, B, C, D)
where
    A: Parameter,
    B: Parameter,
    C: Parameter,
    D: Parameter,
{
    fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        self.0.encode(w)?;
        w.write_all(b",")?;
        self.1.encode(w)?;
        w.write_all(b",")?;
        self.2.encode(w)?;
        w.write_all(b",")?;
        self.3.encode(w)
    }
}
