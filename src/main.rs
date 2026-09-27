use std::io::{self, Write};

fn read_i32(prompt: &str) -> i32 {
    print!("{prompt} ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
fn is_even (n: i32) -> bool {
	if n%2 == 0 {
	   return true;
	}
	return false;
}

fn digit_sum(n: i32) -> i32 {
	let mut sum=0;
	let mut n = n;
	while n != 0 {
	     let last_digit = n % 10;
	     sum = sum + last_digit;
	     n = n / 10;
	}
	return sum;
}
fn main() {
    let start = read_i32("Start of range:");
    let end = read_i32("End of range:");
    for n in start..=end {
        println!(
            "{n}: even={}, digit_sum={}",
            is_even(n), digit_sum(n)
        );
    }
}
