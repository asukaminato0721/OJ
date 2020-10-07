cnt = 0
sol = set()
while True:
    n = int(input())
    if n == 0:
        break
    if cnt > 0:
        print()
    cnt += 1
    有解 = False
    for i in range(1234, 10_0000//n+1):
        sol.clear()
        sol.update(str(i).zfill(5), str(i*n))
        if len(sol) == 10:
            print('%d / %05d = %d' % (i*n, i, n))
            有解 = True
    if not 有解:
        print('There are no solutions for %d.' % (n))
