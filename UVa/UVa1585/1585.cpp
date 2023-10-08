#include <algorithm>
#include <cstdint>
#include <iostream>
#include <sstream>
#define all(x) (x).begin(), (x).end()
std::string tmp, line;
int main() {
        int T;
        std::cin >> T;
        while (T--) {
                std::cin >> line;
                std::replace(all(line), 'X', ' ');
                auto ss = std::stringstream{line};
                auto s = 0;
                while (ss >> tmp) {
                        s += (tmp.length() * (tmp.length() + 1) / 2);
                }
                std::cout << s << std::endl;
        }
        return 0;
}
