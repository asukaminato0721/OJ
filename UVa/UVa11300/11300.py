from itertools import accumulate, repeat
from operator import add

while True:
    try:
        n = int(input())
        case = [int(input()) for _ in range(n)]
        M = sum(case) // n
        C = sorted(accumulate((-i + j for i, j in zip(repeat(M), case)), add))
        mid = C[n // 2]
        print(sum(abs(mid - i) for i in C))
    except EOFError:
        break

# https://www.cnblogs.com/acm-bingzi/p/3198280.html
