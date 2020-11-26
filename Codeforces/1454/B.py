from collections import Counter

for _ in range(int(input())):
    input()
    _list = list(map(int, input().split()))
    index = min((i for i, j in Counter(_list).items() if j == 1), default=-1)
    print(_list.index(index) + 1 if index != -1 else -1)
