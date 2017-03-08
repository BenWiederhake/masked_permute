#[cfg(test)]
use std;

/// Creates a 'bases' lookup table, needed by 'advance'.
pub fn make_bases(mask: u32, bases_into: &mut [u32; 33]) {
    // mask.count_ones() must be a valid index,
    // so 'bases_into[32]' must be an actual element.
    let mut rem_mask = mask;
    let mut acc_mask = 0;

    for field in bases_into[0..32].iter_mut() {
        *field = acc_mask;

        // Determine it.  An overflow can only happen when we're about
        // to "0 & " anyway, so don't can about that behavior.
        let lowest_bit = rem_mask & 0u32.wrapping_sub(rem_mask);
        rem_mask &= !lowest_bit;  // Remove it from "remaining" mask
        acc_mask |= lowest_bit;  // Add it to the "accumulated" mask
    }
    bases_into[32] = acc_mask;

    assert_eq!(acc_mask, mask);
    debug_assert_eq!(bases_into[mask.count_ones() as usize], mask);
}

/// Allocates and returns a 'bases' lookup table.
/// Large return type!
/// Use as a helper function.
pub fn create_bases(mask: u32) -> [u32; 33] {
    let mut bases = [0u32; 33];
    make_bases(mask, &mut bases);
    bases
}

#[test]
fn test_bases_0() {
    let mut bases = [0u32; 33];
    let expected = [0u32; 33];
    make_bases(0b0, &mut bases);
    assert_eq!(&bases[..], &expected[..]);
}

#[test]
fn test_bases_1() {
    let expected : [u32; 33] =
        [0b0, 0b1, 0b1, 0b1, 0b1, 0b1, 0b1, 0b1,
         0b1, 0b1, 0b1, 0b1, 0b1, 0b1, 0b1, 0b1,
         0b1, 0b1, 0b1, 0b1, 0b1, 0b1, 0b1, 0b1,
         0b1, 0b1, 0b1, 0b1, 0b1, 0b1, 0b1, 0b1, 0b1];
    assert_eq!(&create_bases(0b1)[..], &expected[..]);
}

#[test]
fn test_bases_11() {
    let expected : [u32; 33] =
        [0b00, 0b01, 0b11, 0b11, 0b11, 0b11, 0b11, 0b11,
         0b11, 0b11, 0b11, 0b11, 0b11, 0b11, 0b11, 0b11,
         0b11, 0b11, 0b11, 0b11, 0b11, 0b11, 0b11, 0b11,
         0b11, 0b11, 0b11, 0b11, 0b11, 0b11, 0b11, 0b11, 0b11];
    assert_eq!(&create_bases(0b11)[..], &expected[..]);
}

#[test]
fn test_bases_11011() {
    let expected : [u32; 33] =
        [0b00000, 0b00001, 0b00011, 0b01011, 0b11011, 0b11011, 0b11011, 0b11011,
         0b11011, 0b11011, 0b11011, 0b11011, 0b11011, 0b11011, 0b11011, 0b11011,
         0b11011, 0b11011, 0b11011, 0b11011, 0b11011, 0b11011, 0b11011, 0b11011,
         0b11011, 0b11011, 0b11011, 0b11011, 0b11011, 0b11011, 0b11011, 0b11011,
         0b11011];
    assert_eq!(&create_bases(0b11011)[..], &expected[..]);
}

#[test]
fn test_bases_full() {
    let expected : [u32; 33] =
        [0x00000000, 0x00000001, 0x00000003, 0x00000007, 0x0000000F, 0x0000001F, 0x0000003F, 0x0000007F,
         0x000000FF, 0x000001FF, 0x000003FF, 0x000007FF, 0x00000FFF, 0x00001FFF, 0x00003FFF, 0x00007FFF,
         0x0000FFFF, 0x0001FFFF, 0x0003FFFF, 0x0007FFFF, 0x000FFFFF, 0x001FFFFF, 0x003FFFFF, 0x007FFFFF,
         0x00FFFFFF, 0x01FFFFFF, 0x03FFFFFF, 0x07FFFFFF, 0x0FFFFFFF, 0x1FFFFFFF, 0x3FFFFFFF, 0x7FFFFFFF, 0xFFFFFFFF];
    assert_eq!(&create_bases(0xFFFFFFFF)[..], &expected[..]);
}

/// Computes the lexicographically next permutation of the bitmask `last_pos`
/// *within* the bitmask implied by `bases`.  Heavily inspired by
/// [~seander's bithacks](https://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation).
/// 
/// All math operations are intended to run very speedily on both 32-bit
/// and 64-bit hardware.  This is considered the "hot path".
/// Note that the initialization are expected to be called significantly less often,
/// so 'make_bases' can be considered cold.
///
/// Note that this implements wrap-around.  So inputting the lexicographically
/// *last* permutation will yield the lexicographically *first* permutation.
pub fn advance(bases: &[u32; 33], ones: u32, last_perm: u32) -> u32 {
    // Hot path, so disable some assertions for release.
    debug_assert_eq!(last_perm.count_ones(), ones);
    let mask = bases[32];

    // Set it up
    let t = last_perm | last_perm.wrapping_sub(1) | !mask;
    /*
     * The ".wrapping_sub" is some hackery that needs justification.
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
    let next_upper = t.overflowing_add(1).0 & mask;
    // This is essentially the "(((~t & -~t) - 1) >> (__builtin_ctz(v) + 1))"-part,
    // but for arbitrary bitmasks, and precomputed.
    let need_ones = ones - next_upper.count_ones();
    let next_lower = bases[need_ones as usize];
    next_upper | next_lower
}

#[test]
fn test_advance_11_1() {
    let bases = create_bases(0b11);
    assert_eq!(advance(&bases, 1, 0b01), 0b10);
    assert_eq!(advance(&bases, 1, 0b10), 0b01);
}

#[test]
fn test_advance_1101_1() {
    let bases = create_bases(0b1101);
    assert_eq!(advance(&bases, 1, 0b0001), 0b0100);
    assert_eq!(advance(&bases, 1, 0b0100), 0b1000);
    assert_eq!(advance(&bases, 1, 0b1000), 0b0001);
}

#[test]
fn test_advance_1101_2() {
    let bases = create_bases(0b1101);
    assert_eq!(advance(&bases, 2, 0b0101), 0b1001);
    assert_eq!(advance(&bases, 2, 0b1001), 0b1100);
    assert_eq!(advance(&bases, 2, 0b1100), 0b0101);
}

#[test]
fn test_advance_11011_2() {
    let bases = create_bases(0b11011);
    assert_eq!(advance(&bases, 2, 0b00011), 0b01001);
    assert_eq!(advance(&bases, 2, 0b01001), 0b01010);
    assert_eq!(advance(&bases, 2, 0b01010), 0b10001);
    assert_eq!(advance(&bases, 2, 0b10001), 0b10010);
    assert_eq!(advance(&bases, 2, 0b10010), 0b11000);
    assert_eq!(advance(&bases, 2, 0b11000), 0b00011);
}

#[test]
fn test_advance_corner_1() {
    let u32max = std::u32::MAX;

    let bases = create_bases(0b0);
    assert_eq!(advance(&bases, 0, 0b0000), 0b0000);

    let bases = create_bases(u32max);
    assert_eq!(advance(&bases, 31, u32max - 1), (u32max - 1) >> 1);
    assert_eq!(advance(&bases, 32, u32max), u32max);

    let bases = create_bases(0x8000_0001);
    assert_eq!(advance(&bases, 1, 0x0000_0001), 0x8000_0000);
    assert_eq!(advance(&bases, 1, 0x8000_0000), 0x0000_0001);
}
