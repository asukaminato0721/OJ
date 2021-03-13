from bisect import bisect_left
from itertools import count

for _ in count(1):
    N, Q = map(int, input().split())
    if N == 0 and Q == 0:
        break
    print(f"CASE# {_}:")
    case = sorted(int(input()) for _ in range(N))
    for j in range(Q):
        x = int(input())
        true = bisect_left(case, x)
        if true < N and case[true] == x:
            print(x, "found at", true + 1)
        else:
            print(x, "not found")
