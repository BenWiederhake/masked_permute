// masked_permute — Fast lexicographically next permutation *within* a bitmask
// Copyright (C) 2017  Ben Wiederhake
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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
    println!("comes  {:032b}", actual);
    println!("within {:032b}", mask);
}
