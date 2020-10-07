from functools import reduce
from operator import mul
from itertools import count
for cnt in count(1):
    try:
        input()
        num = [int(i) for i in input().split()]
        tmp = max(reduce(mul, num[i:j], 1)
                  for i, _ in enumerate(num) for j in range(i+1, len(num)+1))
        if tmp <= 0:
            print("Case #%d: The maximum product is 0.\n" % (cnt))
        else:
            print("Case #%d: The maximum product is %d.\n" % (cnt, tmp))
        input()
    except EOFError:
        break
