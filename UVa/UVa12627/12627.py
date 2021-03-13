from itertools import accumulate, repeat
from operator import mul

c = [1] + list(accumulate(repeat(3, 30), mul))


def f(k: int, i: int) -> int:
    if i == 0:
        return 0
    if k == 0:
        return 1
    if i < 1 << (k - 1):
        return 2 * f(k - 1, i)
    else:
        return f(k - 1, i - (1 << (k - 1))) + 2 * c[k - 1]


T = int(input())
for _ in range(T):
    k, a, b = map(int, input().split())
    print(f"Case {_ + 1}: {f(k, b) - f(k, a - 1)}")
