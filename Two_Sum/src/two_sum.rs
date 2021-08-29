use std::convert::TryInto;
use std::convert::TryFrom;
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
    println!("{:?}", hash_map);

    // Impl. of binary search
    let mut i: usize = 0;
    while i < sorted.len(){
        let base: i32 = sorted[i];
        
        let mut upper: usize = sorted.len();
        let mut lower = i+1;
        let mut mid: usize = (upper + lower)/2;

        while lower <= upper{
            let addend = sorted[mid];
            let sum = base + addend;
            print!("Base: {}, Addend: {}, Sum: {}", base, addend, sum);

            if sum == target{
                let original_i_index = hash_map.get(&sorted[i]).unwrap();
                let original_mid_index = hash_map.get(&sorted[mid]).unwrap();

                println!("\t Sum = Target");
                print!("\n");
                println!("Indexes are: {}, {}", i, mid);
                println!("Values are: {}, {}", sorted[i], sorted[mid]);
                println!("Target was: {}", target);
                println!("Solution: {}, {}", original_i_index, original_mid_index);
                
                return [(*original_i_index).try_into().unwrap(), (*original_mid_index).try_into().unwrap()].to_vec()
            }
            else if sum < target{
                print!("\t Sum < Target");
                lower = mid + 1;
                println!("\t New Lower: {}, {}", sorted[lower], lower);
            }
            else if sum > target{
                print!("\t Sum > Target");
                upper = mid - 1;
                println!("\t New Upper: {}, {}", sorted[upper], upper);
            }
            mid = (upper + lower)/2;
        } 
    i += 1;
    }
    println!("Gone through everything");
    return [1,2].to_vec()
}