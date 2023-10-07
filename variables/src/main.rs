use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /*
     * const - aren't allowed to use mut with const. They aren't just immutable be default always immutable
     * The data type of the value MUST be annotated. const are helpful for maintaing code since
     * There's only one place in the application you would need to change the value
     */
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // naming convention for const is all uppercase

    /*
     * Shadowing - use a previous variable value to assign to the same variable name
     * Is different then mut because we'll get a error if we try to reassign to this variable without using the let keyword
     * The other difference is that we can change the type val ex.: let spaces = " "; let spaces = spaces.len();
     */
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }

    println!("The value of x is: {x}");

    // data types

    // must annotate data type since it cannot be inferred from parse()
    // let guess: u32 = "42".parse().expect("Not a number");

    // scalar types - a single value, four primary scalar types: int, floating point, bool, and characters

    // floating point types
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32

    // char
    // represents a 4 byte unicode scalar value
    let z: char = 'Z'; // specified with single quotes instead of "" for strings
    let emoji: char = '😀';
    print!("{z}");
    print!("{emoji}");

    /*
     * COMPOUND TYPES
     * tuple - a way of grouping together a num of values with a variety of types into one compound type
     * tuples are fixed size in length. Create by writinga comma-separated list of vals in paraentheses.
     * Each position has a type. optional annotations in example
     */
    // let tup: (i32, f64, u8) = (500, 5.4, 1);

    // retrive tuple vals
    // let (x, y, z) = tup;
    // println!("The value of y is: {y}");

    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // can also access with . syntax
    // let five_hundred = x.0;
    // a tuple without any values has a special name `unit`. Written () represent an empty value or an empty return type
    // expressions implicitly return the unit value if they don't return any other value

    // arrays - same rules as C#, fixed, and one data type
    // let a = [1, 2, 3, 4, 5];
    // let first = a[0];
    // you can initialize an aray to contain the same value for each element by specifiying the initial value
    // and length
    // let a = [3; 5]; // would be [3, 3, 3, 3, 3,]
    // checks for out of range index happen at run time, rust will panic if index is out of range

    // out of index
    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at {index} is: {element}");
}

fn another_function() {
    // we could define this in the beginning of the file, doesn't matter where we define it as long as it's in scope
    println!("Another function");
}
