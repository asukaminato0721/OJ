#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
string line;
int main()
{
    while (getline(cin, line))
    {
        string solution;
        line = ' ' + line;
        for (auto i = line.begin(); i != line.end(); i++)
        {
            bool flag = false;
            if (*i == ' ')
            {
                flag = true;
                solution += ' ';
                i++;
                solution += toupper(*i);
                flag = false;
            }
            else
            {
                solution += *i;
            }
        }
        cout << solution.substr(1) << endl;
    }
    return 0;
}
