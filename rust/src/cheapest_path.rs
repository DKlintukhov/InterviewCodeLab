/*
* В каждой клетке прямоугольной таблицы
* N × M
* N×M записано некоторое число. Изначально игрок находится в левой верхней клетке.
* За один ход ему разрешается перемещаться в соседнюю клетку либо вправо, либо вниз (влево и вверх перемещаться запрещено).
* При проходе через клетку с игрока берут столько килограммов еды, какое число записано в этой клетке
* (еду берут также за первую и последнюю клетки его пути).
* Требуется найти минимальный вес еды в килограммах, отдав которую игрок может попасть в правый нижний угол.
*
* Ограничения
* Ограничение времени
* 1 с
*
* Ограничение памяти
* 256 МБ
*/

pub fn get_cheapest_path(matrix: Vec<Vec<u32>>) -> u32 {
    if matrix.is_empty() || matrix.first().unwrap().is_empty() {
        return 0;
    }

    let rows = matrix.len();
    let cols = matrix.first().unwrap().len();

    let mut dp: Vec<Vec<u32>> = vec![vec![0; cols]; rows];
    dp[0][0] = matrix[0][0];

    for i in 1..rows {
        dp[i][0] = dp[i - 1][0] + matrix[i][0];
    }

    for i in 1..cols {
        dp[0][i] = dp[0][i - 1] + matrix[0][i];
    }

    for i in 1..rows {
        for j in 1..cols {
            dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i][j - 1]) + matrix[i][j];
        }
    }

    return dp[rows - 1][cols - 1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cheapest_path_empty_1() {
        let matrix: Vec<Vec<u32>> = vec![];
        assert_eq!(get_cheapest_path(matrix), 0);
    }

    #[test]
    fn test_get_cheapest_path_empty_2() {
        let matrix: Vec<Vec<u32>> = vec![vec![]];
        assert_eq!(get_cheapest_path(matrix), 0);
    }

    #[test]
    fn test_get_cheapest_path_1() {
        let matrix: Vec<Vec<u32>> = vec![vec![1, 1], vec![2, 1]];
        assert_eq!(get_cheapest_path(matrix), 3);
    }

    #[test]
    fn test_get_cheapest_path_2() {
        let matrix: Vec<Vec<u32>> = vec![vec![1, 3, 1], vec![2, 3, 1]];
        assert_eq!(get_cheapest_path(matrix), 6);
    }

    #[test]
    fn test_get_cheapest_path_3() {
        let matrix: Vec<Vec<u32>> = vec![
            vec![1, 1, 1, 1, 1],
            vec![3, 100, 100, 100, 100],
            vec![1, 1, 1, 1, 1],
            vec![2, 2, 2, 2, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(get_cheapest_path(matrix), 11);
    }
}
