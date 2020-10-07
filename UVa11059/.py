from functools import reduce
from operator import mul
cnt = 1
while True:
    try:
        n = int(input())
        num = [int(i) for i in input().split()]
        tmp = max(reduce(mul, num[i:j], 1)
                  for i in range(n) for j in range(i+1, n+1))
        if tmp <= 0:
            print("Case #%d: The maximum product is 0.\n" % (cnt))
        else:
            print("Case #%d: The maximum product is %d.\n" % (cnt, tmp))
        cnt += 1
        input()
    except EOFError:
        break
