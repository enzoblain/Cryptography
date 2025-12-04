//! SHA-256 cryptographic hash implementation.
//!
//! This module provides a complete implementation of the SHA-256 (Secure Hash Algorithm 256-bit)
//! cryptographic hash function as defined in FIPS 180-4. SHA-256 is a member of the SHA-2 family
//! of hash functions and produces a 256-bit (32-byte) hash value.
//!
//! # Overview
//!
//! SHA-256 is widely used in cryptographic applications, blockchain systems, and security protocols.
//! This implementation focuses on performance while maintaining correctness.
//!
//! # Features
//!
//! - **Two computation modes**: a standard loop-based implementation, and an
//!   optimized `"speed"` mode (fully unrolled, heavier binary, but faster).
//! - **Performance optimized**: Uses unsafe code carefully for performance-critical operations
//! - **Memory efficient**: Operates on 64-byte blocks as per SHA-256 specification
//! - **Padding handling**: Automatic message padding and length encoding
//!
//! # Constants
//!
//! - [`H256_INIT`]: Initial hash values for SHA-256
//! - [`K256`]: Round constants used in the compression function
//!
//! # Modules
//!
//! - [`core`]: Core compression and hashing logic
//! - [`computations`]: Helper functions for bitwise operations and round computations
//!
//! # Example
//!
//! ```ignore
//! use cryptography::hash::sha256;
//!
//! let input = b"hello world";
//! let hash = sha256(input);
//! ```

pub mod computations;
pub mod core;

/// Initial hash values for SHA-256.
///
/// These are the first eight prime numbers' fractional square roots,
/// as defined in the SHA-256 specification (FIPS 180-4).
///
/// Format: [H0, H1, H2, H3, H4, H5, H6, H7]
pub const H256_INIT: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

/// Round constants for SHA-256.
///
/// These 64 constants represent the first 64 bits of the fractional parts of the
/// cube roots of the first 64 prime numbers, as defined in the SHA-256 specification.
/// Used in each of the 64 rounds of the compression function.
pub const K256: [u32; 64] = [
    0x428A2F98, 0x71374491, 0xB5C0FBCF, 0xE9B5DBA5, 0x3956C25B, 0x59F111F1, 0x923F82A4, 0xAB1C5ED5,
    0xD807AA98, 0x12835B01, 0x243185BE, 0x550C7DC3, 0x72BE5D74, 0x80DEB1FE, 0x9BDC06A7, 0xC19BF174,
    0xE49B69C1, 0xEFBE4786, 0x0FC19DC6, 0x240CA1CC, 0x2DE92C6F, 0x4A7484AA, 0x5CB0A9DC, 0x76F988DA,
    0x983E5152, 0xA831C66D, 0xB00327C8, 0xBF597FC7, 0xC6E00BF3, 0xD5A79147, 0x06CA6351, 0x14292967,
    0x27B70A85, 0x2E1B2138, 0x4D2C6DFC, 0x53380D13, 0x650A7354, 0x766A0ABB, 0x81C2C92E, 0x92722C85,
    0xA2BFE8A1, 0xA81A664B, 0xC24B8B70, 0xC76C51A3, 0xD192E819, 0xD6990624, 0xF40E3585, 0x106AA070,
    0x19A4C116, 0x1E376C08, 0x2748774C, 0x34B0BCB5, 0x391C0CB3, 0x4ED8AA4A, 0x5B9CCA4F, 0x682E6FF3,
    0x748F82EE, 0x78A5636F, 0x84C87814, 0x8CC70208, 0x90BEFFFA, 0xA4506CEB, 0xBEF9A3F7, 0xC67178F2,
];
