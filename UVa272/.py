文章 = ''
while True:
    try:
        文章 += (input()+'\n')
    except EOFError:
        break
括号内 = False
for i in 文章:
    if i == '"' and 括号内:
        print('\'\'', end='')
        括号内 = not 括号内
    elif i == '"' and not 括号内:
        print('``', end='')
        括号内 = not 括号内
    else:
        print(i, end='')
