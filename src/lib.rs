//! Gödel numbering — maps sequences of natural numbers to a unique natural number
//! using prime factorization: ⟨a₁, a₂, …, aₙ⟩ = 2^a₁ · 3^a₂ · 5^a₃ · … · pₙ^aₙ

/// Encode a sequence of non-negative integers into a Gödel number.
pub fn encode(sequence: &[u64]) -> u64 {
    let primes = first_n_primes(sequence.len());
    let mut result: u64 = 1;
    for (prime, &exp) in primes.iter().zip(sequence.iter()) {
        result *= prime.pow(exp as u32);
    }
    result
}

/// Decode a Gödel number back into the original sequence of exponents.
/// Returns the exponents for primes 2, 3, 5, 7, … up to the largest prime factor.
pub fn decode(godel_number: u64) -> Vec<u64> {
    if godel_number == 0 {
        return vec![];
    }
    let mut n = godel_number;
    let mut exponents = Vec::new();
    let mut candidate = 2u64;
    loop {
        let mut exp = 0u64;
        while n % candidate == 0 {
            n /= candidate;
            exp += 1;
        }
        exponents.push(exp);
        if n == 1 {
            // Trim trailing zeros
            while exponents.last() == Some(&0) {
                exponents.pop();
            }
            return exponents;
        }
        candidate = next_prime(candidate);
    }
}

/// Compute the Gödel number for a single number (shorthand).
pub fn godel_number_single(n: u64) -> u64 {
    2u64.pow(n as u32)
}

fn first_n_primes(n: usize) -> Vec<u64> {
    let mut primes = Vec::with_capacity(n);
    let mut candidate = 2u64;
    while primes.len() < n {
        if is_prime(candidate) {
            primes.push(candidate);
        }
        candidate += 1;
    }
    primes
}

fn next_prime(n: u64) -> u64 {
    let mut candidate = n + 1;
    while !is_prime(candidate) {
        candidate += 1;
    }
    candidate
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let seq = vec![1, 2, 3];
        let gn = encode(&seq);
        assert_eq!(decode(gn), seq);
    }

    #[test]
    fn test_empty() {
        assert_eq!(encode(&[]), 1);
        assert_eq!(decode(1), vec![]);
    }

    #[test]
    fn test_single() {
        assert_eq!(encode(&[5]), 32); // 2^5
        assert_eq!(decode(32), vec![5]);
    }
}
