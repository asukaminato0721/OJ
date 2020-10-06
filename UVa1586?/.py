import re
mass = {'C': 12.01, 'H': 1.008, 'O': 16.00, 'N': 14.01}
n = int(input())
for i in range(n):
    s = input()
    if s[-1] not in '1234567890':
        s += '1'
    s = re.split('(\d+)', s)
    print(
        '%.3f' % (sum(sum(mass[k] for k in i[:-1])+mass[i[-1]]*int(j)
                      for i, j in zip(s[::2], s[1::2]))))
