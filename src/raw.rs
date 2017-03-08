#[cfg(test)]
use std;

/// Creates a 'bases' lookup table, needed by 'advance'.
// 'Vec' was probably a poor design choice.
// Should take an array into which it writes,
// thus saving a dereference in each permute step.
pub fn make_bases(mask: u32) -> Vec<u32> {
    // mask.count_ones() must be a valid index.
    let mut bases = Vec::with_capacity(33);
    let mut rem_mask = mask;
    let mut acc_mask = 0;
    bases.push(0);  // The base with 0 bits set is always 0.

    while 0 != rem_mask {
        // rustc is clever enough here to optimize `!rem_mask + 1` to `-v`.
        // Can't express that in high-level because of types.
        let lowest_bit = rem_mask & (!rem_mask + 1);  // Determine it
        rem_mask &= !lowest_bit;  // Remove it from "remaining" mask
        acc_mask |= lowest_bit;  // Add it to the "accumulated" mask
        bases.push(acc_mask);
    }

    assert!(bases.len() <= 65);
    assert_eq!(bases.len() as u32, mask.count_ones() + 1);

    bases
}

#[test]
fn test_bases_0() {
    assert_eq!(make_bases(0b0), vec![0b0]);
}

#[test]
fn test_bases_1() {
    assert_eq!(make_bases(0b1), vec![0b0, 0b1]);
}

#[test]
fn test_bases_11() {
    assert_eq!(make_bases(0b11), vec![0b00, 0b01, 0b11]);
}

#[test]
fn test_bases_11011() {
    assert_eq!(make_bases(0b11011),
        vec![0b00000, 0b00001, 0b00011, 0b01011, 0b11011]);
}

/// Computes the lexicographically next permutation of the bitmask `last_pos`
/// *within* the bitmask implied by `bases`.  Heavily inspired by
/// [~seander's bithacks](https://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation).
/// 
/// All math operations are intended to run very speedily on both 32-bit
/// and 64-bit hardware.  This is considered the "hot path".
/// Note that the initialization are expected to be called significantly less often,
/// so 'make_bases' can be considered cold.
pub fn advance(bases: &Vec<u32>, ones: u32, last_perm: u32) -> Option<u32> {
    // Hot path, so disable some assertions for release.
    debug_assert_eq!(last_perm.count_ones(), ones);
    debug_assert!(ones <= (bases.len() - 1) as u32);
    let mask = bases.last().unwrap();

    // Set it up
    let t = last_perm | last_perm.wrapping_sub(1) | !mask;
    /*
     * The ".overflowing_add" is some hackery that needs justification.
     * Here's a line of reasoning:
     * - The underflow (-1u32) only occurs when last_perm is 0.
     * - Whenever last_perm is 0, then mask is 0
     * - Whenever mask is zero, then due to the "| !mask"
     *   we can stomach any undefined *value*.
     * - 'wrapping_sub' compiles down to a single 'leal -1(%rdi)' instruction,
     *   which we need anyway
     * The point is: literally any other way of substracting would be valid,
     * as long as an underflow does not halt execution.
     */

    // Exploit the carry-chain to find the bit that will be *set* in the
    // next permutation, and clean up filler-bits.
    match t.overflowing_add(1) {
        (next_upper_raw, false) => {
            let next_upper = next_upper_raw & mask;
            // This is essentially the "(((~t & -~t) - 1) >> (__builtin_ctz(v) + 1))"-part,
            // but for arbitrary bitmasks, and precomputed.
            let need_ones = ones - next_upper.count_ones();
            let next_lower = bases[need_ones as usize];
            Some(next_upper | next_lower)
        },
        // Last permutation reached
        (_, true) => None,
    }
}

#[test]
fn test_advance_11_1() {
    let bases = make_bases(0b11);
    assert_eq!(advance(&bases, 1, 0b01), Some(0b10));
    assert_eq!(advance(&bases, 1, 0b10), None);
}

#[test]
fn test_advance_1101_1() {
    let bases = make_bases(0b1101);
    assert_eq!(advance(&bases, 1, 0b0001), Some(0b0100));
    assert_eq!(advance(&bases, 1, 0b0100), Some(0b1000));
    assert_eq!(advance(&bases, 1, 0b1000), None);
}

#[test]
fn test_advance_1101_2() {
    let bases = make_bases(0b1101);
    assert_eq!(advance(&bases, 2, 0b0101), Some(0b1001));
    assert_eq!(advance(&bases, 2, 0b1001), Some(0b1100));
    assert_eq!(advance(&bases, 2, 0b1100), None);
}

#[test]
fn test_advance_11011_2() {
    let bases = make_bases(0b11011);
    assert_eq!(advance(&bases, 2, 0b00011), Some(0b01001));
    assert_eq!(advance(&bases, 2, 0b01001), Some(0b01010));
    assert_eq!(advance(&bases, 2, 0b01010), Some(0b10001));
    assert_eq!(advance(&bases, 2, 0b10001), Some(0b10010));
    assert_eq!(advance(&bases, 2, 0b10010), Some(0b11000));
    assert_eq!(advance(&bases, 2, 0b11000), None);
}

#[test]
fn test_advance_corner_1() {
    let u32max = std::u32::MAX;

    let bases = make_bases(0b0);
    assert_eq!(advance(&bases, 0, 0b0000), None);

    let bases = make_bases(u32max);
    assert_eq!(advance(&bases, 31, u32max - 1), None);
    assert_eq!(advance(&bases, 32, u32max), None);

    let bases = make_bases(0x8000_0001);
    assert_eq!(advance(&bases, 1, 0x0000_0001), Some(0x8000_0000));
    assert_eq!(advance(&bases, 1, 0x8000_0000), None);
}
