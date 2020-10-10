from collections import deque
while True:
    n, m = [int(i) for i in input().split()]
    if n == 0 and m == 0:
        break
    if n > m:
        print("Loowater is doomed!")
        continue
    dragon = deque(sorted(int(input()) for i in range(n)))
    people = sorted(int(input()) for i in range(m))
    s = 0
    for i in people:
        if i >= dragon[0]:
            dragon.popleft()
            s += i
        if len(dragon) == 0:
            print(s)
            break
    if len(dragon) > 0:
        print("Loowater is doomed!")
