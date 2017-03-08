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

#![feature(test)]
#![cfg(test)]

extern crate test;
use test::Bencher;

extern crate masked_permute;
use masked_permute::Permutations;
use masked_permute::raw;

#[bench]
fn bench_huge_perm(b: &mut Bencher) {
    // Iterates over 24310 elements.
    b.iter(|| {
        let mut i = 0;
        for p in Permutations::over(0b11010101_11101011_11101101u32, 8) {
            i += p;
        }
        i
    });
}

#[bench]
fn bench_many_perm(b: &mut Bencher) {
    // Iterates over 126 elements.
    b.iter(|| {
        let mut i = 0;
        for p in Permutations::over(0b1110_0010_0110_1101u32, 5) {
            i += p;
        }
        i
    });
}

#[bench]
fn bench_few_perm(b: &mut Bencher) {
    // Iterates over 10 elements.
    b.iter(|| {
        let mut i = 0;
        for p in Permutations::over(0b0110_1101u32, 2) {
            i += p;
        }
        i
    });
}

#[bench]
fn bench_tiny_perm(b: &mut Bencher) {
    // Iterates over 3 elements.
    b.iter(|| {
        let mut i = 0;
        for p in Permutations::over(0b1101u32, 1) {
            i += p;
        }
        i
    });
}

#[bench]
fn bench_make_bases_small(b: &mut Bencher) {
    b.iter(|| {
        raw::make_bases(0b1101u32)
    });
}

#[bench]
fn bench_make_bases_large(b: &mut Bencher) {
    b.iter(|| {
        raw::make_bases(0b11010101_11101011_11101101u32)
    });
}
