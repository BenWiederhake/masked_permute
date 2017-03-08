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
use masked_permute::Permutations;

fn main() {
    let actual = Permutations::over(0b1011, 2).iter().collect::<Vec<u32>>();
    let expected = vec![0b0011, 0b1001, 0b1010];
    assert_eq!(actual, expected);
}
