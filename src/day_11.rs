struct Cat {
    name: String,
    age: u8,
}

fn day_11() {
    let kitty = Cat {
        name: "Kitty".to_string(),
        age: 12,
    };
    let nancy = Cat {
        name: "Nancy".to_string(),
        age: 16,
    };

    let boss = boss_cat(&kitty, &nancy);
    println!("{}", boss.name);

    let total_age = calc_total_age(&kitty, &nancy);
    println!("{}", total_age);

    let mut mary = Cat {
        name: "Mary".to_string(),
        age: 18,
    };
    let new_mary = set_age(&mut mary, 20);
    println!("{}", new_mary.age); // 印出 20
}

fn boss_cat<'z>(c1: &'z Cat, c2: &'z Cat) -> &'z Cat {
    if c1.age > c2.age {
        c1
    } else {
        c2
    }
}

// 不需要寫生命週期的時機：
// 1. 函數裡的參數是 & 參照，但回傳值不是參照：
fn calc_total_age(c1: &Cat, c2: &Cat) -> u8 {
    c1.age + c2.age
}
// 2.只有其中一個參數是 &參照，而且回傳值也是 & 參照：
fn set_age(c: &mut Cat, new_age: u8) -> &Cat {
    c.age = new_age;
    c
}
