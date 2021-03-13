from collections import defaultdict

sol = defaultdict(list)
for i in range(100005):
    sol[i + sum(int(i) for i in str(i))].append(i)
T = int(input())
for _ in range(T):
    print(min(sol.get(int(input()), [0])))
