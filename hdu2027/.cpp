#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
string line;
int n;
int main()
{
    cin >> n;
    getchar();
    while (n--)
    {
        getline(cin, line);
        map<char, int> solution = {
            make_pair('a', 0),
            make_pair('e', 0),
            make_pair('i', 0),
            make_pair('o', 0),
            make_pair('u', 0)};
        for (auto &&i : line)
        {
            solution[i]++;
        }
        for (auto &&i : string("aeiou"))
        {
            cout << i << ":" << solution[i] << "\n";
        }
        if (n)
            cout << endl;
    }
    return 0;
}
