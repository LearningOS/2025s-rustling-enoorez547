// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // let s = "blue";
    // let str = "red".to_string();
    // let str = String::from("hi");
    // let str = "rust is fun!".to_owned();
    // let str = "nice weather".into();
    // let str = format!("Interpolation {}", "Station");
    // let s = &String::from("abc")[0..1];
    // ("  hello there ".trim());
    // ("Happy Monday!".to_string().replace("Mon", "Tues"));
    // ("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
