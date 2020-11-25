def foo():
    return map(int, input().split())


for _ in range(int(input())):
    n, d = foo()
    _list = list(foo())
    _sum = 0
    for i, j in enumerate(_list[1:], 1):
        if d < i * j:
            _sum += d // i
            break
        _sum += j
        d -= i * j
    print(_sum + _list[0])
