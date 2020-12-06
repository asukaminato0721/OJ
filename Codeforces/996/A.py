n = int(input())
sum_ = 0
for i in [100, 20, 10, 5, 1]:
    cnt, n = divmod(n, i)
    sum_ += cnt
print(sum_)
