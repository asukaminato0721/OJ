for _ in range(int(input())):
    a, b, n = map(int, input().split())
    cnt = 0
    while a <= n and b <= n:
        a, b = max(a, b) + min(a, b), max(a, b)
        cnt += 1
    print(cnt)
