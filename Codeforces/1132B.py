input()
_list = sorted(map(int, input().split()))
_sum = sum(_list)
input()
for i in map(int, input().split()):
    print(_sum - _list[-i])
