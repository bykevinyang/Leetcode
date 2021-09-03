use std::collections::LinkedList;
use std::collections::HashMap;
fn main() {
    find_longest_sub("joemamaisintown".to_string());
}

pub fn find_longest_sub(s: String) -> i32 {
    let mut letters: LinkedList<char> = LinkedList::new();
    for char in s.chars() {
        letters.push_back(char);
    }
    
    println!("List: {:?}", letters);
    return 10;
}

pub fn length_of_longest_substring(s: String) -> i32 {
    // Create hashmap of previously encountered char
    let mut hash = HashMap::new();
    let mut length = 0;
    let mut record_length = 0;
    let mut prev_char: char = ' ';

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
            hash.remove(&char);
            hash.insert(char, true);
            println!("\nPrev char is: {}", prev_char);
            if prev_char == char {
                length = 1;
            }
            println!("{:?}", hash);
            println!("Length is: {}", length);

        } else {
            print!("Char does not exist, adding to hash");
            hash.insert(char, true);
            length += 1;
            print!("\nLength is: {}\n", length);
        }
        print!("\n");
        prev_char = char;
    }

    if length > record_length {
        record_length = length;
    }
    return record_length
}
