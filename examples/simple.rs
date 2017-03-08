extern crate masked_permute;
use masked_permute::Permutations;

fn main() {
    let mask = 0b10000000010111;
    let amount = 3;
    println!("These are all permutations of {} bits within the mask {:032b}:",
        amount, mask);

    for p in Permutations::over(mask, amount) {
        println!("{:032b}", p);
    }

    println!("That's it!");
}
