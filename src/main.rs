fn main() {
    #[allow(dead_code)]
    struct Cat {
        name: String,
        age: u8,
        is_sleeping: bool,
    }
    // 定義方法
    impl Cat {
        fn greeting(&self) {
            println!("Hello, my name is {}", self.name);
        }

        fn set_age(&mut self, age: u8) {
            self.age = age;
        }

        // 沒有給 self 的方法，則不會綁再實例上
        fn run() {
            println!("Go Go Power Rangers");
        }

        // 需要帶參數的類別方法
        fn count(list: &[u8]) -> u8 {
            list.iter().sum()
        }
    }

    // let kitty = Cat {
    //     name: String::from("Kitty"),
    //     age: 12,
    //     is_sleeping: true,
    // };

    let name = String::from("Kitty");
    let age = 12;
    let is_sleeping = true;

    let kitty = Cat {
        name,
        age,
        is_sleeping,
    };

    println!("{}", kitty.name);
    println!("{}", kitty.age);
    println!("{}", kitty.is_sleeping);

    kitty.greeting();
    Cat::run();
    let list: [u8; 5] = [1, 2, 3, 4, 5];
    let sum = Cat::count(&list);
    println!("貓米有{}隻", sum);
}
