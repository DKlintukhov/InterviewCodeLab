#include "pch.h"
#include "util.h"

namespace util
{
    std::vector<std::vector<uint32_t>> get_matrix_from_input()
    {
        size_t rows, cols;
        if (!(std::cin >> rows >> cols))
        {
            throw std::runtime_error("Failed to read matrix size");
        }

        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        std::vector<std::vector<uint32_t>> matrix;
        matrix.reserve(rows);

        while (rows--)
        {
            std::string row_line;
            if (!std::getline(std::cin, row_line))
            {
                throw std::runtime_error("Failed to read line");
            }

            std::stringstream ss(row_line);
            std::vector<uint32_t> row;
            row.reserve(cols);
            size_t value;
            while (ss >> value)
            {
                row.push_back(value);
            }

            if (row.size() != cols)
            {
                throw std::runtime_error("Incorrect number of columns in row");
            }

            matrix.push_back(std::move(row));
        }

        return matrix;
    }
}
