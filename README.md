# Gödel Numbering

**A Rust library for encoding and decoding sequences of natural numbers using Gödel numbering** — the prime factorization technique that maps sequences to unique integers via `⟨a₁,...,aₙ⟩ = 2^a₁ · 3^a₂ · 5^a₃ · ... · pₙ^aₙ`.

## Why It Matters

Kurt Gödel's 1931 incompleteness theorem relied on this elegant encoding to arithmetize metamathematics — representing logical formulas, proofs, and computation as natural numbers. By showing that any formal system capable of arithmetic can encode its own proof predicate, Gödel established that sufficiently powerful formal systems cannot be both consistent and complete. Beyond logic, Gödel numbering is the theoretical foundation of data serialization: it proves that arbitrary data structures can be losslessly encoded as integers. It also connects to the fundamental theorem of arithmetic (unique prime factorization) and appears in computability theory (Kleene's T predicate, register machines).

## How It Works

**Encoding** takes a sequence `[a₁, a₂, ..., aₙ]` and computes `∏ pᵢ^aᵢ` where `pᵢ` is the i-th prime. The library generates the first n primes using trial division with 6k±1 optimization (checking divisibility only by candidates of the form `6k±1` after handling 2 and 3). Each exponentiation `p^a` uses Rust's built-in `pow` method.

**Decoding** factorizes the Gödel number by trial division: repeatedly dividing by each prime in ascending order and counting the multiplicity. The process terminates when the quotient reaches 1, and trailing-zero exponents are trimmed. For a Gödel number with largest prime factor `pₖ`, this is **O(pₖ²)** in the worst case due to trial division.

The library also provides `godel_number_single(n) = 2^n` as a shorthand for single-element sequences.

## Quick Start

```rust
use godel_number::{encode, decode, godel_number_single};

fn main() {
    // Encode a sequence
    let seq = vec![1, 2, 3];
    let gn = encode(&seq);
    println!("Gödel number of [1,2,3] = {}", gn);
    // 2^1 · 3^2 · 5^3 = 2 · 9 · 125 = 2250

    // Decode it back
    let recovered = decode(gn);
    println!("Recovered: {:?}", recovered); // [1, 2, 3]

    // Single number encoding
    println!("2^5 = {}", godel_number_single(5)); // 32
}
```

## API

| Function | Complexity | Description |
|---|---|---|
| `encode(sequence)` | **O(n · log pₙ)** | Encode `[a₁,...,aₙ]` via prime factorization |
| `decode(godel_number)` | **O(√N)** | Decode via trial division factorization |
| `godel_number_single(n)` | **O(1)** | Shorthand: `2^n` |

## Architecture Notes

Part of the SuperInstance mathematical logic and computation theory collection. See the [Architecture Guide](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
