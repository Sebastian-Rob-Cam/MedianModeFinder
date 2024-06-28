/*
    Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

fn median(user_vec: &mut Vec<i32>) -> f64 {
    user_vec.sort();
    let len = user_vec.len();
    let middle = len / 2;

    if len % 2 == 0 {
        (user_vec[middle - 1] as f64 + user_vec[middle] as f64) / 2.0
    } else {
        user_vec[middle] as f64
    }
}

fn mode(user_vec: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in user_vec.iter() {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap()
}

fn main() {
    let test_vec = vec![2, 4, 1, 3, 5, 2, 2];

    println!("{}", mode(&test_vec));
}
