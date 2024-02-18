pub fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut primes = vec![true; limit + 1];
    let mut p = 2;

    while p * p <= limit {
        if primes[p] {
            let mut i = p * p;
            while i <= limit {
                primes[i] = false;
                i += p;
            }
        }
        p += 1;
    }

    (2..limit).filter(|&p| primes[p]).collect()
}

// fn main() {
//     let prime_numbers = sieve_of_eratosthenes(100);
//     println!("{:?}", prime_numbers);
// }
