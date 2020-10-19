from sys import stdin
maxlength = [0]*183
input_text = [i.split() for i in stdin.readlines()]
for i in input_text:
    for ids, (word, maxl) in enumerate(zip(i, maxlength)):
        maxlength[ids] = max(len(word), maxl)
for i in input_text:
    for word, maxl in zip(i[:-1], maxlength):
        print(word, end=' '*(maxl + 1-len(word)))
    print(i[-1])
