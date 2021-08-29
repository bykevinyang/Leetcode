mod two_sum;

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn example1(){
        assert_eq!(two_sum::two_sum([2, 7, 11, 15].to_vec(), 9), [0, 1]);
    }

    #[test]
    fn example2(){
        assert_eq!(two_sum::two_sum([3, 2, 4].to_vec(), 6), [1, 2]);
    }

    #[test]
    fn example3(){
        assert_eq!(two_sum::two_sum([3,3].to_vec(), 6), [0, 1]);
    }
}