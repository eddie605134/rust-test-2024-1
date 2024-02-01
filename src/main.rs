fn main() {
    print_number(5566, 7788);
    let test = add_extra(1, 2);

    println!("test: {}", test);
    test_if_statement();
}

fn print_number(a: i32, b: i32) -> i32 {
    return a + b;
    // println!("{}", n);
}

fn add_extra(a: i32, b: i32) -> i32 {
    let extra = 100;
    // a + b + extra
    return a + b + extra;
}

fn test_if_statement() -> () {
    let age = 20;

    let message = if age < 8 {
        "小朋友"
    } else if age >= 8 && age < 18 {
        "年輕人"
    } else {
        "成年人"
    };

    println!("{}", message);
}
