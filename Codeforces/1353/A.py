def foo():
    return map(int, input().split())


for _ in range(int(input())):
    n, m = foo()
    if n == 1:
        print(0)
    elif n == 2:
        print(m)
    else:
        print(2 * m)
