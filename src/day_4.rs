fn day_4() {
    // u8 -> 型別; 3 -> 長度
    let list: [u8; 3] = [1, 2, 3];

    println!("{:?}", list);

    // let list2 = [2, "3", 4];

    let list3 = [1450, 9527, 5566];

    for item in list3.iter() {
        println!("{}", item);
    }

    let list4 = [1450, 9527, 5566];
    let [_, b, c] = list4;

    // tuple
    let answer: (char, bool) = ('蛤', false);
    let pet = ('🐈', false);

    let point = (100, 200, 300);
    let (x, y, z) = point;

    // Unit () 空tuple; 類似 void
    // 表示「這個函數是沒有回傳值」

    // fn main() -> () {
    //     println!("Hello Rust")
    // }
}
