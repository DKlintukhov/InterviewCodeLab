#include "pch.h"

#include "cheapest_path.h"
#include <cassert>

namespace cheapest_path
{
    uint32_t get_cheapest_path(const std::vector<std::vector<uint32_t>>& matrix)
    {
        if (matrix.empty())
        {
            return 0;
        }

        size_t rows = matrix.size();
        size_t cols = matrix.front().size();

        std::vector<std::vector<uint32_t>> dp(rows, std::vector<uint32_t>(cols));
        dp[0][0] = matrix[0][0];

        for (size_t i = 1; i < rows; ++i)
        {
            dp[i][0] = dp[i - 1][0] + matrix[i][0];
        }

        for (size_t i = 1; i < cols; ++i)
        {
            dp[0][i] = dp[0][i - 1] + matrix[0][i];
        }

        for (size_t i = 1; i < rows; ++i)
        {
            for (size_t j = 1; j < cols; ++j)
            {
                dp[i][j] = std::min(dp[i - 1][j], dp[i][j - 1]) + matrix[i][j];
            }
        }


        return dp[rows - 1][cols - 1];
    }

    void run_tests()
    {
        {
            const std::vector<std::vector<uint32_t>> matrix;
            assert(get_cheapest_path(matrix) == 0);
        }

        {
            const std::vector<std::vector<uint32_t>> matrix{ {
                { 1, 1 },
                { 2, 1 }
            } };
            assert(get_cheapest_path(matrix) == 3);
        }

        {
            const std::vector<std::vector<uint32_t>> matrix{ {
                { 1, 3, 1 },
                { 2, 3, 1 }
            } };
            assert(get_cheapest_path(matrix) == 6);
        }

        {
            const std::vector<std::vector<uint32_t>> matrix{ {
                { 1, 1, 1, 1, 1 },
                { 3, 100, 100, 100, 100 },
                { 1, 1, 1, 1, 1 },
                { 2, 2, 2, 2, 1 },
                { 1, 1, 1, 1, 1 },
            } };
            assert(get_cheapest_path(matrix) == 11);
        }
    }
}
