line = input()
len_ = len(line)
print(sum(f"{line[i]}{line[j]}{line[k]}" == "QAQ" for i in range(len_ - 2) for j in range(i, len_ - 1) for k in range(j, len_)))
