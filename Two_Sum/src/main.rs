mod two_sum;

fn main(){
    let arr = vec![32, 2, 4, 10, 28, 30];    
    let example = vec![3, 2, 4];
    let example_target = 6;
    println!("{:?}", two_sum::two_sum(example, example_target));
}