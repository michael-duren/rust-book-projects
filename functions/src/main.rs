fn main() {
    another_function(32);
    let sum = add(3, 5);
    let sum = sum.to_string();
    println!("{sum}");

    counter();
    count_label();

    while_loops();

    for_loopz();
    for_loopz_with_range();

    /*
     * Statements are instructions that perform some action and do not have a return value
     * Expressions evaluate to a resultant value
     * Calling a function is an expression
     */

    // LOOPS
    /*
     * Rust has three loops: loop, while and for
     * loops: infinite loops broken with continue or break
     */
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn counter() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is {result}");
}

//  loop labels
fn count_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFT OFF!!!!");
}

fn for_loopz () {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is : {element}");
    }
}

fn for_loopz_with_range() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("DOUBLE LIFTOFF");
}
