fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let mut n: u32 = 0;
    let mut result: u32 = 0;

    while fibonacci(n) < 4000000 {
        n = n + 1;
        
        if fibonacci(n) % 2 == 0 {
            result = fibonacci(n) + result;
            println!("{}", fibonacci(n));
        }
    }
    println!("Here is the final result {}", result);
}