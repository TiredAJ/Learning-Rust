//chapter 6!

//embedding data
/*enum IPAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IPAddrKind::V4(127, 0, 0, 1);

    let loopback = IPAddrKind::V6(String::from("::1"));

}*/

//enum methods
/*fn main(){
    let m = Message::Write(String::from("Hello!"));

    m.call();
}

enum Message{
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message{
    fn call(&self){
        //not sure how to access the contents of an enum, but this \/ doesn't work
        //println!("message: {}", Write.0);
    }
}*/

//us money example
/*#[derive(Debug)] //to get the internals out with the println!()
enum USState{
    Alabama,
    Alaska,
    //--snip--
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn main(){
    value_in_cents(Coin::Quarter(USState::Alaska));
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}*/

//Matching with Option<T>
/*fn main(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}*/

//game example & catch-all patterns
/*fn main(){
    let dice_roll = 9;

    match dice_roll{
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other),  //other catches any other value and 
                                        //passes it to move_player()
        //_ => reroll(),    //_ ignores what the other value
        _ => (),    //unit value () means nothing happens
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num spaces: u8) {}
fn reroll() {}*/

//if let
fn main(){
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}