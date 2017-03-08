# masked_permute

> Computes the lexicographically next permutation *within* a bitmask.

Assume you want a highly efficient way to enumerate all permutations of
very few elements, but don't consider "all" elements; whatever that means
in your context.

Sure you could just write a high-level algorithm for that which uses sets
and probably vectors and what-not, using thousands of instructions per permutation.
You could even use a standard permutation-enumerating algorithm and then map it back,
using dozens of instructions.

Or just use this library, which only needs very few instructions per permute step,
and is straight-forward to use, as it provides an `Iterator<u32>`.

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [TODOs](#todos)
- [Contribute](#contribute)

## Background

### Usual bit-permutations

You may already know the "usual" algorithm for computing the
lexicographically next permutation of bits in an `int` from the
[bithacks page](https://graphics.stanford.edu/~seander/bithacks.html)
or elsewhere:

```
unsigned int v; // current permutation of bits 
unsigned int w; // next permutation of bits

unsigned int t = v | (v - 1); // t gets v's least significant 0 bits set to 1
// Next set to 1 the most significant bit to change, 
// set to 0 the least significant ones, and add the necessary 1 bits.
w = (t + 1) | (((~t & -~t) - 1) >> (__builtin_ctz(v) + 1));
```

<!--
  Copied in good faith that "Public Domain" and proper attribution mean
  that it's okay to share.  Please contact me if that's not the case:
  BenWiederhake.GitHub@gmail.com
-->

Starting with a value like `0000 1001`, this produces sequences like this:

* `0000 1010`
* `0000 1100`
* `0001 0001`
* `0001 0010`
* `0001 0100`
* `0001 1000`
* `0010 0001`
* and so on.  I think you get the idea.

### Masked permutations

FIXME: Explain

### Corner cases

0-bit mask, (absence of) wrap-around

## Install

Add at an appropriate position to your `Cargo.toml`:

```
[dependencies]
masked_permut = { git = "https://github.com/BenWiederhake/masked_permute.git" }
```

That should be it.  You'll be glad to hear that `masked_permute` itself
does not have any dependencies.

<!-- FIXME: Check whether that acutally works! -->

## Usage

Just use it!  No dependencies, and it's short enough.
The complexity lies in constructing an algorithm,
not in writing the code.

```
extern crate masked_permute;
use masked_permute::Permutations;

let mask = 0b1001_1110_0110_0111u32;
for x in Permutations::over(mask, 3) {
    // Prints all permutations of 3 bits where 'x & mask == x'.
    println!("{:032b}", x);
}
```

<!-- FIXME: Check whether that actually compiles. -->

## TODOs

Next up are these:
* Import code from "that one" project
* Care about integration issues
* Set up performance testing so that I actually know what's happening
* Fiddle around and analyse performance with:
    * Using arrays(?) instead of `Vec`
    * Keeping `bases` in `Permutations` instead of `PermIter`
* Possible extensions:
    * Try to compile with `nostdlib` or whatever that's called
    * Build for different kinds of integers
    * Find out other ways to make it attractive for
      micro-controller-like environments, too.
* Ask people for feedback on:
    * Performance methodology, performance improvements
    * Making it "Idiomatic Rust"
    * Iterator-like interfaces that might be interesting to provide

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/masked_permute/issues/new) or submit PRs.
