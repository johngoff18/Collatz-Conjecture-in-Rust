# Collatz-Problem-in-Rust
Boilerplate rust code to prove that the Collatz Problem converges to 4,2,1 repeating cycle at it approaches infinity

Rust cost uses a struct that contains a u128 integer with a maximum value of 340282366920938463463374607431768211455.

The project is contained in a cargo build environment and has helper methods to view the logic in plain English.

The code DOES NOT contain methods that perform logic on the sequence, each step of the problem is simply printed and once 4 is reached, the sequence jumps to the next number to be tested.

CONCLUSION:
On my current hardware using Rust the programming language, The Collatz Problem converges to the 4,2,1 on all values up to 2^127-1.  Nothing further can be proven using this current method until u256 or larger are created.
