//! Conversions between `U256` and `u16` (and arrays of 16-bit words).

use super::U256;

/// Splits a `U256` into 16 big-endian `u16` words.
impl From<U256> for [u16; 16] {
    fn from(value: U256) -> Self {
        let mut out = [0u16; 16];

        for (i, chunk) in value.0.chunks_exact(2).enumerate() {
            out[i] = u16::from_be_bytes([chunk[0], chunk[1]]);
        }

        out
    }
}

/// Builds a `U256` from 16 big-endian `u16` words.
impl From<[u16; 16]> for U256 {
    fn from(value: [u16; 16]) -> Self {
        let mut out = [0u8; 32];

        for (i, v) in value.into_iter().enumerate() {
            out[2 * i..2 * i + 2].copy_from_slice(&v.to_be_bytes());
        }

        U256(out)
    }
}

/// Attempts to downcast a `U256` into `u16` (fails if high bytes are non-zero).
impl TryFrom<U256> for u16 {
    type Error = ();

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        if value.0[..30].iter().any(|&b| b != 0) {
            return Err(());
        }

        Ok(u16::from_be_bytes([value.0[30], value.0[31]]))
    }
}

/// Promotes a `u16` into big-endian `U256`.
impl From<u16> for U256 {
    fn from(value: u16) -> Self {
        let mut out = [0u8; 32];
        out[30] = (value >> 8) as u8;
        out[31] = (value & 0xFF) as u8;

        U256(out)
    }
}
