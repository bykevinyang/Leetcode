pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sorted = nums;
    sorted.sort(); 
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
                println!("\t Sum = Target");
                print!("\n");
                println!("Indexs are: {}, {}", i, mid);
                println!("Values are: {}, {}", sorted[i], sorted[mid]);
                println!("Target was: {}", target);
                
                return [i.try_into().unwrap(), mid.try_into().unwrap()].to_vec();
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