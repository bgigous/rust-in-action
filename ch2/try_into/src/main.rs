use std::convert::TryInto;

fn main() {
    let a = 10_i32;
    let b = 100_u16;
    let b_ = b.try_into()
              .unwrap();

    if a < b_ {
        println!("Ten is less than one hundred, ¿capisce?");
    }

    let a_: u16 = a.try_into()
              .unwrap();

    if a_ < b {
        println!("Ten is less than one hundred, ¿capisce?");
    }
}
