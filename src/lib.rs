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


pub mod raw;

/// Captures the concept of the permutation space itself.
/// As indicated by the pluralized name,
/// this does not describe a single permutation.
pub struct Permutations {
    mask: u32,
    count: u32,
}

/// Iterator object, yields single permutations and has a 'next' method.
/// Not supposed to be stored explicitly.
pub struct PermIter {
    bases: Vec<u32>,
    ones: u32,
    next: Option<u32>,
}

impl Iterator for PermIter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let to_return: Option<u32> = self.next;
        if let Some(last_perm) = self.next {
            let next_perm = raw::advance(&self.bases, self.ones, last_perm);
            if next_perm > last_perm {
                self.next = Some(next_perm);
            } else {
                self.next = None;
            }
        }
        to_return
    }
}

impl Permutations {
    pub fn over(mask: u32, count: u32) -> Self {
        Permutations {
            mask: mask,
            count: count,
        }
    }

    pub fn iter(&self) -> PermIter {
        if self.mask.count_ones() < self.count {
            // Use invalid values in many places, in order to fail-fast
            // if anything goes wrong.
            PermIter{
                bases: vec![],
                ones: u32::max_value(),
                next: None,
            }
        } else {
            let bases = raw::make_bases(self.mask);
            assert!(self.count < 33);
            let start: u32 = bases[self.count as usize];
            PermIter{
                bases: bases,
                ones: self.count,
                next: Some(start),
            }
        }
    }
}

/// Provide a consuming iterator,
/// even though there is no "consumption" going on.
/// This simplifies some code, but may be considered bad style.
impl IntoIterator for Permutations {
    type Item = u32;
    type IntoIter = PermIter;

    fn into_iter(self) -> PermIter {
        self.iter()
    }
}

/// Provide a non-consuming iterator.
impl<'a> IntoIterator for &'a Permutations {
    type Item = u32;
    type IntoIter = PermIter;

    fn into_iter(self) -> PermIter {
        self.iter()
    }
}

#[test]
fn test_perm_iter_corner() {
    assert_eq!(None, Permutations::over(0b111, 4).iter().next());
    assert_eq!(None, Permutations::over(0b0, 1).iter().next());
    assert_eq!(Some(0xFFFF_FFFF), Permutations::over(0xFFFF_FFFF, 32).iter().next());
}

#[test]
fn test_perm_iter_corner2() {
    let mut i = Permutations::over(0b0, 0).iter();
    assert_eq!(Some(0b0), i.next());
    assert_eq!(None, i.next());
}

#[test]
fn test_perm_iter_simple() {
    let mut i = Permutations::over(0b11101101, 2).iter();
    assert_eq!(Some(0b00000101), i.next());
    assert_eq!(Some(0b00001001), i.next());
    assert_eq!(Some(0b00001100), i.next());
    assert_eq!(Some(0b00100001), i.next());
    assert_eq!(Some(0b00100100), i.next());
    assert_eq!(Some(0b00101000), i.next());
    assert_eq!(Some(0b01000001), i.next());
}

#[test]
fn test_perm_iter_near_full() {
    let mut i = Permutations::over(0b11101101, 5).iter();
    assert_eq!(Some(0b01101101), i.next());
    assert_eq!(Some(0b10101101), i.next());
    assert_eq!(Some(0b11001101), i.next());
    assert_eq!(Some(0b11100101), i.next());
    assert_eq!(Some(0b11101001), i.next());
    assert_eq!(Some(0b11101100), i.next());
    assert_eq!(None, i.next());
}

#[test]
fn test_perm_iter_full() {
    let mut i = Permutations::over(0b11101101, 6).iter();
    assert_eq!(Some(0b11101101), i.next());
    assert_eq!(None, i.next());
}

#[test]
fn test_perm_collect() {
    let p = Permutations::over(0b1101, 1).iter().collect::<Vec<_>>();
    assert_eq!(p, vec![0b0001, 0b0100, 0b1000]);
}

#[test]
fn test_perm_for_consuming() {
    let mut perms = Vec::<u32>::with_capacity(3);
    let perms_obj = Permutations::over(0b1101000, 1);
    for p in perms_obj {
        perms.push(p);
    }
    assert_eq!(perms, vec![0b0001000, 0b0100000, 0b1000000]);
}

#[test]
fn test_perm_for_ref() {
    let mut perms = Vec::<u32>::with_capacity(3);
    let perms_obj = Permutations::over(0b1101, 1);
    for p in &perms_obj {
        perms.push(p);
    }
    for p in &perms_obj {
        perms.push(p);
    }
    assert_eq!(perms, vec![0b0001, 0b0100, 0b1000, 0b0001, 0b0100, 0b1000]);
}
