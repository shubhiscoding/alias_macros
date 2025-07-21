use alias_macros::define;

define!(MyInt = i64);
define!(ltr = char);
define!(str = String);

fn main() {
    let v: MyInt!() = 100;
    let str: ltr!() = 'n';
    let strg: str!() = String::from("Shubh");
    println!("Hello, world!, {:?}, +, {}, and the string {}", v, str, strg);
}
