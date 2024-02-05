trait Flyable {
    fn fly(&self);
}
trait FlyableAndSay {
    fn fly_and_say(&self) {
        println!(
            "你好，我是一隻會飛的動物！ 名叫: {}，年齡: {}",
            self.name(),
            self.age()
        );
    }

    fn name(&self) -> String;
    fn age(&self) -> u8;
}

struct Cat {
    name: String,
    age: u8,
}

struct Dog {
    name: String,
}

impl Flyable for Cat {
    fn fly(&self) {
        println!("嘿，我是 {}，你看我會飛，你不會！", self.name);
    }
    // fn hey(&self) {
    //     println!("How you doing")
    // }
}

impl FlyableAndSay for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn age(&self) -> u8 {
        self.age
    }
}

fn day_12() {
    let kitty = Cat {
        name: String::from("Kitty"),
        age: 18,
    };
    let nancy = Cat {
        name: String::from("Nancy"),
        age: 12,
    };
    kitty.fly(); // 印出 嘿，我是 Kitty，你看我會飛，你不會！
    kitty.fly_and_say(); // 印出 嘿，我是 Kitty，你看我會飛，你不會！

    bungee(&kitty); // 印出 飛呀~飛呀~小飛俠！
    bungee(&nancy); // 印出 飛呀~飛呀~小飛俠！

    let lucky = Dog {
        name: String::from("Lucky"),
    }; // 沒有實作 Flyable
       // bungee(&lucky); // 編譯錯誤
}

fn bungee(someone: &dyn Flyable) {
    someone.fly();
}
