fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(val: Option<i32>) -> Option<i32> {
    match val {
        Some(i) => Some(i + 1),
        other => other,
    }
}
