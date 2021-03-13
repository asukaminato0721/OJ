from functools import partial
from operator import itemgetter

sort = partial(sorted, key=itemgetter(2, 1))


X = [0] * 5005
Y = [0] * 5005


def solve(pts: list, ans: list) -> bool:
    vis = [False] * 5005
    for idx, pl, pr in pts:
        while pl <= pr and vis[pl]:
            pl += 1
        if pl == pr + 1:
            return False
        ans[idx] = pl
        vis[pl] = True
    return True


while True:
    n = int(input())
    if n == 0:
        break
    pts = [list(map(int, input().split())) for i in range(n)]
    pts_x = sort([[i, xli, xri] for i, (xli, yli, xri, yri) in enumerate(pts)])
    pts_y = sort([[i, yli, yri] for i, (xli, yli, xri, yri) in enumerate(pts)])
    if (not solve(pts_x, X)) or (not solve(pts_y, Y)):
        print("IMPOSSIBLE")
    else:
        for i in zip(X[:n], Y[:n]):
            print(*i)
