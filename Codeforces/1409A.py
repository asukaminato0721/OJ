for _ in range(int(input())):
    a, b = map(int, input().split())
    times_of_10, remain = divmod(abs(b - a), 10)
    print(times_of_10 + int(remain > 0))
