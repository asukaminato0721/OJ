from collections import Counter
n = int(input())
for _ in range(n):
    input()
    T = int(input())
    print('\n' if _ else '', end='')
    case = list(zip(*([int(i) for i in input().split()]
                      for i in range(T))))
    line12 = Counter([i+j for i in case[0] for j in case[1]])
    print(sum(line12[-c-d] for c in case[2] for d in case[3]))
