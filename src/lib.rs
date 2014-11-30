
//! Quoted-printable serialization as specified in RFC 2045 Section 6.7.

#![experimental]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#[cfg(test)]
extern crate test;

extern crate serialize;

pub use serialize::{Decoder, Encoder, Decodable, Encodable, DecoderHelpers, EncoderHelpers};

static CHARS: &'static[u8] = b"0123456789ABCDEF";

/// A trait for converting a value to quoted-printable encoding.
pub trait ToQP for Sized? {
    /// Converts the value of `self` to a quoted-printable value following the specified format
    /// configuration, returning the owned string.
    fn to_qp(&self, line_length: Option<uint>) -> String;
}

struct QPTracking {
    buf: Vec<u8>,
    white: Vec<u8>,
    cur_length: uint
}

impl ToQP for [u8] {
    fn ensure_line_wrap(line_length: &Option<uint>, extra: uint, tracking: &mut QPTracking) {
        let mut QPTracking { buf: buf, white: white, cur_length: cur_length } = tracking;
        match line_length {
            Some(line_length) =>
                if cur_length + tracking.white.len() + extra >= line_length {
                    buf.push_all(white.as_slice());
                    buf.push(61);
                    buf.push(13);
                    buf.push(10);
                    white.clear();
                    cur_length = 0;
                },
            None => ()
        }
    }

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
        let mut tracking = QPTracking { buf: Vec::new(), white: Vec::new(), cur_length: 0 };
        let mut i = 0;
        let len = self.len();

        let ensure_line_wrap = |extra, v: &mut Vec<u8>, w: &mut Vec<u8>, l: &mut uint| {
        };

        while i < len {
            let b1 = self[i];
            let b2 = if i + 1 < len {
                self[i + 1]
            }
            else {
                0
            };

            match (b1, b2) {
                (13, 10) => {
                    if !w.is_empty() {
                        ensure_line_wrap(0, &mut v, &mut w, &mut l);
                    }
                    v.push(13);
                    v.push(10);
                    l = 0;
                    i += 2;
                }
                _ => {
                    match b1 {
                        10 => {
                            if !w.is_empty() {
                                ensure_line_wrap(0, &mut v, &mut w, &mut l);
                            }
                            v.push(13);
                            v.push(10);
                            l = 0;
                        },
                        9 | 32 => {
                            w.push(b1);
                            ensure_line_wrap(0, &mut v, &mut w, &mut l);
                        }
                        33 ... 60 | 62 ... 126 => {
                            ensure_line_wrap(1, &mut v, &mut w, &mut l);
                            v.push_all(w.as_slice());
                            w.clear();
                            v.push(b1);
                            l += 1;
                        }
                        _ => {
                            ensure_line_wrap(3, &mut v, &mut w, &mut l);
                            v.push_all(w.as_slice());
                            w.clear();
                            v.push(61);
                            v.push(CHARS[(b1 as uint >> 4)]);
                            v.push(CHARS[(b1 as uint & 0xf)]);
                            l += 3;
                        }
                    }
                    i += 1;
                }
            }

        }
        unsafe {
            String::from_utf8_unchecked(v)
        }
    }
}
