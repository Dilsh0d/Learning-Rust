fn main() {
    // let x = 5; If we are use this then give us ^^^^^ cannot assign twice to immutable variable
    let mut x = 5;
    println!("x = {}",x);
    println!("x = {x}");
    x = 7;
    println!("x = {x}");
}
