//! SHA-256 computation functions and round operations.
//!
//! This module provides low-level helper functions used in the SHA-256 compression function,
//! including the bitwise operations and the 64-round main loop.
//!
//! # Bitwise Operations
//!
//! SHA-256 uses several specialized bitwise operations (sigma functions and choice/majority):
//!
//! | Function | Purpose | Formula |
//! |----------|---------|---------|
//! | `small_sigma0` | Word expansion | ROTR(7) ⊕ ROTR(18) ⊕ SHR(3) |
//! | `small_sigma1` | Word expansion | ROTR(17) ⊕ ROTR(19) ⊕ SHR(10) |
//! | `big_sigma0` | State rotation | ROTR(2) ⊕ ROTR(13) ⊕ ROTR(22) |
//! | `big_sigma1` | State rotation | ROTR(6) ⊕ ROTR(11) ⊕ ROTR(25) |
//! | `ch` | Choice function | (e ∧ f) ⊕ (¬e ∧ g) |
//! | `maj` | Majority function | (a ∧ b) ⊕ (a ∧ c) ⊕ (b ∧ c) |
//!
//! # Computation Modes
//!
//! This module provides two implementations of the 64 rounds:
//! - **Standard** (default): Uses safe array bounds
//! - **Speed** (with "speed" feature): Uses unsafe unrolled macros for performance
//!
//! # References
//!
//! - FIPS 180-4: Secure Hash Standard (SHS)
//! - IEEE 754: Floating-point standard

pub use super::K256;

/// Computes the SHA-256 small sigma0 function.
///
/// Used in word schedule expansion (W[t] calculation).
///
/// # Formula
///
/// Σ₀(x) = ROTR(7, x) ⊕ ROTR(18, x) ⊕ SHR(3, x)
///
/// Where ROTR is rotate right, SHR is shift right, and ⊕ is XOR.
///
/// # Arguments
///
/// * `x` - Input 32-bit value
///
/// # Returns
///
/// Result of the small sigma0 operation
#[inline(always)]
pub fn small_sigma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

/// Computes the SHA-256 small sigma1 function.
///
/// Used in word schedule expansion (W[t] calculation).
///
/// # Formula
///
/// Σ₁(x) = ROTR(17, x) ⊕ ROTR(19, x) ⊕ SHR(10, x)
///
/// Where ROTR is rotate right, SHR is shift right, and ⊕ is XOR.
///
/// # Arguments
///
/// * `x` - Input 32-bit value
///
/// # Returns
///
/// Result of the small sigma1 operation
#[inline(always)]
pub fn small_sigma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

/// Computes the SHA-256 big sigma0 function.
///
/// Used in the main compression loop for state updates.
///
/// # Formula
///
/// Σ₀(x) = ROTR(2, x) ⊕ ROTR(13, x) ⊕ ROTR(22, x)
///
/// Where ROTR is rotate right and ⊕ is XOR.
///
/// # Arguments
///
/// * `x` - Input 32-bit value
///
/// # Returns
///
/// Result of the big sigma0 operation
#[inline(always)]
pub fn big_sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

/// Computes the SHA-256 big sigma1 function.
///
/// Used in the main compression loop for state updates.
///
/// # Formula
///
/// Σ₁(x) = ROTR(6, x) ⊕ ROTR(11, x) ⊕ ROTR(25, x)
///
/// Where ROTR is rotate right and ⊕ is XOR.
///
/// # Arguments
///
/// * `x` - Input 32-bit value
///
/// # Returns
///
/// Result of the big sigma1 operation
#[inline(always)]
pub fn big_sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

/// Computes the SHA-256 choice function.
///
/// Selects bits from `f` where `e` is 1, and bits from `g` where `e` is 0.
///
/// # Formula
///
/// Ch(e, f, g) = (e ∧ f) ⊕ (¬e ∧ g)
///
/// Where ∧ is bitwise AND, ¬ is bitwise NOT, and ⊕ is XOR.
///
/// # Arguments
///
/// * `e` - Selector value (32-bit)
/// * `f` - First choice value (32-bit)
/// * `g` - Second choice value (32-bit)
///
/// # Returns
///
/// Result of the choice operation
#[inline(always)]
pub fn ch(e: u32, f: u32, g: u32) -> u32 {
    (e & f) ^ ((!e) & g)
}

/// Computes the SHA-256 majority function.
///
/// Returns the majority bit for each bit position across the three inputs.
///
/// # Formula
///
/// Maj(a, b, c) = (a ∧ b) ⊕ (a ∧ c) ⊕ (b ∧ c)
///
/// Where ∧ is bitwise AND and ⊕ is XOR.
///
/// # Arguments
///
/// * `a` - First input (32-bit)
/// * `b` - Second input (32-bit)
/// * `c` - Third input (32-bit)
///
/// # Returns
///
/// Result of the majority operation
#[inline(always)]
pub fn maj(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (a & c) ^ (b & c)
}

/// Executes all 64 rounds of the SHA-256 compression function.
///
/// Standard SHA-256 implementation that computes the 64 rounds through a loop
/// instead of using an unrolled round structure.
/// Used when the "speed" feature is not enabled.
///
/// # Algorithm
///
/// The function maintains 8 working variables (a-h) and performs 64 rounds:
///
/// For each round i (0..64):
/// 1. If i >= 16: Expand the message schedule: W[i] = σ₁(W[i-2]) + W[i-7] + σ₀(W[i-15]) + W[i-16]
/// 2. Calculate: T1 = h + Σ₁(e) + Ch(e,f,g) + K[i] + W[i]
/// 3. Calculate: T2 = Σ₀(a) + Maj(a,b,c)
/// 4. Update: a←T1+T2, e←d+T1, and rotate other variables
///
/// # Arguments
///
/// * `state` - Current hash state [a, b, c, d, e, f, g, h], updated in-place
/// * `w` - Message schedule array (16 values, circular buffer)
///
/// # Safety
///
/// Uses unsafe unchecked indexing with circular addressing (via `$i & 15`).
/// This is safe because all indices are statically verified to be within bounds.
#[cfg(not(feature = "speed"))]
pub fn all_rounds(state: &mut [u32; 8], mut w: [u32; 16]) {
    // Load hash state into working variables
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    for i in 0..64 {
        if i >= 16 {
            unsafe {
                // Circular buffer indexing: access W[i-2], W[i-7], W[i-15], W[i-16] via modulo 16
                let w16 = *w.get_unchecked((i - 16) & 15);
                let w15 = *w.get_unchecked((i - 15) & 15);
                let w7 = *w.get_unchecked((i - 7) & 15);
                let w2 = *w.get_unchecked((i - 2) & 15);

                let s0 = small_sigma0(w15);
                let s1 = small_sigma1(w2);

                *w.get_unchecked_mut(i & 15) =
                    w16.wrapping_add(s0).wrapping_add(w7).wrapping_add(s1);
            }
        }

        let wi = unsafe { *w.get_unchecked(i & 15) };
        let ki = unsafe { *K256.get_unchecked(i) };

        let bs1 = big_sigma1(e);
        let ch = ch(e, f, g);

        let bs0 = big_sigma0(a);
        let maj = maj(a, b, c);

        let t1 = h
            .wrapping_add(bs1)
            .wrapping_add(ch)
            .wrapping_add(wi)
            .wrapping_add(ki);

        let t2 = bs0.wrapping_add(maj);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

/// Executes all 64 rounds of the SHA-256 compression function (optimized version).
///
/// This is the performance-optimized implementation enabled via the `"speed"` feature.
/// It fully unrolls all 64 rounds using macros, mutates the 16-word circular message
/// schedule directly, and removes loop overhead for maximum throughput.
/// This mode produces a heavier binary due to full unrolling, but is significantly faster.
///
/// # Algorithm
///
/// Similar to the standard implementation, but with optimizations:
/// - Unrolled round macro for better instruction scheduling
/// - Direct mutation of the mutable `w` array instead of copying
/// - Four macro invocations per block for fewer instruction dependencies
///
/// # Arguments
///
/// * `state` - Current hash state [a, b, c, d, e, f, g, h], updated in-place
/// * `w` - Mutable reference to the message schedule array (16 values, circular buffer)
///
/// # Performance Notes
///
/// This version may be faster on some CPUs due to:
/// - Better compiler optimization of unrolled code
/// - Reduced function call overhead from macro expansion
/// - Improved instruction-level parallelism from scheduling
///
/// # Safety
///
/// Uses unsafe unchecked indexing with circular addressing (via `$i & 15`).
/// This is safe because all indices are statically verified to be within bounds.
#[cfg(feature = "speed")]
pub fn all_rounds(state: &mut [u32; 8], w: &mut [u32; 16]) {
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    macro_rules! R {
        ($i:expr) => {{
            if $i >= 16 {
                unsafe {
                    // Circular buffer indexing: access W[i-2], W[i-7], W[i-15], W[i-16] via modulo 16
                    let w16 = *w.get_unchecked(($i - 16) & 15);
                    let w15 = *w.get_unchecked(($i - 15) & 15);
                    let w7 = *w.get_unchecked(($i - 7) & 15);
                    let w2 = *w.get_unchecked(($i - 2) & 15);

                    let s0 = small_sigma0(w15);
                    let s1 = small_sigma1(w2);

                    *w.get_unchecked_mut($i & 15) =
                        w16.wrapping_add(s0).wrapping_add(w7).wrapping_add(s1);
                }
            }

            let wi = unsafe { *w.get_unchecked($i & 15) };
            let ki = unsafe { *K256.get_unchecked($i) };

            let bs1 = big_sigma1(e);
            let ch = ch(e, f, g);

            let bs0 = big_sigma0(a);
            let maj = maj(a, b, c);

            let t1 = h
                .wrapping_add(bs1)
                .wrapping_add(ch)
                .wrapping_add(wi)
                .wrapping_add(ki);

            let t2 = bs0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }};
    }

    R!(0);
    R!(1);
    R!(2);
    R!(3);
    R!(4);
    R!(5);
    R!(6);
    R!(7);
    R!(8);
    R!(9);
    R!(10);
    R!(11);
    R!(12);
    R!(13);
    R!(14);
    R!(15);

    R!(16);
    R!(17);
    R!(18);
    R!(19);
    R!(20);
    R!(21);
    R!(22);
    R!(23);
    R!(24);
    R!(25);
    R!(26);
    R!(27);
    R!(28);
    R!(29);
    R!(30);
    R!(31);

    R!(32);
    R!(33);
    R!(34);
    R!(35);
    R!(36);
    R!(37);
    R!(38);
    R!(39);
    R!(40);
    R!(41);
    R!(42);
    R!(43);
    R!(44);
    R!(45);
    R!(46);
    R!(47);

    R!(48);
    R!(49);
    R!(50);
    R!(51);
    R!(52);
    R!(53);
    R!(54);
    R!(55);
    R!(56);
    R!(57);
    R!(58);
    R!(59);
    R!(60);
    R!(61);
    R!(62);
    R!(63);

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}
