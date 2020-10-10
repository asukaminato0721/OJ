s = "`1234567890-=QWERTYUIOP[]\\ASDFGHJKL;'ZXCVBNM,./"

dic = dict(zip(s[1:], s))
while True:
    try:
        print(''.join(dic[i] if i in dic else i for i in input()))
    except EOFError:
        break
