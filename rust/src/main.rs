fn foo() -> String {
    return "FUBAR!".to_string()
}
fn main() {
    println!("Hello, world!");
    let _bar = foo();
    println!("Bar: {}", _bar);
}
