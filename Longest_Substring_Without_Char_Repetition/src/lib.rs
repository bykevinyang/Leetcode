mod main;

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = main::length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn example2() { 
        let result = main::length_of_longest_substring("bbbbb".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn example3() { 
        let result = main::length_of_longest_substring("pwwkew".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn example4() { 
        let result = main::length_of_longest_substring("".to_string());
        assert_eq!(result, 0);
    }

}