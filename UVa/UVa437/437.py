from itertools import permutations, count


def LIS(blocks: list):
    dp = [0] * len(blocks)  # dp[i]:前 i 个中累计最高
    for i, block in enumerate(blocks):
        dp[i] = block[-1]
        for j in range(i):
            if (
                block[0] > blocks[j][0]
                and block[1] > blocks[j][1]
                and dp[j] + block[-1] > dp[i]
            ):
                dp[i] = dp[j] + block[-1]
    return max(dp)


for case in count(1):
    blocks = []
    n = int(input())
    if n == 0:
        break
    for _ in range(n):
        blocks.extend(permutations(map(int, input().split())))
    blocks.sort()
    print(f"Case {case}: maximum height = {LIS(blocks)}")
