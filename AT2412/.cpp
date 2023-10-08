#include <cstdint>
#include <iostream>
#include <numeric>
#include <vector>

int main() {
        auto a = std::vector<int>{};
        std::int32_t n, k;
        std::cin >> n >> k;
        for (std::int32_t ai; n; std::scanf("%d", &ai), a.push_back(ai), n--)
                ;
        auto b = std::vector<int>{};
        std::partial_sum(a.cbegin(), a.cend(), std::back_inserter(b));
        auto maxs = *(b.cbegin() + k - 1);
        for (auto i = b.cbegin(); i != b.cend() - k; i++)
                maxs = std::max(maxs, *(i + k) - *i);
        std::cout << maxs << std::endl;
        return 0;
}
