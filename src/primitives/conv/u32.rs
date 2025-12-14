//! Conversions between `U256` and `u32` (and arrays of 32-bit words).

use super::U256;

/// Splits a `U256` into 8 big-endian `u32` words.
impl From<U256> for [u32; 8] {
    fn from(value: U256) -> Self {
        let mut out = [0u32; 8];

        for (i, chunk) in value.0.chunks_exact(4).enumerate() {
            out[i] = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }

        out
    }
}

/// Builds a `U256` from 8 big-endian `u32` words.
impl From<[u32; 8]> for U256 {
    fn from(value: [u32; 8]) -> Self {
        let mut out = [0u8; 32];

        for (i, v) in value.into_iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&v.to_be_bytes());
        }

        U256(out)
    }
}

/// Attempts to downcast a `U256` into `u32` (fails if high bytes are non-zero).
impl TryFrom<U256> for u32 {
    type Error = ();

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        if value.0[..28].iter().any(|&b| b != 0) {
            return Err(());
        }

        Ok(u32::from_be_bytes([
            value.0[28],
            value.0[29],
            value.0[30],
            value.0[31],
        ]))
    }
}

/// Promotes a `u32` into big-endian `U256`.
impl From<u32> for U256 {
    fn from(value: u32) -> Self {
        let mut out = [0u8; 32];
        out[28..32].copy_from_slice(&value.to_be_bytes());

        U256(out)
    }
}
