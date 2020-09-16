n = int(input())
左手, 右手 = map(int, input().split())
sol = [tuple(map(int, input().split())) for i in range(n)]
sol.sort(key=lambda x: x[0]*x[1])
maxs = 左手//sol[0][1]
for i, j in sol:
    maxs = max(maxs, 左手//j)
    左手 *= i
print(maxs)
