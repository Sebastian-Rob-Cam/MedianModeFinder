/*
    Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

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

fn main() {
    let mut test_vec = vec![2, 4, 1, 3, 5];

    println!("{}", median(&mut test_vec))
}
