fn main() {
    let twenty = 20_;
    let twenty_one: i32 = 21;
    let twenty_two: i32 = 22i32;
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));
    let forty_twos = [
        42.0,
        42f32,
        // underscores in numbers are (usually) completely ignored by the compiler
        42.0___________________________0_____________________________________f32,
    ];

    println!("{:02}", forty_twos[0]);
}
