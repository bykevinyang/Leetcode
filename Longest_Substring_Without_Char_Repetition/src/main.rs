fn main() {
    println!("{:?}", length_of_longest_substring("Hello".to_string()));

}

use std::collections::HashMap;
pub fn length_of_longest_substring(s: String) -> i32 {
    // Create hashmap of previously encountered char
    let mut hash = HashMap::new();
    let mut length = 0;
    let mut record_length = 0;

    for char in s.chars() {
        print!("{}: ", char);
        if hash.contains_key(&char) {
            print!("Char exists\n");
            if length > record_length {
                print!("Updating record length to: {}", length);
                record_length = length;
            } else {
                print!("\nNot updating record length\n");
            }
            hash.clear();
            length = 0;
        } else {
            print!("Char does not exist, adding to hash");
            hash.insert(char, true);
            length += 1;
            print!("\nLength is: {}\n", length);
        }
        print!("\n");
    }

    if length > record_length {
        record_length = length;
    }
    return record_length
}
