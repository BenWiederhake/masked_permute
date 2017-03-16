# DEPRECATION NOTICE

I don't maintain this code any longer.  Feel free to use or fork it.
I'd be happy to link to your fork.

See below what replacement I found for my use case.

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

Note that in some cases, you can avoid this entire situation:
*Counting* within a bitmask is surprisingly easy, and my use case
actually works better with this.  Specifically:

- [Counting DOWN: `for (int submask = mask; submask; submask = (submask - 1) & mask) {`](https://www.quora.com/What-are-some-of-the-amazing-math-tricks-that-you-have-come-across-as-a-coder/answer/Darius-Marian)
- [Counting UP: `int u = 0; do printf( "%d\n", u ); while(u = u-v&v);`](https://www.quora.com/What-are-some-of-the-amazing-math-tricks-that-you-have-come-across-as-a-coder/answer/Glenn-Rhoads)

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [Performance](#performance)
- [TODOs](#todos)
- [Contribute](#contribute)

## Background

### Usual bit-permutations

You may already know the "usual" algorithm for computing the
lexicographically next permutation of bits in an `int` from the
[bithacks page](https://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation)
or elsewhere:

```C
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

Masked permutations are a strict generalization.  In other words:
`masked_permute` can emulate the "normal" next-permutation algorithm by
just setting the mask to `!0u32`.

Specifically, let's assume you have a mask `0b1001_1110_0110_0111u32`
and want to iterate over each value that "chooses" exactly
two non-identical set bits of this mask.  Then it would produce these values:

* `0b0000_0000_0000_0011u32`
* `0b0000_0000_0000_0101u32`
* `0b0000_0000_0000_0110u32`
* `0b0000_0000_0010_0001u32`
* `0b0000_0000_0010_0010u32`
* `0b0000_0000_0010_0100u32`
* `0b0000_0000_0100_0001u32`
* and so on.

As you can see, the "middle bits" were just skipped.

"But that's easy!  It's not NP-hard!"  you might say.  And you're right,
it's pretty easy to do this in a more readable way using sets or lists
or any other higher-level representation.  You might even go so far and
use the original algorithm together with a masked-expand step.

However, that would (necessarily) still be slower than this.
This implementation directly enumerates the desired values,
not wasting any time on overhead (as abstractions would inevitably cause in this case),
non-permutations (as some naive approaches might generate internally and then skip over),
or inefficient instructions (see notes in [Install](#install)).

### Corner cases

In cases where there are no such permutations, the `Iterator` will appear to be empty;
just like one would expect.
Technically, this abstraction costs another conditional branch that could be saved,
which is why the `raw` module contains the raw computational primitives
that don't need any conditional branching whatsever.

When `mask == 0`, this is actually a special case of the above,
unless you want a permutation where 0 bits are set.
That, of course, exists.

This project does not explicitly support starting with an arbitrary permutation yet,
but it might be added if there's actually a use case for that.
[Tell me](#contribute) if there is!

## Install

Add at an appropriate position to your `Cargo.toml`:

```TOML
[dependencies]
masked_permute = { git = "https://github.com/BenWiederhake/masked_permute.git" }
```

That should be it.  You'll be glad to hear that `masked_permute` itself
does not have any dependencies.

### Additional step for best performance

For best performance, you should allow `rustc` (or in this case, LLVM actually)
to use special instructions that can speed up execution even more.
Specifically, this library makes extensive use of `u32::count_ones()`,
which could be compiled to the single special-purpose instruction `popcnt`.

To enable this instruction, add this to your `.cargo/config` file
[somewhere up the tree](http://doc.crates.io/config.html#hierarchical-structure):

```TOML
[build]
rustflags = ["-C", "target-feature=+popcnt"]
#rustflags = ["-C", "target-cpu=native"]
```

Feel free to be even more specific about the target architecture.
I only highlighted this singular instruction, as it is available
on all common architectures, and has the most impact, as far as the
current benchmarks are concerned.

<!--
  Assuming that the processor doesn't already recognize the pattern and
  optimize on its own.  In this case, `popcnt` might still be of advantage
  because of the limited instruction cache.
  The "bitcount hack" is pretty long!
-->

## Usage

Just use it!  No dependencies, and it's short enough.
The complexity lies in constructing an algorithm,
not in writing the code.

```Rust
extern crate masked_permute;
use masked_permute::Permutations;

let mask = 0b1001_1110_0110_0111u32;
for x in Permutations::over(mask, 3) {
    // Prints all permutations of 3 bits where 'x & mask == x'.
    println!("{:032b}", x);
}
```

## Performance

As expected, enumerating permutations is very fast (<10ns/permutation),
but the initial setup needs a little time (<80ns):

```
test bench_make_bases_large ... bench:          78 ns/iter (+/- 9)
test bench_make_bases_small ... bench:          39 ns/iter (+/- 2)

test bench_huge_perm        ... bench:     190,226 ns/iter (+/- 12,519)
         which sums over permutations:      24,310
                                    =>           8 ns/perm (+/- 1)

test bench_many_perm        ... bench:       1,077 ns/iter (+/- 89)
         which sums over permutations:         126
                                    =>           9 ns/perm (+/- 1)

test bench_few_perm         ... bench:         154 ns/iter (+/- 31)
         which sums over permutations:          10
                                    =>          15 ns/perm (+/- 3)

test bench_tiny_perm        ... bench:          83 ns/iter (+/- 8)
         which sums over permutations:           3
                                    =>          27 ns/perm (+/- 3)
```

(Raw data generated by `cargo bench`, with additional calculations by me.)

## TODOs

Next up are these:
* Fiddle around and analyse performance with:
    * Using arrays instead of `Vec` should be faster.
        Investigate why `historic/iter-uses-arrays` isn't faster.
    * Struct order?
* Possible extensions:
    * Try to compile with `nostdlib` or whatever that's called
    * Build for different kinds of integers
    * Find out other ways to make it attractive for
      micro-controller-like environments, too.
* Ask people for feedback on:
    * Performance methodology, performance improvements
    * Making it "Idiomatic Rust"
    * Other Iterator-like interfaces that might be interesting to provide

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/masked_permute/issues/new) or submit PRs.
