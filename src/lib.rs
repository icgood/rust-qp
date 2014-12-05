
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
pub trait ToQP for Sized? {
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
                push_encoded(line_length, trk, vec![EQ]);
            },
        None => ()
    }
}

fn push_encoded(line_length: &Option<uint>, trk: &mut Tracking, encoded: Vec<u8>) {
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
                push_encoded(line_length, trk, vec![EQ]);
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
        let iter2 = self.iter().skip(2).fuse();
        let iter1 = self.iter().skip(1).fuse();
        let mut line = Vec::with_capacity(line_length.unwrap_or(76));
        let mut v = Vec::with_capacity(self.len());

        let encode_char = |c: u8| {
            &[EQ,
              HEX_CHARS[(c as uint >> 4)],
              HEX_CHARS[(c as uint & 0xf)]];
        };

            let next_one = iter1.next();
            let next_two = (next_one, iter2.next());
    }

        unsafe {
            String::from_utf8_unchecked(v)
        }
    }
    //fn to_qp(&self, line_length: Option<uint>) -> String {
    //    let mut tracking = Tracking { buf: Vec::new(), white: Vec::new(), width: 0 };
    //    let mut i = 0;
    //    let len = self.len();

    //    while i < len {
    //        let b1 = self[i];
    //        let b2 = if i + 1 < len {
    //            self[i + 1]
    //        }
    //        else {
    //            0
    //        };

    //        match (b1, b2) {
    //            (CR, LF) => {
    //                push_encoded(&line_length, &mut tracking, vec![LF]);
    //                i += 2;
    //            }
    //            _ => {
    //                match b1 {
    //                    LF => {
    //                        push_encoded(&line_length, &mut tracking, vec![LF]);
    //                    },
    //                    9 | 32 ... 60 | 62 ... 126 => {
    //                        push_encoded(&line_length, &mut tracking, vec![b1]);
    //                    },
    //                    _ => {
    //                        let encoded = vec![61,
    //                                           HEX_CHARS[(b1 as uint >> 4)],
    //                                           HEX_CHARS[(b1 as uint & 0xf)]];
    //                        push_encoded(&line_length, &mut tracking, encoded);
    //                    }
    //                }
    //                i += 1;
    //            }
    //        }

    //    }
    //    unsafe {
    //        String::from_utf8_unchecked(tracking.buf)
    //    }
    //}
}
