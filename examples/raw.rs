extern crate masked_permute;
use masked_permute::raw;

fn main() {
    let mask = 0b0010_1110_0000_1001u32;
    let bases = raw::make_bases(mask);
    let prev = 0b0010_0000_0000_1001u32;
    // Alternatively, to get the "3" you *could* use prev.count_ones().
    // However, this provides an opportunity to shave off this
    // method/instruction when the caller already knows it anyway.
    let ones = 3;

    let actual = raw::advance(&bases, ones, prev);
    println!("After  {:032b}", prev);
    // We know that there is a next one, so just 'unwrap()'.
    println!("comes  {:032b}", actual.unwrap());
    println!("within {:032b}", mask);
}
