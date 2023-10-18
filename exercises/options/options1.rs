fn maybe_icecream(time_of_day: u16) -> Option<u16> {  
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a  
    // value of 0. The Option output should gracefully handle cases where  
    // time_of_day > 23.  
    if time_of_day < 22 {  
        Some(5)  
    } else if time_of_day == 22 {  
        Some(0)  
    } else {  
        None  
    }  
 }
 
 #[cfg(test)]  
 mod tests {  
    use super::*;
 
    #[test]  
    fn check_icecream() {  
        assert_eq!(maybe_icecream(9), Some(5));  
        assert_eq!(maybe_icecream(10), Some(0));  
        assert_eq!(maybe_icecream(23), Some(0));  
        assert_eq!(maybe_icecream(22), Some(0));  
        assert_eq!(maybe_icecream(25), None);  
    }
 
    #[test]  
    fn raw_value() {  
        let icecreams = maybe_icecream(12);  
        match icecreams {  
            Some(icecreams) => assert_eq!(icecreams, 5),  
            None => panic!("There is no icecream left."),  
        }  
    }  
 }
 