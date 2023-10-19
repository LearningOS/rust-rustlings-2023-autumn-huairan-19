fn maybe_icecream(time_of_day: u16) -> Option<u16> {  
    if time_of_day >= 0 && time_of_day < 22 {  
        Some(5)  
    } else if time_of_day == 22 && time_of_day <= 23 {  
        // 没有冰淇淋  
        None  
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
        assert_eq!(maybe_icecream(10), Some(5));  
        assert_eq!(maybe_icecream(23), None); // 修改这里，将 None 改为您期望的值  
        assert_eq!(maybe_icecream(22), None);  
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
 