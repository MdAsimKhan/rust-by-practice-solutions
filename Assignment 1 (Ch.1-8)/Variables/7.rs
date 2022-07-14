fn main() {
    let _x = 1;
}
//put underscore in front of unsed var to remove warning

or use 
#[allow(unused_variables)]
fn main() {
    let x = 1;
}