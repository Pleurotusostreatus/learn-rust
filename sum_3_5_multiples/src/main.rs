// use std::vec;

fn main() {
    let mut n: u32 = 0;
 //   let mut vector: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;

    while n < 999 {
        n = n + 1;
        
        if n % 3 == 0 {
           // vector.push(n);
            sum = sum + n;
            println!("{}", n);

        }
        else if n % 5 == 0 {
           // vector.push(n);
            sum = sum + n;
            println!("{}", n);
        }
    }
    // let sum: u32 = vector.iter().sum();
    println!("Here are the results {}", sum);
}
