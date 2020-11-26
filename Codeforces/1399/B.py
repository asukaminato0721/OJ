def foo():
    return list(map(int, input().split()))


for _ in range(int(input())):
    input()
    a, b = foo(), foo()
    Amin, Bmin = min(a), min(b)
    print(sum(max(i - Amin, j - Bmin) for i, j in zip(a, b)))
