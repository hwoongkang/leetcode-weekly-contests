mod weeks;
type Solution = weeks::Week329;
fn main() {
    println!("{}", Solution::alternate_digit_sum(521));
    println!(
        "{:?}",
        Solution::sort_the_students(
            vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
            2
        )
    );
    println!(
        "{}",
        Solution::make_strings_equal("11".to_string(), "00".to_string())
    );

    println!("{}", Solution::min_cost(vec![1, 2, 1, 2, 1], 5));
}
