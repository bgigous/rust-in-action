fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];
    let buffer_overflow = fruit[4]; // crash yo
    assert_eq!(buffer_overflow, '🍉')
}
