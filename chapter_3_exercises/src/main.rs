fn main() {
    let celsius = convert_betweeen_f_c(12, false);
    println!("{celsius}");

    let fib = generate_nth_fibonacci(30);
    println!("{fib}");

    let fib_r = fib_with_recurssion(30);
    println!("{fib_r}");

    days_of_christmas();
}

fn convert_betweeen_f_c(number:i32, is_farenheit: bool) -> i32 {
    if is_farenheit {
        return (number -32) * 5/9;
    } else {
        return (number * 9/5) + 32;
    }
}


fn generate_nth_fibonacci(n: i32) -> i32 {
    let mut a = 0; // the number from two numbers back
    let mut b = 1; // the current
    let mut c: i32; // the number previous

    if n == 0 {
        return 0;
    }

    let mut count = 2;
    while count < n {
        c = a + b; // store the new number
        a = b; // store the number that will be two previous
        b = c; // the current value in the sequence

        count += 1; // increment counter
    }
    return b;
}

fn fib_with_recurssion(n: i32) -> i32 {
    if  n <= 1 {
        return n;
    }
    return fib_with_recurssion(n - 1) + fib_with_recurssion(n - 2);
}

fn days_of_christmas() {
    let lyrics = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
    ];
    let number_word = ["first", "second", "third", "fourth", "fifth", "sixth", "seven", "eight", "nine"];
    
    for number in 0..8 {
        let word = number_word[number];
        println!("On the {word} day of christmas my true love brought to me");
        for y in 0..number {
            let line = lyrics[y];
            println!("{line}");
        }
        if number == 0 {
            println!("A partridge in a pear tree");
        } else {
            println!("And A partridge in a pear tree");
        }
    }
}
