from collections import Counter
T = int(input())
for _ in range(T):
    m, n = map(int, input().split())
    DNAT = list(zip(*(input() for _ in range(m))))
    s = ''.join(max(Counter(i).items(), key=lambda x: (x[-1], -ord(x[0])))[0] for i in DNAT)
    Sum = sum(sum(1 for i in k if i != j) for j, k in zip(s, DNAT))
    print(s, Sum, sep='\n')
