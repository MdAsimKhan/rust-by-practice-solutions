
// Fix the error below with least amount of modification
fn main() {
//x and y are global vars
    let x: i32 = 10;
    let y: i32 = 5;
    {//curly braces define scope of vars
        println!("The value of x is {} and value of y is {}", x, y);//print in a non-global scope
    }
    println!("The value of x is {} and value of y is {}", x, y);//global print
}
