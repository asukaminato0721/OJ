#include <bits/stdc++.h>
using namespace std;
set<string> dict;
int main()
{
    for (string word; cin >> word; dict.insert(word))
        ;
    for (auto &&i : dict)
    {
        for (size_t j = 1; j < i.size(); j++)
        {
            if (dict.count(i.substr(0, j)) && dict.count(i.substr(j)))
            {
                cout << i << "\n";
                break;
            }
        }
    }
    return 0;
}
