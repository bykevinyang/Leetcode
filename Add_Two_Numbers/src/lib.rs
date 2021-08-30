mod Add_Two_Numbers;

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn example1() {
        let list1 = Add_Two_Numbers::construct_link(vec![5, 6, 4]);
        let list2 = Add_Two_Numbers::construct_link(vec![2, 4, 3]);
        assert_eq!(Add_Two_Numbers::add_two_numbers(list1, list2), Add_Two_Numbers::construct_link(vec![8, 0 ,7]));
    }

    #[test]
    fn example2() {
        let list1 = Add_Two_Numbers::construct_link(vec![0]);
        let list2 = Add_Two_Numbers::construct_link(vec![0]);
        assert_eq!(Add_Two_Numbers::add_two_numbers(list1, list2), Add_Two_Numbers::construct_link(vec![0]));
    }

    #[test]
    fn example3() {
        let list1 = Add_Two_Numbers::construct_link(vec![9, 9, 9, 9, 9, 9, 9]);
        let list2 = Add_Two_Numbers::construct_link(vec![9, 9, 9, 9]);
        assert_eq!(Add_Two_Numbers::add_two_numbers(list1, list2), Add_Two_Numbers::construct_link(vec![8, 9, 9, 9, 0, 0, 0, 1]));
    }
}

