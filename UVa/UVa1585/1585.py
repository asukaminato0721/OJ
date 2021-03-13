def score(n):
    return n * (n + 1) // 2


T = int(input())
for i in range(T):
    line = input().replace("X", " ").split()
    print(sum(score(len(j)) for j in line))
