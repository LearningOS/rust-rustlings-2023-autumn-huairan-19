use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    Empty,
    BadLen,
    NoName,
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let fields: Vec<&str> = s.split(',').collect();
        if fields.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = fields[0].to_string();
        let age_result = fields[1].parse::<usize>();

        match age_result {
            Ok(age) => Ok(Person { name, age }),
            Err(_) => Err(ParsePersonError::ParseInt(age_result.unwrap_err())),
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }

    // 测试其他情况...
}
