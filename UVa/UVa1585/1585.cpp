#include <bits/stdc++.h>
using namespace std;
#define all(x) (x).begin(), (x).end()
string tmp, line;
int main()
{
    int T;
    cin >> T;
    while (T--)
    {
        cin >> line;
        replace(all(line), 'X', ' ');
        stringstream ss(line);
        int s = 0;
        while (ss >> tmp)
        {
            s += (tmp.length() * (tmp.length() + 1) / 2);
        }
        cout << s << endl;
    }
    return 0;
}
