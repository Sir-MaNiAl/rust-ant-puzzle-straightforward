use std::collections::HashSet;

fn main() {
    let start_point: (i64, i64) = (1000, 1000);
    let max_digits_sum: u8 = 25;
    let mut seen = HashSet::new();
    let mut count: u128 = 0;
    let mut stack: Vec<(i64, i64)> = vec![start_point];

    while let Some((x, y)) = stack.pop() {
        if !seen.insert((x, y)) {
            continue;
        }
        if sum_digits(x) + sum_digits(y) > max_digits_sum {
            continue;
        }
        stack.push((x + 1, y));
        stack.push((x, y + 1));
        stack.push((x - 1, y));
        stack.push((x, y - 1));
        count += 1;
    }
    println!("{}", count);
}

fn sum_digits(num: i64) -> u8 {
    let mut sum: u8 = 0;
    let mut n: u128 = (num as i128).abs() as u128;
    while n != 0 {
        sum += (n % 10) as u8;
        n /= 10;
    }
    sum
}
