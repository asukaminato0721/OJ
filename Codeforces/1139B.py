input()
_list = list(map(int, input().split()))[::-1]
if len(_list) == 1:
    print(_list[0])
    exit(0)
_sum, current = [_list[0]] * 2
for i in _list[1:]:
    current = max(0, min(current - 1, i))
    _sum += current
print(_sum)
