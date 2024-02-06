// Option => Some, None
// Result => Ok, Err

fn day_14() {
    let friends = get_friends(true);

    match &friends {
        None => println!("我是邊緣人我驕傲！"),
        Some(list) => println!("我有好多朋友 {:?}", list),
    }

    // println!("{:?}", friends.unwrap());

    if let Some(friends_list) = &friends {
        println!("有好多朋友 {:?}", friends_list);
    } else {
        println!("沒有朋友");
    }

    println!("{:?}", friends);

    println!("{}", friends.is_some());
    println!("{}", friends.is_none());

    println!("{:?}", friends.unwrap_or(vec![]));

    // Result
    match withdraw(100) {
        Ok(amount) => println!("提領金額 {} 元", amount),
        Err(message) => println!("提領失敗：{}", message),
    }
}

fn get_friends(has_money: bool) -> Option<Vec<u8>> {
    if !has_money {
        return None;
    }

    let friends: Vec<u8> = vec![1, 2, 3, 4, 5];
    Some(friends)
}

fn withdraw(amount: u32) -> Result<u32, String> {
    const BANK_BALANCE: u32 = 1000;
    // 判斷帳戶餘額
    if amount > BANK_BALANCE {
        return Err(String::from("餘額不足"));
    }

    Ok(amount)
}
