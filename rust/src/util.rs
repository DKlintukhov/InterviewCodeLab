use std::io;

pub fn get_matrix_from_input() -> Result<Vec<Vec<u32>>, &'static str> {
    let mut matrix_size = String::new();
    io::stdin()
        .read_line(&mut matrix_size)
        .expect("Failed to read line");
    let matrix_size: Vec<usize> = matrix_size
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse matrix size"))
        .collect();

    let rows = matrix_size[0];
    let cols = matrix_size[1];

    let mut matrix: Vec<Vec<u32>> = Vec::with_capacity(rows);

    for _ in 0..rows {
        let mut row_line = String::new();
        io::stdin()
            .read_line(&mut row_line)
            .expect("Failed to read line");

        let row_values: Vec<u32> = row_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse unsigned integer"))
            .collect();

        if row_values.len() != cols {
            panic!("Incorrect number of columns in row");
        }

        matrix.push(row_values);
    }
    println!("{:?}", matrix);
    Ok(matrix)
}
