use std::ops::{Add, Sub};

struct Rectangle<T> {
    width: T,
    height: T,
}

fn day_15() {
    let rect_a = Rectangle {
        width: 100,
        height: 50,
    };
    let rect_b = Rectangle {
        width: 38.5,
        height: 19.5,
    };

    let result = calc(1, 8, 2);

    println!("運算結果是{}", result);
}

// fn calc<T: Add<Output = T> + Sub<Output = T>>(a: T, b: T, c: T) -> T {
//     a + b - c
// }

fn calc<T>(a: T, b: T, c: T) -> T
where
    T: Add<Output = T> + Sub<Output = T>,
{
    a + b - c
}

trait Flyable {
    fn fly(&self);
}

// 動態分發 &dyn Trait
fn bungee(someone: &dyn Flyable) {
    someone.fly();
}

// 可以用泛型實現
// fn bungee<T: Flyable>(someone: &T) {
//     someone.fly();
// }
