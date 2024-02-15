use std::io::stdin;
fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut operator = String::new();

    let _ = stdin().read_line(&mut x);
    let _ = stdin().read_line(&mut operator);
    let _ = stdin().read_line(&mut y);

    let x: f32 = x.trim().parse().unwrap();
    let y: f32 = y.trim().parse().unwrap();
    let operator = operator.trim();

    if operator == "+".to_owned() {
        println!("{}", x + y)
    } else if operator == "-".to_owned() {
        println!("{}", x - y)
    } else if operator == "*".to_owned() {
        println!("{}", x * y)
    } else {
        println!("{}", x / y)
    }
}
