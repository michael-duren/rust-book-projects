pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(num: usize) -> usize {
    num + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 10 and 100 got {}", value);
        }
        return Guess { value };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
