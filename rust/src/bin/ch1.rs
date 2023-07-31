#![allow(dead_code)]

use std::collections::HashMap;

fn main() {}

// interview question 1.1
// implement an alogorithm to determine if a string has all unique characters.
// What if you cannot use additional data structures?
fn is_unique(input: String) -> bool {
    let mut hm: HashMap<char, char> = HashMap::new(); 
    let binding = input.to_ascii_lowercase();
    let ch = binding.chars();
    if input.is_empty() {
        return false;
    } 
    for c in ch {
        if hm.contains_key(&c) {
            return false;
        }
        hm.insert(c, c);
    }
    return true;
}

// interview question 1.2
// given two strings. write a method to decide if one is a permutation of the other.
fn check_permutation(str1: String, str2: String) -> bool {

    if str1.len() != str2.len() { return false;}
 
    // collect strings into vectors so we can utelize the sort_by function
    let mut chars1: Vec<char> = str1.chars().collect();
    chars1.sort_by(|a, b| b.cmp(a));

    let mut chars2: Vec<char> = str1.chars().collect();
    chars2.sort_by(|a, b| b.cmp(a));

    chars1 == chars2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_unique() {
        assert_eq!(is_unique(String::from("")), false);
        assert_eq!(is_unique(String::from("hash")), false);
        assert_eq!(is_unique(String::from("tuesday")), true);
    }

    #[test]
    fn check_check_permutation() {
        assert_eq!(check_permutation(String::from("was"),String::from("")), false);
        assert_eq!(check_permutation(String::from("hash"),String::from("maps")), false);
        assert_eq!(check_permutation(String::from("red"),String::from("dar")), true);
    }


}