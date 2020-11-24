for _ in range(int(input())):
    a, b = map(int, input().split())
    a, b = min(a, b), max(a, b)
    times_of_10, remain = divmod(b - a, 10)
    print(times_of_10 + int(remain > 0))
