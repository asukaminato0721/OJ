from queue import PriorityQueue
word = set()
res = PriorityQueue()
while True:
    try:
        word.add(input())
    except EOFError:
        break
for i in word:
    for j in range(1, len(i)):
        if (i[:j] in word) and (i[j:] in word):
            res.put(i)
            break
while not res.empty():
    print(res.get(0))
