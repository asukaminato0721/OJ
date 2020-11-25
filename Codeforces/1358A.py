for _ in range(int(input())):
    n, m = map(int, input().split())
    if any(i == 0 for i in (n % 2, m % 2)):
        print(m * n // 2)
    else:
        print((m - 1) * n // 2 + (n + 1) // 2)
