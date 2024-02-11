fn main() {
    // let condition = false;
    //
    // let number = if condition { 5 } else { 7 };
    //
    // println!("The value of number: {}", number);
    //
    //

    // Loops in Rust
    // loop {
    //     println!("Looppp!!!");
    // }

    // // New concept of loop
    // let mut counter = 1;
    //
    // let result = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 2;
    //     };
    // };
    //
    // println!("The value of result: {result}")
    //
    //

    // For loop
    let arr = [1, 2, 3, 4, 5];

    for a in arr {
        println!("{a}");
    }

    // For looping in integers
    for num in (1..10).rev() {
        println!("{num}");
    }
}
