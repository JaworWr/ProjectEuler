use crate::utils::LowFactors;
use itertools::Itertools;

fn pascal(n_rows: usize) -> Vec<usize> {
    let mut result = vec![1, 1, 1];
    let mut row = vec![1, 1];
    for _i in 2..n_rows {
        let new_elems = row
            .into_iter()
            .tuple_windows()
            .map(|(a, b)| a + b);
        row = vec![1];
        row.extend(new_elems);
        row.push(1);
        result.extend(row.iter());
    }
    result
}

fn is_square_free(x: usize) -> bool {
    LowFactors::of(x)
        .skip(1)
        .all(|d| x % (d * d) != 0)
}

pub fn solve() {
    let result = pascal(51)
        .into_iter()
        .unique()
        .filter(|&x| is_square_free(x))
        .sum::<usize>();
    println!("{}", result);
}