import time
import statistics

import prime_number_calculation

LIMIT:int = 1000000
NUM:int = 10
def main():

    l = []
    for _ in range(NUM):
        start = time.perf_counter()
        prime_number_calculation.sieve_of_eratosthenes(LIMIT)
        duration = time.perf_counter() - start
        print(f"処理時間 = {duration}s")
        l.append(duration)

    mean = statistics.mean(l)
    print(f'平均処理時間: {mean}s')

    


if __name__ == '__main__':
    main()
