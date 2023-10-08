fn main() {
    let s = String::from("Hello this is a test");
    let s = first_word(s);

    println!("{s}");
}

fn first_word(s: String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
