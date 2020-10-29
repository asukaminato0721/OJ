from collections import deque
from itertools import count

for case in count(1):
    t = int(input())
    if t == 0:
        break
    print(f"Scenario #{case}")
    团队成员队列 = [deque() for _ in range(1010)]
    团队的队列 = deque()
    队 = {int(j): int(i) for i in range(t) for j in input().split()[1:]}
    while True:
        cmd = input()
        if cmd.startswith("S"):
            break
        elif cmd.startswith("E"):  # ENQUEUE
            人 = cmd.split()[-1]
            人在哪一队 = 队[int(人)]
            if len(团队成员队列[人在哪一队]) == 0:
                团队的队列.append(人在哪一队)
            团队成员队列[人在哪一队].append(人)
        else:
            最前队伍 = 团队的队列[0]
            print(f"{团队成员队列[最前队伍][0]}")
            团队成员队列[最前队伍].popleft()
            if len(团队成员队列[最前队伍]) == 0:
                团队的队列.popleft()
    print()
