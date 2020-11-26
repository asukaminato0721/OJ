# 贪心，先把较小的数填入每个数组的前 ⌈n/2⌉−1 个元素，即前 (⌈n/2⌉−1)*k 是不要的，从 (⌈n/2⌉−1)*k+1 开始，然后依次填完每个数组即可。
# 然后把中位数加起来，中位数步长为 n - begin_pos

from math import ceil


def foo():
    return map(int, input().split())


for _ in range(int(input())):
    n, k = foo()
    a = [0] + list(foo())
    begin_pos = ceil(n / 2) - 1
    print(sum(a[begin_pos * k + 1 :: n - begin_pos]))
