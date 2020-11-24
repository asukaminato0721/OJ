for _ in range(int(input())):
    _, k = map(int, input().split())
    _list = list(map(int, input().split()))
    small = min(_list)
    times_of_small = k // small - 1
    print(sum((k - i) // small for i in _list) - times_of_small)
