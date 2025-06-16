use std::fmt::format;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2,2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
        // assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contaoins_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));

    }

    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}")
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}







