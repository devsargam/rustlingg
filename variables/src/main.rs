fn main() {
    let x = 5;
    const PI: f32 = 3.14;
    println!("The value of x: {}", x);
    let x = x + 10;
    println!("The value of x: {}", x);
    println!("The value of pi: {}", PI);

    {
        let x = x + 10;
        println!("The value of x in inner scope: {}", x);
    }

    println!("The value of x in outer scope: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("{}", spaces);

    // Data types
    // Tuples
    let tup: (i32, f64, u8) = (500, 10.00, 20);
    let (x, y, _) = tup;
    println!("{}, {}, {}", x, y, tup.2);

    let _ = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{}", next(next(0)));
}

fn next(x: i32) -> i32 {
    x + 1
}
