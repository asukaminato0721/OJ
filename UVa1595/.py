from collections import defaultdict
T = int(input())
left_plus_right = set()
pts = defaultdict(list)
for i in range(T):
    left_plus_right.clear()
    pts.clear()
    n = int(input())
    for j in range(n):
        x, y = input().split()
        pts[int(y)].append(int(x))
    for pt in pts.values():
        pt.sort()
        for k in range(len(pt)//2+(len(pt) % 2)):
            left_plus_right.add(pt[k]+pt[-(k+1)])
    if len(left_plus_right) == 1:
        print('YES')
    else:
        print('NO')
