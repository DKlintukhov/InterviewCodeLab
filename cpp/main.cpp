#include "pch.h"

#include "util.h"
#include "cheapest_path.h"
#include "sort.hpp"

int main(int, char const**)
{
    cheapest_path::run_tests();
    sort::run_tests();
    return 0;
}
