use std::env;

fn is_even(n: u32) -> bool {
    let squares: Vec<i32> = (1..=n).map(|x| x * x).collect();
    for i in 1..n {
        let diff = squares[i as usize] - squares[(i - 1) as usize];
        if diff == n {
            return false;
        }
        if diff > n {
            return true;
        }
    }
    return true;
}

fn main() {
    let value = args[1].parse::<u32>().unwrap();
    println!(
        "{} is {}",
        value,
        if is_even(value) { "even" } else { "odd" }
    );
}
