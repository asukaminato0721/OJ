from operator import itemgetter
from itertools import count


def isVisible(b: list, i: int, midx: float):  # 判断建筑i在midx所在区间是否可见
    if not (b[i][0] <= midx and b[i][0] + b[i][2] >= midx):
        return False  # 不在区间
    for j in range(n):
        if b[j][4] >= b[i][4] and b[j][1] < b[i][1] and b[j][0] <= midx and b[j][0] + b[j][2] >= midx:  # 更高andy更小and存在于此区间
            return False  # 建筑i被j挡住
    return True  # 不被挡住


for case in count():
    n = int(input())
    if n == 0:
        break
    block = sorted([[*(float(i) for i in input().split()), _ + 1] for _ in range(n)], key=itemgetter(0, 1))
    if case:
        print()
    print(f"For map #{case+1}, the visible buildings are numbered as follows:")
    tmp = sorted({x[0] for x in block} | {x[0] + x[2] for x in block})
    print(*filter(None, (next((bi[-1] for xj, xj1 in zip(tmp, tmp[1:]) if isVisible(block, i, (xj + xj1) / 2)), None) for i, bi in enumerate(block))))
