extern crate masked_permute;
use masked_permute::Permutations;

fn main() {
    let actual = Permutations::over(0b1011, 2).iter().collect::<Vec<u32>>();
    let expected = vec![0b0011, 0b1001, 0b1010];
    assert_eq!(actual, expected);
}
