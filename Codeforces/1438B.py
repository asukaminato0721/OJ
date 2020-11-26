# 假如数字存在一样的，就直接把它们选为需要的子列表，不然因为二进制的关系，和不可能一样

from collections import Counter

for _ in range(int(input())):
    input()
    print("YES" if any(j > 1 for j in Counter(input().split()).values()) else "NO")
