use withargs::withargs;

#[withargs]
fn main(a: i32, b: i32) {
    println!("Your total is {}", a + b);
}
