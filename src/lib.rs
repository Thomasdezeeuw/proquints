//! proquints: PRO-nouncable QUINT-uplets of alternating unambiguous consonants
//! and vowels.
//!
//! See <https://arxiv.org/html/0901.4016> for an introduction and the
//! specification.

use std::convert::AsRef;
use std::mem::size_of;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str;

static CONSONANTS: [u8; 16] = [
    b'b', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b'm', b'n', b'p', b'r', b's', b't', b'v', b'z',
];

static VOWELS: [u8; 4] = [b'a', b'i', b'o', b'u'];

/// Create the proquint for `input`.
pub fn proquints<T: Proquint>(input: T) -> String {
    let input = input.as_bytes();
    let input = input.as_ref();
    let mut buf = vec![0u8; output_length(input.len())];
    proquints_buf(input, &mut buf, b'-');
    // SAFETY: we only use valid UTF-8 characters.
    unsafe { String::from_utf8_unchecked(buf) }
}

/// Same as [`proquints`] but allows the user to provide the `buf`fer and
/// `separator`.
///
/// # Panics
///
/// This will panic if `input`'s length is not even or if the `buf`fer's length
/// is not larger than [`output_length`]`(buf)`.
pub fn proquints_buf<'a>(input: &[u8], buf: &'a mut [u8], separator: u8) -> &'a str {
    assert!(input.len() % 2 == 0);
    assert!(output_length(input.len()) >= buf.len());
    let mut i = 0;
    let mut c = 0;
    while c < input.len() {
        let b = ((u16::from(input[c])) << 8) | u16::from(input[c + 1]);
        c += 2;
        buf[i] = CONSONANTS[usize::from((b & 0b1111_0000_0000_0000) >> 12)];
        i += 1;
        buf[i] = VOWELS[usize::from((b & 0b0000_1100_0000_0000) >> 10)];
        i += 1;
        buf[i] = CONSONANTS[usize::from((b & 0b0000_0011_1100_0000) >> 6)];
        i += 1;
        buf[i] = VOWELS[usize::from((b & 0b0000_0000_0011_0000) >> 4)];
        i += 1;
        buf[i] = CONSONANTS[usize::from(b & 0b0000_0000_0000_1111)];
        i += 1;
        if i != buf.len() {
            buf[i] = separator;
            i += 1;
        }
    }
    // SAFETY: we only use valid UTF-8 characters above.
    unsafe { str::from_utf8_unchecked(&buf[..i]) }
}

/// Returns the output length for `input_length`.
///
/// The returned length for `input_length`s that are not even is invalid.
pub const fn output_length(input_length: usize) -> usize {
    // Per 16 bits (2 bytes) we output 5 characters + separator, minus the
    // separator at the end.
    ((input_length / 2) * 6) - 1
}

/// Trait to define what types can be used in [`proquints`].
///
/// Note that it's not required to implement this trait. You can also convert
/// your type to `&[u8]` and call `proquints` using that, but ensure that the
/// length of the slice is even!
pub trait Proquint {
    /// Some type that can be referenced as a slice of bytes.
    type Bytes: AsRef<[u8]>;

    /// Return itself as bytes.
    fn as_bytes(&self) -> Self::Bytes;
}

/// # Panics
///
/// The implementation of [`proquints`] will panic if the length of the slice is
/// not even.
impl<'a> Proquint for &'a [u8] {
    type Bytes = &'a [u8];

    fn as_bytes(&self) -> Self::Bytes {
        *self
    }
}

impl Proquint for u16 {
    type Bytes = [u8; 2];

    fn as_bytes(&self) -> Self::Bytes {
        self.to_be_bytes()
    }
}

impl Proquint for u32 {
    type Bytes = [u8; 4];

    fn as_bytes(&self) -> Self::Bytes {
        self.to_be_bytes()
    }
}

impl Proquint for u64 {
    type Bytes = [u8; 8];

    fn as_bytes(&self) -> Self::Bytes {
        self.to_be_bytes()
    }
}

impl Proquint for usize {
    type Bytes = [u8; size_of::<usize>()];

    fn as_bytes(&self) -> Self::Bytes {
        self.to_be_bytes()
    }
}

impl Proquint for Ipv4Addr {
    type Bytes = [u8; 4];

    fn as_bytes(&self) -> Self::Bytes {
        self.octets()
    }
}

impl Proquint for Ipv6Addr {
    type Bytes = [u8; 16];

    fn as_bytes(&self) -> Self::Bytes {
        self.octets()
    }
}
