#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target: Option<&str> = Some(target);

        match optional_target {
            Some(word) => {
                assert_eq!(word, target);
            }
            None => {
                println!("No value found");
            }
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range as i8;

        while let Some(integer) = optional_integers.pop().flatten() {
            assert_eq!(integer, cursor as i8);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}

