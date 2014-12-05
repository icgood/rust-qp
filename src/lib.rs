
//! Quoted-printable serialization as specified in RFC 2045 Section 6.7.

#![experimental]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#[cfg(test)]
extern crate test;

extern crate serialize;

pub use serialize::{Decoder, Encoder, Decodable, Encodable, DecoderHelpers, EncoderHelpers};

static HEX_CHARS: &'static[u8] = b"0123456789ABCDEF";
const CR: u8 = 13;
const LF: u8 = 10;
const EQ: u8 = 61;
const SPACE: u8 = 32;
const TAB: u8 = 9;

/// A trait for converting a value to quoted-printable encoding.
pub trait ToQP {
    /// Converts the value of `self` to a quoted-printable value following the specified format
    /// configuration, returning the owned string.
    fn to_qp(&self, line_length: Option<uint>) -> String;
}

struct Tracking {
    buf: Vec<u8>,
    white: Vec<u8>,
    width: uint
}

fn check_width(line_length: &Option<uint>, trk: &mut Tracking, change: uint) {
    match *line_length {
        Some(len) =>
            if trk.width + change >= len {
                push_encoded(line_length, trk, &[EQ]);
            },
        None => ()
    }
}

fn push_encoded(line_length: &Option<uint>, trk: &mut Tracking, encoded: &[u8]) {
    if encoded[0] == EQ && encoded.len() == 1 {
        // Want to push a soft newline.
        trk.buf.push_all(trk.white.as_slice());
        trk.buf.push_all(encoded.as_slice());
        trk.buf.push_all(&[CR, LF]);
        trk.white.clear();
        trk.width = 0;
        return;
    }

    match encoded[0] {
        LF => {
            // Want to push a hard newline.
            if trk.white.len() > 0 {
                push_encoded(line_length, trk, &[EQ]);
            }
            trk.buf.push_all(&[CR, LF]);
            trk.width = 0;
        },
        SPACE | TAB => {
            // Want to push a space or tab, which have special rules.
            let change = trk.white.len() + encoded.len();
            check_width(line_length, trk, change);
            trk.white.push_all(encoded.as_slice());
        },
        _ => {
            // Want to push a safe or encoded character.
            let change = trk.white.len() + encoded.len();
            check_width(line_length, trk, change);
            let new_change = trk.white.len() + encoded.len();
            trk.buf.push_all(trk.white.as_slice());
            trk.buf.push_all(encoded.as_slice());
            trk.white.clear();
            trk.width += new_change;
        }
    }
}

fn encode_byte(line_length: &Option<uint>, trk: &mut Tracking, b1: u8, b2: u8) {
    match (b1, b2) {
        (CR, LF) => {
            push_encoded(&line_length, &mut trk, &[LF]);
            iter.next();
        }
        _ => {
            match b1 {
                LF => {
                    push_encoded(&line_length, &mut trk, &[LF]);
                },
                9 | 32 ... 60 | 62 ... 126 => {
                    push_encoded(&line_length, &mut trk, &[b1]);
                },
                _ => {
                    let encoded = &[61,
                                    HEX_CHARS[(b1 as uint >> 4)],
                                    HEX_CHARS[(b1 as uint & 0xf)]];
                    push_encoded(&line_length, &mut trk, encoded);
                }
            }
        }
    }
}

impl ToQP for [u8] {
    /// Turn a vector of `u8` bytes into a quoted-printable string.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate qp;
    /// use qp::ToQP;
    ///
    /// fn main() {
    /// XXX
    /// }
    /// ```
    fn to_qp(&self, line_length: Option<uint>) -> String {
        let mut tracking = Tracking { buf: Vec::with_capacity(self.len()*3),
                                      white: Vec::with_capacity(76),
                                      width: 0 };
        let mut iter = self.iter().peekable();

        loop {
            let b1 = match iter.next() {
                Some(c) => *c,
                None => break,
            };
            let b2 = match iter.peek() {
                Some(c) => **c,
                None => 0,
            };
            encode_byte(&line_length, &mut tracking, b1, b2);
        }
        unsafe {
            String::from_utf8_unchecked(tracking.buf)
        }
    }
}
