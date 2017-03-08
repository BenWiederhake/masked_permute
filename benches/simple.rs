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
    let mut bases = [77u32; 33];
    b.iter(|| {
        raw::make_bases(0b1101u32, &mut bases);
        bases[3]
    });
}

#[bench]
fn bench_make_bases_large(b: &mut Bencher) {
    let mut bases = [77u32; 33];
    b.iter(|| {
        raw::make_bases(0xFA77AFE7u32, &mut bases);
        bases[29]
    });
}
