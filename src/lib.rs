pub mod raw;

/// Captures the concept of the permutation space itself.
/// As indicated by the pluralized name,
/// this does not describe a single permutation.
pub struct Permutations {
    mask: u32,
    count: u32,
    bases: Vec<u32>,
    start: Option<u32>,
}

/// Iterator object, yields single permutations and has a 'next' method.
/// Not supposed to be stored explicitly.
pub struct PermIter<'a> {
    bases: &'a [u32],
    mask: u32,
    ones: u32,
    next: Option<u32>,
}

impl<'a> Iterator for PermIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let to_return: Option<u32> = self.next;
        if let Some(last_perm) = self.next {
            self.next = raw::advance(self.bases, self.ones, last_perm);
        }
        to_return
    }
}

impl Permutations {
    pub fn over(mask: u32, count: u32) -> Self {
        if mask.count_ones() < count {
            Permutations {
                mask: mask,
                count: count,
                bases: vec![],
                start: None,
            }
        } else {
            let bases = raw::make_bases(mask);
            let start = *bases.last().unwrap();
            Permutations {
                mask: mask,
                count: count,
                bases: bases,
                start: Some(start),
            }
        }
    }

    pub fn iter(&self) -> PermIter {
        PermIter{
            bases: self.bases.as_slice(),
            mask: self.mask,
            ones: self.count,
            next: self.start,
        }
    }
}

/// Provide a non-consuming iterator.
impl<'a> IntoIterator for &'a Permutations {
    type Item = u32;
    type IntoIter = PermIter<'a>;

    fn into_iter(self) -> PermIter<'a> {
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

/*
#[test]
fn test_perm_for_consuming() {
    let mut perms = Vec::<u32>::with_capacity(3);
    let perms_obj = Permutations::over(0b1101000, 1);
    for p in perms_obj {
        perms.push(p);
    }
    assert_eq!(perms, vec![0b0001000, 0b0100000, 0b1000000]);
} // */

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
