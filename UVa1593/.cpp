#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
vector<vector<string>> input;
vector<string> each;
size_t length[1010] = {0};
string line, word;
int main()
{
    while (getline(cin, line))
    {
        stringstream ss(line);
        each.clear();
        while (ss >> word)
        {
            each.push_back(word);
        }
        input.push_back(each);
    }
    for (auto &&i : input) //然后统计出每列 string 的最大宽度
    {
        for (size_t j = 0; j < i.size(); j++)
        {
            length[j] = max(length[j], i[j].size());
        }
    }
    for (auto &&i : input) // 根据每列 max 输出
    {
        for (size_t j = 0; j < i.size() - 1; j++)
        {
            cout << i[j] << string(length[j] - i[j].size() + 1, ' ');
        }
        cout << i.back() << "\n";
    }
    return 0;
}
