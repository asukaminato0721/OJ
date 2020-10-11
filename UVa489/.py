while True:
    n = input()
    if n == '-1':
        break
    print('Round', n)
    right, guess, wrong, has_guess = input(), input(), 0, set()
    for i in guess:
        if i in right:
            right = right.replace(i, '')
            has_guess.add(i)
        else:
            wrong += 1
            has_guess.add(i)
        if right != '' and wrong > 6:
            print('You lose.')
            break
        if right == '' and wrong <= 6:
            print('You win.')
            break
    else:
        print('You chickened out.')
