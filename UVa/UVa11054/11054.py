from itertools import accumulate, islice
from operator import add
while True:
    try:
        n = int(input())
        case = (int(_) for _ in input().split())
        print(sum(abs(i) for i in islice(accumulate(case, add), 0, n - 1, 1)))
    except EOFError:
        break
