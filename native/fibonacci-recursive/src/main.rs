use std::time::{ Instant};

fn main() {
    // n is the input value
    let n = 20;

    // start measuring time
    let start = Instant::now();

    // call function to get the nth fibonacci number
    let result = fibonacci_recursive(n);

    // stop the time measuring
    let duration = start.elapsed();

    println!("Time: {:?} \nResult {}",duration, result);
}

// simple recursive function to get the nth number of the fibonacci sequence
pub fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
}