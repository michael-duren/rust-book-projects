use colored::*;
use std::{collections::HashMap, io};

fn print_employees(employees: &HashMap<String, Vec<String>>) {
    for (key, value) in employees {
        println!("{}", key.green().bold());
        for name in value {
            println!("{name}");
        }
        print!("\n");
    }
}

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    employees.insert(String::from("Engineering"), vec![String::from("Sally")]);
    employees.insert(String::from("Sales"), vec![String::from("Amir")]);

    loop {
        let title = "Here are the current employees \n";
        println!("{}", title.bold());
        print_employees(&employees);

        let question = "What department would you like to add to ?\n";
        let question2 = "Type the number \n1 - Engineering\n2 - Sales";
        println!("{}", question.green());
        println!("{}", question2.bold());

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let num: usize = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        println!("You entered: {}", num);

        // check that number is valid option
        if num > employees.len() {
            println!("Invalid number");
            continue;
        }

        println!("{}", "Enter Name:".bold());

        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        match num {
            1 => {
                employees
                    .entry(String::from("Engineering"))
                    .and_modify(|emps| emps.push(String::from(name.trim())));
            }
            2 => {
                employees
                    .entry(String::from("Sales"))
                    .and_modify(|emps| emps.push(String::from(name.trim())));
            }
            _ => println!("Incorrect Number"),
        }

        print_employees(&employees);

        println!("Would you like to add another employee? (Y/n)");

        let mut cont_app = String::new();

        io::stdin()
            .read_line(&mut cont_app)
            .expect("Failed to read line");

        println!("Continue: {}", cont_app);

        if cont_app.trim().to_lowercase() == "y" {
            continue;
        } else {
            println!("In Break");
            break;
        }
    }
}
