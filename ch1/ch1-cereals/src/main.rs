#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice, Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Barley);
    grains.push(Cereal::Millet);
    grains.push(Cereal::Rice);
    grains.push(Cereal::Rye);
    grains.push(Cereal::Spelt);
    drop(grains);
    println!("{:?}", grains);
}
