use crate::primitives::U256;

pub const H256_INIT: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

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

#[inline(always)]
fn compress(block: &[u8; 64], state: &mut [u32; 8]) {
    let mut w = [0u32; 64];

    for (i, item) in w.iter_mut().enumerate().take(16) {
        let base = i * 4;
        *item = u32::from_be_bytes([
            block[base],
            block[base + 1],
            block[base + 2],
            block[base + 3],
        ]);
    }

    expand_w!(w);

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;

    all_rounds!(a, b, c, d, e, f, g, h, w, K256);

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

pub fn sha256(input: &[u8]) -> U256 {
    let mut state = H256_INIT;

    let mut i = 0;
    let len = input.len();

    while i + 64 <= len {
        let block = input[i..i + 64].try_into().unwrap();
        compress(block, &mut state);
        i += 64;
    }

    let mut block = [0u8; 64];
    let rem = len - i;

    block[..rem].copy_from_slice(&input[i..]);
    block[rem] = 0x80;

    if rem > 55 {
        compress(&block, &mut state);
        block = [0; 64];
    }

    let bit_len = (len as u64) << 3;
    block[56..].copy_from_slice(&bit_len.to_be_bytes());

    compress(&block, &mut state);

    U256::from(state)
}
