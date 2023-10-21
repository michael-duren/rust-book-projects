use std::collections::HashMap;

fn get_median(ints: &Vec<i32>) -> i32 {
    let is_even = ints.len() % 2 == 0;

    if is_even {
        let middle_one = ints.get(ints.len() / 2).expect("There to be number");
        let middle_two = ints.get((ints.len() / 2) - 1).expect("There to be number");
        return (middle_one + middle_two) / 2;
    }

    return *ints.get(ints.len() / 2).unwrap();
}

fn get_mode(ints: &Vec<i32>) -> i32 {
    let mut map: HashMap<&i32, i32> = HashMap::new();

    // count repititons
    for int in ints {
        *map.entry(int).or_insert(0) += 1;
    }

    let result = map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(&k, _v)| k)
        .unwrap();

    return *result;
}

fn convert_to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();

    if first_char == 'a'
        || first_char == 'e'
        || first_char == 'i'
        || first_char == 'o'
        || first_char == 'u'
    {
        // result.push_str(word);
        // result.push_str("-hay");
        return String::from(format!("{}-hay", word));
    }

    return String::from(format!("{}-{}ay", &word[1..], first_char));
}

fn main() {
    let mut ints = vec![32, 5, 6, 100, 230, 432, 10, 6];
    // get median
    ints.sort();
    let median = get_median(&ints);
    let mode = get_mode(&ints);

    println!("{}", median);
    println!("{}", mode);

    assert_eq!(String::from("irst-fay"), convert_to_pig_latin("first"));
}
