// #[derive(Debug)]
// enum Something {
//     Name,
//     Coords { x: i32, y: i32 },
//     Str(String),
//     Color(u8, u8, u8),
// }
//
// impl Something {
//     fn call(&self) {
//         println!("Hello world!");
//     }
// }

enum Daru {
    MagicMoments,
    KhukuriRum,
    Ruslan,
    Arna,
    Kingfisher,
    JackDenials,
}

fn main() {
    // let sth = Something::Str(String::from("hello "));
    //
    // sth.call();
    // println!("{:#?}", sth);

    // let some_number = Some(5);
    // let some_string = Some(String::from("Sargam"));
    //
    // let none: Option<i32> = None;
    //

    let drink = Daru::KhukuriRum;

    println!("The price of KhukuriRum is: {}", get_daru_amount(drink))
}

fn get_daru_amount(alcohol: Daru) -> u32 {
    match alcohol {
        Daru::MagicMoments => 1_600,
        Daru::JackDenials => 7_700,
        Daru::Arna => 120,
        Daru::Ruslan => 1_200,
        Daru::KhukuriRum => 2000,
        Daru::Kingfisher => 165,
    }
}
