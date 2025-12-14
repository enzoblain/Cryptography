//! Conversions between `U256` and `u128` (and arrays of 128-bit halves).

use super::U256;

/// Splits a `U256` into two big-endian `u128` halves.
impl From<U256> for [u128; 2] {
    fn from(value: U256) -> Self {
        let mut hi = [0u8; 16];
        let mut lo = [0u8; 16];

        hi.copy_from_slice(&value.0[..16]);
        lo.copy_from_slice(&value.0[16..]);

        [u128::from_be_bytes(hi), u128::from_be_bytes(lo)]
    }
}

/// Builds a `U256` from two big-endian `u128` halves.
impl From<[u128; 2]> for U256 {
    fn from(value: [u128; 2]) -> Self {
        let mut out = [0u8; 32];

        out[..16].copy_from_slice(&value[0].to_be_bytes());
        out[16..].copy_from_slice(&value[1].to_be_bytes());

        U256(out)
    }
}

/// Attempts to downcast a `U256` into `u128` (fails if high bytes are non-zero).
impl TryFrom<U256> for u128 {
    type Error = ();

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        if value.0[..16].iter().any(|&b| b != 0) {
            return Err(());
        }

        let mut buf = [0u8; 16];
        buf.copy_from_slice(&value.0[16..]);

        Ok(u128::from_be_bytes(buf))
    }
}

/// Promotes a `u128` into big-endian `U256`.
impl From<u128> for U256 {
    fn from(value: u128) -> Self {
        let mut out = [0u8; 32];
        out[16..].copy_from_slice(&value.to_be_bytes());

        U256(out)
    }
}
