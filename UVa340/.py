from collections import Counter, defaultdict
from itertools import count
for _ in count(1):
    n = input()
    if n == '0':
        break
    print("Game %d:" % (_))
    right = input().split()
    right_count = Counter(right)
    while True:
        guess = input().split()
        if guess[0] == '0':
            break
        strong, weak = defaultdict(int), defaultdict(int)
        guess_count = Counter(guess)
        for i, j in zip(guess, right):
            strong[i] += int(i == j)
        for i in guess:
            weak[i] = min(right_count[i], guess_count[i])-strong[i]
        print("    (%d,%d)" % (sum(strong.values()), sum(weak.values())))
