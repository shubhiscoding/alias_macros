use alias_macros::define;

define!(MyInt = i64);
define!(str = String);
define!(char = String);

fn main() {
    let v: MyInt!() = 100;
    let str: str!() = <str!()>::from("proof");
    let char: char!() = <char!()>::from("I'm a String not a char");

    println!("--------------------------------------------------------");
    println!("I'm called MyInt, and I'm a i64, Check this out: {}", v);
    println!("");
    println!("My name is str and I'm a String, here's a proof: {}", str);
    println!("");
    println!("People call me char, but secretly I'm a String: {}", char);
    println!("--------------------------------------------------------");
}
