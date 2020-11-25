for _ in range(int(input())):
    input()
    _list = sorted(map(int, input().split()))
    print(min(j - i for i, j in zip(_list, _list[1:])))
