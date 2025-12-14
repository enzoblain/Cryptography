//! Conversions between `U256` and `u64` (and arrays of 64-bit words).

use super::U256;

/// Splits a `U256` into 4 big-endian `u64` words.
impl From<U256> for [u64; 4] {
    fn from(value: U256) -> Self {
        let mut out = [0u64; 4];

        for (i, chunk) in value.0.chunks_exact(8).enumerate() {
            out[i] = u64::from_be_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
            ]);
        }

        out
    }
}

/// Builds a `U256` from 4 big-endian `u64` words.
impl From<[u64; 4]> for U256 {
    fn from(value: [u64; 4]) -> Self {
        let mut out = [0u8; 32];

        for (i, v) in value.into_iter().enumerate() {
            out[i * 8..i * 8 + 8].copy_from_slice(&v.to_be_bytes());
        }

        U256(out)
    }
}

/// Attempts to downcast a `U256` into `u64` (fails if high bytes are non-zero).
impl TryFrom<U256> for u64 {
    type Error = ();

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        if value.0[..24].iter().any(|&b| b != 0) {
            return Err(());
        }

        Ok(u64::from_be_bytes([
            value.0[24],
            value.0[25],
            value.0[26],
            value.0[27],
            value.0[28],
            value.0[29],
            value.0[30],
            value.0[31],
        ]))
    }
}

/// Promotes a `u64` into big-endian `U256`.
impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&value.to_be_bytes());

        U256(out)
    }
}
