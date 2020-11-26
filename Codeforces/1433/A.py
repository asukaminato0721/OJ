for _ in range(int(input())):
    input_ = input()
    len_, num_ = len(input_), int(input_[0])
    print((num_ - 1) * (1 + 2 + 3 + 4) + sum((1, 2, 3, 4)[:len_]))
