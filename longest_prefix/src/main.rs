fn main() {
    let test1: Vec<String> =  vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
    println!("Test 1: {}", longest_common_prefix(test1));

    let test2: Vec<String> =  vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
    println!("Test 2: {}", longest_common_prefix(test2));
}

pub fn string_is_short(s: &String, index: usize) -> bool {
    index >= s.len() 
}

pub fn char_at_ascii_string(s: &String, index: usize) -> char {
    let bytes = s.as_bytes();
    return bytes[index] as char;
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return "".to_string();
    }

    let mut index = 0;
    'outer: while index < strs[0].len() {
        if string_is_short(&strs[0], index) {
            break 'outer;
        }
        let test_char = char_at_ascii_string(&strs[0], index);       

        for k in 1..strs.len() {
            if string_is_short(&strs[k], index) {
                break 'outer;
            }

            let c = char_at_ascii_string(&strs[k],index);
            if c != test_char {
                break 'outer;
            }
        }
        index += 1;
    }
    
    let result = &strs[0][0..index];
    return result.to_string();
}
