from itertools import accumulate, repeat
from operator import mul

for _ in range(int(input())):
    n = int(input())
    left = sum(accumulate(repeat(2, n // 2 - 1), mul)) + pow(2, n)
    right = (2 * (1 - pow(2, n))) // (1 - 2) - left
    print(abs(left - right))
