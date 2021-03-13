from collections import Counter

T = int(input())
for _ in range(T):
    n = int(input())
    sol = Counter("".join(str(i) for i in range(1, n + 1)))
    print(*(sol[str(i)] for i in range(10)))
