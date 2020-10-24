from sys import stdin
from itertools import zip_longest

input_text = [i.split() for i in stdin.readlines()]
maxlength = [max(map(len, i)) for i in zip_longest(*input_text, fillvalue="")]
for i in input_text:
    for word, maxl in zip(i[:-1], maxlength):
        print(word, end=" " * (maxl + 1 - len(word)))
    print(i[-1])
