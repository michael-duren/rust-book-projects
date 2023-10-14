fn main() {
    // let s = String::from("Hello this is a test");
    // let s = first_word(s);

    // println!("{s}");

    // let s = String::from("Hello this is a test");
    // let hello = &s[0..5]; // using string slices instead of first_word fn
    // let this = &s[6..10]; // these are references to portions of that string
    // let hello_two = first_word(&s);

    // println!("{hello_two} from hello_two");

    // println!("{hello} {this}");

    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3];
    // assert_eq!(slice, &[2, 3]);

    let foo = vec![1, 2, 3];
    let mut foo_iter = foo.iter();

    assert!(foo_iter.next() == Some(&1));

    let result = descending_order(32918);
    print!("result is {}", result);
    assert!(result == 98321);
}

// fn first_word(s: &str) -> &str {
//     // go through the string el by el, convert string to array of bytes
//     let bytes = s.as_bytes();

//     // create iterator over the arr of bytes using iter returns each el
//     // enumerate wwraps the result of iter and returns each el as apart of a tuple instead
//     // the first el of the tuple returned from enumerate is index, second el is ref to el
//     for (i, &item) in bytes.iter().enumerate() {
//         // byte literal syntax
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

fn descending_order(x: u64) -> u64 {
    let mut x: Vec<char> = x.to_string().chars().collect(); // convert to char list
    let count = x.len(); // get length

    for _ in 0..count {
        for j in 0..count - 1 {
            let a = x.get(j).unwrap().to_digit(10).unwrap();
            let b = x.get(j + 1).unwrap().to_digit(10).unwrap();

            if b > a {
                let tmp = x[j];
                x[j] = x[j + 1];
                x[j + 1] = tmp;
            }
        }
    }

    return x.iter().collect::<String>().parse::<u64>().unwrap();
}
