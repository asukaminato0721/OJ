# A

for _ in range(int(input())):
    n = int(input())
    print(n, *range(1, n))

# B

from collections import Counter

for _ in range(int(input())):
    input()
    _list = list(map(int, input().split()))
    index = min((i for i, j in Counter(_list).items() if j == 1), default=-1)
    print(_list.index(index) + 1 if index != -1 else -1)

# D

from collections import Counter


def prime_factors(n: int) -> list:
    i = 2
    factors = []
    while i * i <= n:
        if n % i:
            i += 1
        else:
            n //= i
            factors.append(i)
    if n > 1:
        factors.append(n)
    return factors


for _ in range(int(input())):
    n = int(input())
    primes = Counter(prime_factors(n))
    num, num_of_most = primes.most_common(1)[0]
    res = [*[num] * (num_of_most - 1), n // pow(num, num_of_most - 1)]
    print(len(res))
    print(*res)
