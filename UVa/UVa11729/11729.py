from operator import itemgetter
cnt = 1
while True:
    n = int(input())
    if n == 0:
        break
    case = sorted(([int(i) for i in input().split()]
                   for j in range(n)), key=itemgetter(1), reverse=True)
    s, ans = 0, 0
    for i in case:
        s += i[0]
        ans = max(ans, s + i[1])
    print('Case %d: %d' % (cnt, ans))
    cnt += 1
