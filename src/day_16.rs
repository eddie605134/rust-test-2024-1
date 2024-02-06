fn day_16() {
    match bmi_calculator(170, 70.5) {
        Ok(result) => handle_ok(result),
        Err(reason) => handle_err(reason),
    }

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("{:?}", person); // 使用 `{:?}` 來打印 Debug 格式

    hello();
}

fn bmi_calculator(height_cm: u32, weight_kg: f32) -> Result<f32, String> {
    if height_cm == 0 {
        return Err("身高不能為零。".to_string());
    }

    let height_m = height_cm as f32 / 100.0;
    let bmi = weight_kg / (height_m * height_m);

    Ok(bmi)
}

fn handle_ok(result: f32) {
    println!("{:.2}", result);
}

fn handle_err(reason: String) {
    println!("{}", reason);
}

fn hello() {
    world();
}

fn world() {
    hey();
}

fn hey() {
    panic!("😱😱😱😱😱😱😱"); // 在這裡引爆
}
