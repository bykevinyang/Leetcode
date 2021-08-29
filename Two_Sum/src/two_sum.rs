use std::convert::TryInto;
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Sort input
    let mut sorted = nums.to_vec();
    sorted.sort(); 
    
    // Create hash map that maps values to orginal index positions
    let mut hash_map = HashMap::new();
    let num_copy = nums.to_vec();
    for (i, num) in num_copy.iter().enumerate(){
        hash_map.insert(num, i);
    }

    // Impl. of binary search
    let mut i: usize = 0;
    while i < sorted.len(){
        let base: i32 = sorted[i];
        let mut upper: usize = sorted.len()-1;
        let mut lower = i+1;
        let mut mid: usize = (upper + lower)/2;
        while lower <= upper{
            let addend = sorted[mid];
            let sum = base + addend;

            if sum == target{
                let original_i_index = hash_map.get(&sorted[i]).unwrap();
                let original_mid_index = hash_map.get(&sorted[mid]).unwrap();

                if sorted[i] == sorted[mid]{
                    let iter_num = nums.to_vec();
                    let updated_original_i_index = iter_num.iter().position(|&x| x==sorted[mid]).unwrap();
                    return [(updated_original_i_index).try_into().unwrap(), (*original_mid_index).try_into().unwrap()].to_vec()
                }

                // println!("\t Sum = Target");
                // print!("\n");
                // println!("Indexes are: {}, {}", i, mid);
                // println!("Values are: {}, {}", sorted[i], sorted[mid]);
                // println!("Target was: {}", target);
                // println!("Solution: {}, {}", original_i_index, original_mid_index);

                return [(*original_i_index).try_into().unwrap(), (*original_mid_index).try_into().unwrap()].to_vec()
            }
            else if sum < target{
                lower = mid + 1;
            }
            else if sum > target{
                upper = mid - 1;
            }
            mid = (upper + lower)/2;
        } 
    i += 1;
    }
    println!("Gone through everything, no solution");
    return [0, 0].to_vec();
}