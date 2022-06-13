/*
You are required to, given a string, replace every letter with its position in the alphabet.

If anything in the text isn't a letter, ignore it and don't return it.

"a" = 1, "b" = 2, etc.

Example
alphabet_position("The sunset sets at twelve o' clock.")
Should return "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11" ( as a string )
*/

pub fn alphabet_position(text: &str) -> String {
    let mut s: Vec<String> = Vec::new();
    let cs: Vec<char> = text.to_lowercase().chars().collect();
    let ca = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    
    for c in cs.into_iter().peekable() {
        let mut number:i32 = 0;
        for i in &ca {
            number += 1;
            if &c == i { 
                s.push(number.to_string());
            }
        }
    }
    
    s.join(" ")
}