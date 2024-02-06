#[allow(dead_code)]
// enum CatBreed {
//     Persian,           // 波斯貓
//     AmericanShorthair, // 美國短毛貓
//     Mix,               // 米克斯
// }

// enum CatBreed {
//     Persian,           // 波斯貓
//     AmericanShorthair, // 美國短毛貓
//     Mix(String, u8),   // 米克斯
// }

struct Skill {
    action: String,
}

enum CatBreed {
    Persian,              // 波斯貓
    AmericanShorthair,    // 美國短毛貓
    Mix(String, u8),      // 米克斯
    Other(Skill),         // 其它
    Alien { power: u32 }, // 外星貓
}

impl CatBreed {
    fn go(&self) {
        println!("Go!");
    }
}

fn day_13() {
    let breed = CatBreed::Persian;

    // match breed {
    //     CatBreed::Mix(name, age) => println!("我是米克斯{}, 今年 {} 歲", name, age),
    //     _ => println!("我是品種貓"),
    // }

    // match breed {
    //     CatBreed::Persian => println!("我是波斯貓"),
    //     CatBreed::AmericanShorthair => println!("我是美國短毛貓"),
    //     CatBreed::Mix => println!("我是米克斯"),
    // }

    let kitty = CatBreed::Mix(String::from("Kitty"), 8);
    let nancy = CatBreed::Persian;

    greeting(&kitty);
    greeting(&nancy);

    let goku_cat = CatBreed::Other(Skill {
        action: "龜派氣功".to_string(),
    });
    let frieza_cat = CatBreed::Alien { power: 530000 }; // 戰鬥力 53 萬

    greeting(&goku_cat);
    greeting(&frieza_cat);
}

fn greeting(cat: &CatBreed) {
    match cat {
        CatBreed::Mix(name, age) => println!("我是米克斯，我叫 {}，我今年 {} 歲", name, age),
        CatBreed::Other(skill) => println!("使出絕招{}！", skill.action),
        CatBreed::Alien { power } => println!("我的戰鬥力是 {}", power),
        _ => println!("我是品種貓"),
    }
}

// Enum vs Struct
// Rust 中的 Enum 和 Struct
// 確實有些相似之處，但它們也有一些不一樣的地方，使用情境也不太一樣：

// 相同:
//
// 都可以用來產生實體或傳進函數裡。
// 都可以用 impl 幫自己增加功能，Trait 也都能用。
// 都可以配合 match 一起使用。

// 不同:
//
// Enum 可以有很多的變體（Variant），每個變體都還能另外接不同型態的參數；Struct 的可以有很多欄位（Field），不過就沒辦法像變體這麼多變化了。
// 雖然 Enum 跟 Struct 都可以跟 match 搭配著使用，但 Enum 的話會檢查是不是每個變體的情況都有考慮到了，Sturct 就沒這設計。
// Enum 裡面可以包 Struct，但 Struct 裡面不能包 Enum。

// 使用時機
// 需要匹配多種情況時，用 Enum 會比較方便，因為 Enum 可以有很多變體，每個變體都可以接不同型態的參數，這樣就可以用 match 來處理很多不同的情況。
// 如果只是單純的資料結構，用 Struct 就好了，Enum 會讓程式碼變得比較複雜。
