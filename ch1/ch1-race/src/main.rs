use std::thread;
fn main() {
    // first write
    let mut data = 100;

    // second write
    thread::spawn(|| { data = 500; });
    // third reich
    thread::spawn(|| { data = 1000; });
    // Which one will write to data first? That's up to OS so who knows
    println!("{}", data);
}
