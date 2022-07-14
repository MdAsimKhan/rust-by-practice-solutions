fn main() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string(); //why convert str to str?
    x
}
//functions return by using ->