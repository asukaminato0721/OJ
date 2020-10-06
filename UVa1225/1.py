from collections import defaultdict
sol = defaultdict(int)
T = int(input())
for _ in range(T):
    sol.clear()
    n = int(input())
    line = ''.join(str(i) for i in range(1, n+1))
    for i in line:
        sol[i] += 1
    print(*(sol[str(i)] for i in range(10)))
