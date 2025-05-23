use std::cmp::PartialOrd;

fn bubble_sort<T: PartialOrd + Copy>(arr: &[T]) -> Vec<T> {
    if arr.is_empty() {
        return vec![];
    }

    let mut sorted: Vec<T> = arr.to_vec();

    for i in 0..sorted.len() {
        for j in 0..sorted.len() - 1 {
            if sorted[j] > sorted[j + 1] {
                sorted.swap(i, j+ 1);
            }
        }
    }

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_empty() {
        assert_eq!(bubble_sort::<i32>(&[]), &[]);
    }

    #[test]
    fn test_bubble_sort_1() {
        assert_eq!(bubble_sort::<i32>(&[1, 2, 2, 1]), vec![1, 1, 2, 2]);
    }

        #[test]
    fn test_bubble_sort_2() {
        assert_eq!(bubble_sort::<i32>(&[-1, -2, -2, -1]), vec![-2, -2, -1, -1]);
    }
}
