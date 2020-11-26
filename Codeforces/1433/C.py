def foo():
    return map(int, input().split())


for _ in range(int(input())):
    input()
    _list = list(foo())
    if len(set(_list)) == 1:
        print(-1)
        continue
    _max = max(_list)
    indexs = [i for i, j in enumerate(_list) if j == _max]
    for i in indexs:
        if i == 0 and _list[1] < _max:
            print(1)
            break
        elif i == len(_list) - 1 and _list[-2] < _max:
            print(i + 1)
            break
        elif 0 < i < len(_list) - 1 and (_list[i - 1] != _max or _list[i + 1] != _max):
            print(i + 1)
            break
    else:
        print(-1)
