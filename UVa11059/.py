from math import prod
from itertools import count

for cnt in count(1):
    try:
        input()
        num = [int(i) for i in input().split()]
        tmp = max(prod(num[i:j]) for i, _ in enumerate(num) for j in range(i + 1, len(num) + 1))
        if tmp <= 0:
            print(f"Case #{cnt}: The maximum product is 0.\n")
        else:
            print(f"Case #{cnt}: The maximum product is {tmp}.\n")
        input()
    except EOFError:
        break
