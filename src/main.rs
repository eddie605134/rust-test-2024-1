fn main() {
    // draw(9527);

    let mut test_vec = vec![1, 2, 3, 4, 5];
    test_vec.push(6);

    println!("test_vec: {:?}", test_vec);

    let mut numbers = vec![1, 2, 3];
    println!("{}, {}", numbers.len(), numbers.capacity()); // 3, 3

    numbers.push(1);
    println!("{}, {}", numbers.len(), numbers.capacity()); // 4, 6

    numbers.push(1);
    numbers.push(1);
    println!("{}, {}", numbers.len(), numbers.capacity()); // 6, 6

    numbers.push(1);
    println!("{}, {}", numbers.len(), numbers.capacity()); // 7, 12
}

fn show_lotteries(n1: i32, n2: i32, n3: i32) {
    println!("the lottery numbers are {} {} {}", n1, n2, n3);
}

fn draw(num: i32) {
    show_lotteries(num + 1, num + 5, num + 10);
}
