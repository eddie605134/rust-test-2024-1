fn main() {
    let lost_numbers = vec![4, 8, 15, 16, 23, 42];
    let first_two_nums = &lost_numbers[..2]; // 前 2 個
    let last_three_nums = &lost_numbers[lost_numbers.len() - 3..]; // 後 3 個

    println!("{:?}", first_two_nums); // 印出 [4, 8]
    println!("{:?}", last_three_nums); // 印出 [16, 23, 42]

    let numbers = &lost_numbers[..];
    println!("{:?}", numbers); // 印出 [4, 8, 15, 16, 23, 42]

    let mut lost_numbers = vec![4, 8, 15, 16, 23, 42];
    let nums = &mut lost_numbers[0..3];

    nums[0] = 5566;
    println!("{:?}", lost_numbers); // 印出 [5566, 8, 15, 16, 23, 42]

    let book = "為你自己學 Rust";
    publish_book(book);

    let book2: String = String::from("為你自己學 Rust");
    println!("{}", book2);
}

fn publish_book(book: &str) {
    println!("{:?} 要上市囉！", book)
}
