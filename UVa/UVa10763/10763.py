from collections import defaultdict

people = defaultdict(int)
while True:
    n = int(input())
    if n == 0:
        break
    people.clear()
    for i in range(n):
        k = tuple(input().split())
        if people[k[::-1]] > 0:
            people[k[::-1]] -= 1
        else:
            people[k] += 1
    if all(j == 0 for j in people.values()):
        print("YES")
    else:
        print("NO")
