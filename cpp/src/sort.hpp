#include <vector>
#include <concepts>
#include <type_traits>
#include <cassert>

namespace sort
{
    template <typename T>
    concept TriviallyThreeWayComparable = std::three_way_comparable<T> && std::is_trivially_copyable_v<T>;

    template <TriviallyThreeWayComparable T>
    constexpr std::vector<T> bubble_sort(const std::vector<T>& arr)
    {
        if (arr.empty())
        {
            return {};
        }

        auto sorted = arr;

        for (size_t i = 0; i < arr.size(); ++i)
        {
            for (size_t j = 0; j < arr.size() - 1; ++j)
            {
                if (sorted[i] < sorted[j])
                {
                    std::swap(sorted[i], sorted[j]);
                }
            }
        }


        return sorted;
    }

    void run_tests()
    {
        {
            std::vector<int> arr;
            assert(bubble_sort(arr).size() == 0);
        }

        {
            std::vector<int> arr{ 1, 2, 2, 1 };
            auto res = bubble_sort(arr);
            assert(res.size() == 4);
            assert(res[0] == 1);
            assert(res[1] == 1);
            assert(res[2] == 2);
            assert(res[3] == 2);
        }

        {
            std::vector<int> arr{ -1, -2, -2, -1 };
            auto res = bubble_sort(arr);
            assert(res.size() == 4);
            assert(res[0] == -2);
            assert(res[1] == -2);
            assert(res[2] == -1);
            assert(res[3] == -1);
        }
    }
}
