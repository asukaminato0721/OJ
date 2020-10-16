T = int(input())
for _ in range(T):
    N = int(input())
    dup = set()
    pl, pr, ans = 0, 0, 0
    snow = [input() for _ in range(N)]
    while pr < N:
        if snow[pr] not in dup:
            dup.add(snow[pr])
            ans = max(ans, pr-pl+1)
        else:
            while pl < pr and snow[pl] != snow[pr]:
                dup.remove(snow[pl])
                pl += 1
            pl += 1
        pr += 1
    print(ans)
