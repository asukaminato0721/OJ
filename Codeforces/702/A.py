def foo():
    return map(int, input().split())


input()
nums = list(foo())
current = nums[0]
currentlen, max_ = 1, 1
for i in nums[1:]:
    if i > current:
        currentlen += 1
        max_ = max(max_, currentlen)
    else:
        currentlen = 1
    current = i
print(max_)
