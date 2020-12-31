while True:
    try:
        s, t = input().split()
        for i in t:
            if s[0] == i:
                s = s[1:]
            if len(s) == 0:
                print('Yes')
                break
        else:
            print('No')
    except EOFError:
        break
