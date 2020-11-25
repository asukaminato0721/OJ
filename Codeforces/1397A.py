from itertools import chain
from collections import Counter

for _ in range(int(input())):
    n = int(input())
    print("YES" if all(i % n == 0 for i in Counter((*chain(*(input() for i in range(n))),)).values()) else "NO")

# way 2

from collections import Counter

for _ in range(int(input())):
    n = int(input())
    print("YES" if all(i % n == 0 for i in Counter("".join(input() for i in range(n))).values()) else "NO")
