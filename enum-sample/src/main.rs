#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    {
        let coin1 = Coin::Penny;
        let coin2 = Coin::Quarter(UsState::Alabama);
        value_in_cents(coin1);
        value_in_cents(coin2);
    }
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        // 以下のように、Someは Option::Someの省略形である
        let five = Option::Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    }
    {
        let some_value = Some(0u8);
        if let Some(3) = some_value {
            println!("three");
        }
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
