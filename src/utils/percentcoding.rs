//! Implements URL percent coding

use crate::err;
use crate::error::Error;
use std::borrow::Cow;

/// Percent-decodes the encoded data
pub fn decode<'a, Encoded>(encoded: Encoded) -> Result<Cow<'a, [u8]>, Error>
where
    Encoded: Into<Cow<'a, [u8]>>,
{
    /// Encodes a nibble into a hex char
    fn percent_decode_nibble(nibble: u8) -> Result<u8, Error> {
        // Note: All operations are safe since they are implicitly validated by the range comparisons
        #[allow(clippy::arithmetic_side_effects, reason = "The range is validated by the match")]
        match nibble {
            b'0'..=b'9' => Ok(nibble - b'0'),
            b'a'..=b'f' => Ok((nibble - b'a') + 0xA),
            b'A'..=b'F' => Ok((nibble - b'A') + 0xA),
            nibble => Err(err!("Invalid nibble 0x{nibble:01x}")),
        }
    }

    /// Encodes a byte
    fn percent_decode_byte(high: u8, low: u8) -> Result<u8, Error> {
        Ok((percent_decode_nibble(high)? << 4) | percent_decode_nibble(low)?)
    }

    // Check if we need some decoding
    let encoded = encoded.into();
    let needs_decode = encoded.contains(&b'%');
    if !needs_decode {
        // Fast-path to avoid alloc
        return Ok(encoded);
    }

    // Perform decoding
    let mut source = encoded.iter().copied();
    let mut decoded = Vec::new();
    while let Some(mut byte) = source.next() {
        // Decode percent literal if necessary
        if byte == b'%' {
            // Get the encoded bytes
            let high = source.next().ok_or(err!("Truncated hex literal"))?;
            let low = source.next().ok_or(err!("Truncated hex literal"))?;
            byte = percent_decode_byte(high, low)?;
        }

        // Write byte
        decoded.push(byte);
    }
    Ok(Cow::Owned(decoded))
}
