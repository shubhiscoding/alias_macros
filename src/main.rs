use alias_macros::define;

define!(MyInt = i64);
define!(str = String);
define!(char = String);

define!(interface = #[derive(Debug)]struct);

interface!(Nothing {
    num: MyInt!()
});

#[derive(Debug)]
pub struct something {
    str: str!()
}

define!(strignStruct = something);


fn main() {
    let v: MyInt!() = 100;
    let char: char!() = <char!()>::from("I'm a String not a char");

    let s = Nothing {
        num: 1
    };

    let a = strignStruct!({
        str: <str!()>::from("Shubh")
    });

    println!("--------------------------------------------------------");
    println!("I'm a struct, but a TS nerd initialized me calling Interface, but I still work: {:?}", s);
    println!("");
    println!("A dumb guy created me with a different name and then aliased me to call strignStruct, anyways I'm: {:?}", a);
    println!("");
    println!("I'm called MyInt, and I'm a i64, Check this out: {}", v);
    println!("");
    println!("People call me char, but secretly I'm a String: {}", char);
    println!("--------------------------------------------------------");
}
