def foo():
    return map(int, input().split())


for _ in range(int(input())):
    input()
    nums = [x % 2 for x in foo()]
    if nums == [1]:
        print(-1)
        continue
    if 0 in nums:  # find even
        print(1)
        print(nums.index(0) + 1)
    else:  # find odd
        print(2)
        print(*[i for i, x in enumerate(nums, 1) if x == 1][:2])
