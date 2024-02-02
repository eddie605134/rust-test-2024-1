// fn main() {
//     let mut scores = get_scores();
//     let total_score = calc_score(&mut scores);
//     // let total_score = calc_score(scores.clone());

//     println!("{:?}", total_score);
//     println!("{:?}", scores);
// }

// fn get_scores() -> Vec<i32> {
//     let scores = vec![1450, 9527, 5566];
//     return scores;
// }

// fn calc_score(scores: &mut Vec<i32>) -> i32 {
//     scores.push(123); // 加料！
//     let mut total = 0;

//     for score in scores.iter() {
//         total += score;
//     }

//     return total;
// }

// fn main() {
//     let mut scores = vec![1450, 9527, 5566];
//     let total_score = calc_score(&mut scores);

//     print!("total_score: {}", total_score);
// }

// fn calc_score(scores: &mut Vec<i32>) -> i32 {
//     scores.push(123); // 加料
//     let mut total = 0;

//     for score in scores.iter() {
//         total += score;
//     }
//     return total;
// }

fn main() {
    let book = String::from("為你自己學 Rust");

    // let b1 = &mut book; // 如果是 &mut book 只能有一個
    // let b2 = &mut book;
    // let b3 = &mut book;

    let b1 = &book; // 一般的 borrow 可以有多個
    let b2 = &book;
    let b3 = &book;

    println!("{:?}, {:?}, {:?}", b1, b2, b3);
}
