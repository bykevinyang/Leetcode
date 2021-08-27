fn main(){

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted = nums;
        sorted.sort(); 

        let sorted_len = sorted.len().into();
        // Impl. of binary search
        let mut middle: i32 = sorted_len / 2; 
        let mut lower: i32 = 0;
        let mut upper: i32 = sorted_len; 
        for num in sorted{
            println!("{}", num);

        }
     return [1,2].to_vec()
    }

    let arr = vec![32, 2, 4, 10, 28, 30];    

    two_sum(arr, 32);
    
}