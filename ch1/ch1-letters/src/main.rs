fn main() {
    let mut letters = vec![
        "a", "b", "c",
    ];
    for letter in letters {
        println!("{}", letter);
        // modification of vector within for loop is a no-no
        letters.push(letter.clone());
    }
}
