// fn add<T>(i: T, j: T) -> T {
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let a: i32 = 5;
    let b: i32 = 8;
    let c: i32 = add(a, b);
    println!("{}", c);
}
