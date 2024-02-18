
def sieve_of_eratosthenes(limit):
    primes = [True] * (limit + 1)
    p = 2

    while p * p <= limit:
        if primes[p]:
            for i in range(p * p, limit + 1, p):
                primes[i] = False
        p += 1

    prime_numbers = [p for p in range(2, limit) if primes[p]]
