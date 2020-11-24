# https://www.cnblogs.com/luckyblock/p/13916522.html

from collections import Counter


def foo():
    return map(int, input().split())


for _ in range(int(input())):
    if _:
        input()
    _, s = foo()
    a, cnt = foo(), Counter((*foo(),))
    for i in a:
        canfind = False
        for j in range(s - i, 0, -1):
            if cnt[j] > 0:
                canfind = True
                cnt[j] -= 1
                break
    print("YES" if canfind else "NO")
