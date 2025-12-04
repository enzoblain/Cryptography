//! Core SHA-256 compression and hashing functions.
//!
//! This module contains the main hashing logic for SHA-256, including the block compression
//! function and the high-level `sha256` function that processes arbitrary-length input.
//!
//! # Algorithm
//!
//! SHA-256 processes input messages in 512-bit (64-byte) blocks. The algorithm:
//!
//! 1. **Initialization**: Starts with initial hash values ([`H256_INIT`](super::H256_INIT))
//! 2. **Padding**: Appends padding bits and the original message length
//! 3. **Block Processing**: For each 512-bit block, runs the compression function
//! 4. **Output**: Returns the final hash state as a 256-bit value
//!
//! # Performance Considerations
//!
//! - Uses `unsafe` code for performance-critical operations
//! - Performs unaligned reads for processing input blocks
//! - Implements a 16-word circular buffer for the message schedule (`W`)
//! - Removes bounds checks via unchecked indexing (`get_unchecked`)
//! - Supports two computation modes via the `"speed"` Cargo feature
//!
//! # Functions
//!
//! - [`compress`]: Processes a single 512-bit block
//! - [`sha256`]: Hashes an arbitrary-length input

use super::H256_INIT;
use super::computations::all_rounds;
use crate::primitives::U256;

use core::ptr::{copy_nonoverlapping, read_unaligned};

/// Compresses a single 512-bit (64-byte) block using the SHA-256 compression function.
///
/// This function implements the main SHA-256 compression function which updates the state
/// based on the input block. The compression consists of 64 rounds of operations.
///
/// # Arguments
///
/// * `block` - A 64-byte block to process
/// * `state` - The current hash state (8 x 32-bit values), updated in-place
///
/// # Safety
///
/// This function uses unsafe code to perform unaligned reads from the input block
/// for performance. The block parameter must be valid for at least 64 bytes.
///
/// # Details
///
/// The function:
/// 1. Converts the input block into 16 words (32-bit values) in big-endian format
/// 2. Calls the round computation function with the state and word schedule
/// 3. Updates the hash state with the computed values
#[inline(always)]
pub fn compress(block: &[u8; 64], state: &mut [u32; 8]) {
    let mut w = [0u32; 16];

    for (i, slot) in w.iter_mut().enumerate().take(16) {
        // Read unaligned u32 and convert from big-endian to native byte order
        let ptr = unsafe { block.as_ptr().add(i * 4) as *const u32 };
        *slot = u32::from_be(unsafe { read_unaligned(ptr) });
    }

    #[cfg(not(feature = "speed"))]
    all_rounds(state, w);

    #[cfg(feature = "speed")]
    all_rounds(state, &mut w);
}

/// Computes the SHA-256 hash of the input data.
///
/// This function is the main entry point for hashing. It processes the input message
/// of arbitrary length and produces a 256-bit hash value.
///
/// # Arguments
///
/// * `input` - The input data to hash (any length, including zero)
///
/// # Returns
///
/// A [`U256`] containing the 256-bit hash output
///
/// # Algorithm Steps
///
/// 1. **Block Processing**: Iterates through complete 64-byte blocks, compressing each
/// 2. **Padding**: Handles the final partial block with proper SHA-256 padding:
///    - Appends a single '1' bit (0x80 byte)
///    - Pads with zeros to align to 56 bytes (mod 64)
///    - If necessary, creates an additional block for padding
/// 3. **Length Encoding**: Appends the original message length in bits (64-bit big-endian)
/// 4. **Final Compression**: Processes the padded final block(s)
/// 5. **Result**: Returns the hash state as a U256 value
///
/// # Example
///
/// ```ignore
/// let hash = sha256(b"hello");
/// ```
///
/// # Performance
///
/// The function supports two optimization levels:
/// - **Standard mode**: Uses safe array indexing
/// - **Speed mode** (with "speed" feature): Uses unsafe indexing with circular buffering
///
/// # Safety
///
/// This function uses `unsafe` code internally for:
/// - Pointer arithmetic when copying input data
/// - Unaligned memory reads
/// - Circular buffer indexing
///
/// However, the function is safe to call with any input slice.
pub fn sha256(input: &[u8]) -> U256 {
    let mut state = H256_INIT;

    let mut i = 0;
    let len = input.len();

    while i + 64 <= len {
        // Transmute slice to 64-byte block pointer
        let block: &[u8; 64] = unsafe { &*(input.as_ptr().add(i) as *const [u8; 64]) };
        compress(block, &mut state);
        i += 64;
    }

    let mut block = [0u8; 64];
    let rem = len - i;

    unsafe {
        let src = input.as_ptr().add(i);
        let dst = block.as_mut_ptr();

        copy_nonoverlapping(src, dst, rem);
        *block.as_mut_ptr().add(rem) = 0x80; // SHA-256 padding bit
    }

    if rem > 55 {
        // Need extra block for message length
        compress(&block, &mut state);
        block = [0; 64];
    }

    let bit_len = (len as u64) << 3; // Convert bytes to bits
    let len_bytes = bit_len.to_be_bytes();

    unsafe {
        // Insert message lenght in the last 8 byte
        let src = len_bytes.as_ptr();
        let dst = block.as_mut_ptr().add(56);

        copy_nonoverlapping(src, dst, 8);
    }

    compress(&block, &mut state);

    U256::from(state)
}
