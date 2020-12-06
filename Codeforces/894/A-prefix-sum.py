from itertools import accumulate
from operator import add

line = input()
len_ = len(line)
prefix_sum = list(accumulate((int(x == "Q") for x in line), add))
print(sum(int(j == "A") * prefix_sum[i] * (prefix_sum[-1] - prefix_sum[i]) for i, j in enumerate(line)))
