from queue import PriorityQueue

li = PriorityQueue()
li.put(1)
isin = {1}
for _ in range(1500):
    tmp = li.get()
    for i in (tmp * 2, tmp * 3, tmp * 5):
        if i not in isin:
            li.put(i)
            isin.add(i)
print(f"The 1500'th ugly number is {sorted(isin)[1499]}.")
