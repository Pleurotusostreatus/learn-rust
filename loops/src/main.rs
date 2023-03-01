fn main() {
    // ----- a nicer way to for loop
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!!");

    // ----- for loop
    // let a = [10, 2, 3, 4, 484];

    // for element in  a {
    //     println!{"This number is {element}"};
    // }

    //-----  while loop 
    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");
    //----- loop break on outer loop
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");
    //------ loop with if and break
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");
}