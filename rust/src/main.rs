mod util;
mod cheapest_path;
use cheapest_path::get_cheapest_path;
use util::get_matrix_from_input;

fn main() {
    let matrix = get_matrix_from_input().unwrap();
    let result = get_cheapest_path(matrix);
    println!("{}", result);
}
