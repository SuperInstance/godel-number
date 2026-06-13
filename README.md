# godel-number: Gödel Numbering via Prime Factorization

Encodes sequences of natural numbers into a single natural number using **Gödel's incompleteness encoding** — the fundamental construction from Gödel's (1931) incompleteness theorems that maps formal arithmetic statements to integers.

## Why It Matters

Gödel numbering is the foundation of **mathematical logic** and **computability theory**. It proves that any finite sequence — whether a list of numbers, a logical formula, or a computer program — can be encoded as a single integer and recovered losslessly. This insight enabled:

- **Gödel's First Incompleteness Theorem**: Any sufficiently powerful formal system contains true statements that cannot be proved within the system
- **Computability**: Turing used Gödel numbering to show that programs can be inputs to programs (universal machines)
- **Cryptography**: RSA and other factoring-based schemes depend on the difficulty of reversing this encoding
- **Kolmogorov complexity**: The shortest program producing a string is well-defined because programs have Gödel numbers

## How It Works

### Encoding

Given a sequence (a₁, a₂, ..., aₙ), assign each position a prime number and use the sequence elements as exponents:

$$\langle a_1, a_2, \ldots, a_n \rangle = 2^{a_1} \cdot 3^{a_2} \cdot 5^{a_3} \cdot 7^{a_4} \cdots p_n^{a_n}$$

where pᵢ is the i-th prime (2, 3, 5, 7, 11, 13, ...).

By the **Fundamental Theorem of Arithmetic** (unique prime factorization), this mapping is **bijective**: every sequence maps to a unique integer, and every positive integer decodes to exactly one sequence.

### Decoding

To recover the sequence, factor the Gödel number and read off exponents:

```
decode(N):
  for each prime p from 2 upward:
    count how many times p divides N → exponent e_i
    append e_i to result
    if N = 1: stop and trim trailing zeros
```

### Primality Testing

The `is_prime` function uses the **6k±1 optimization**: after checking 2 and 3, test divisors of the form 6k−1 and 6k+1 up to √n.

**Complexity**: O(√n) per primality test.

### Example

```
encode([1, 2, 3]) = 2¹ · 3² · 5³ = 2 · 9 · 125 = 2250

decode(2250):
  2250 / 2 = 1125 (exp=1)
  1125 / 3 = 375, / 3 = 125 (exp=2)
  125 / 5 = 25, / 5 = 5, / 5 = 1 (exp=3)
  → [1, 2, 3] ✓
```

### Complexity

| Operation | Time | Space | Notes |
|-----------|------|-------|-------|
| `encode(seq)` | O(n · √p_max) | O(n) | n primes, each found by trial division |
| `decode(N)` | O(√N · log N) | O(log N) | Factor by trial division up to √N |
| `is_prime(n)` | O(√n) | O(1) | 6k±1 trial division |

**Warning**: Because Gödel numbers grow exponentially, this implementation is limited to small sequences of small numbers before `u64` overflow. For [1,2,3] the result is 2250; for [10,10,10] it would need 2¹⁰ · 3¹⁰ · 5¹⁰ ≈ 6 × 10¹⁵.

## Quick Start

```rust
use godel_number::{encode, decode};

let seq = vec![1, 2, 3];
let gn = encode(&seq);          // 2¹·3²·5³ = 2250
assert_eq!(gn, 2250);

let recovered = decode(gn);
assert_eq!(recovered, seq);     // perfect roundtrip
```

## API

| Function | Signature | Description |
|----------|-----------|-------------|
| `encode` | `(&[u64]) -> u64` | Encode sequence to Gödel number |
| `decode` | `(u64) -> Vec<u64>` | Decode Gödel number to sequence |
| `godel_number_single` | `(u64) -> u64` | Shorthand: 2^n |

Internal: `first_n_primes(n)`, `next_prime(n)`, `is_prime(n)`.

## Architecture Notes

This is a **γ (gamma)** module — pure mathematics with a bijective encoding guarantee. In the γ + η = C framework, Gödel numbering is the ultimate γ: it proves that structure (sequences, logic, programs) can be reduced to numbers. The **η** layer builds interpretation on top — for example, a theorem prover that encodes propositions as Gödel numbers and checks derivability.

## References

- Gödel, K. (1931). *Über formal unentscheidbare Sätze der Principia Mathematica und verwandter Systeme I*. Monatshefte für Mathematik und Physik 38, 173–198.
- Boolos, G. S., Burgess, J. P., & Jeffrey, R. C. (2007). *Computability and Logic* (5th ed.). Cambridge.
- Smullyan, R. M. (1992). *Gödel's Incompleteness Theorems*. Oxford University Press.

## License

MIT
