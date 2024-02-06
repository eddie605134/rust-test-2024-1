#[derive(Debug)]
#[allow(dead_code)]
struct Cat {
    name: String,
    age: u8,
}

fn day_17() {
    let kitty = Cat {
        name: String::from("Kitty"),
        age: 18,
    };
    println!("{:?}", kitty);
}
