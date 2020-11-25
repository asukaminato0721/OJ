# A

for _ in range(int(input())):
    input()
    num_list = sorted(map(int, input().split()))
    print("YES" if all(j - i <= 1 for i, j in zip(num_list, num_list[1:])) else "NO")

# B

def foo():
    return list(map(int, input().split()))


for _ in range(int(input())):
    input()
    a, b = foo(), foo()
    Amin, Bmin = min(a), min(b)
    print(sum(max(i - Amin, j - Bmin) for i, j in zip(a, b)))
