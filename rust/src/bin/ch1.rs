#![allow(dead_code)]

use std::{collections::HashMap};

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
// returns true if strings are identical, otherwise false.
fn check_permutation(str1: String, str2: String) -> bool {
    if str1.len() != str2.len() {
        return false;
    }

    // collect strings into vectors so we can utelize the sort_by function
    let mut chars1: Vec<char> = str1.chars().collect();
    chars1.sort_by(|a, b| b.cmp(a));

    let mut chars2: Vec<char> = str1.chars().collect();
    chars2.sort_by(|a, b| b.cmp(a));

    chars1 == chars2
}

// interview question 1.3
// URLify: Write a method to replace all spaces in a string with '%20'. You may assume that the string
// has sufficient space at the end to hold the additional characters, and that you are given the "true"
// length of the string. (Note:  If implementing in Java, please use a character array so that you can
// perform this operation in place.)
fn url_ify(input: String) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    // input.trim().replace(" ", "%20") 

    let mut new_str: Vec<String> = Vec::new(); 

    for c in input.trim().clone().chars().into_iter() {
        match c {
            ' ' => new_str.push("%20".to_string()),
            _ => new_str.push(c.to_string()), 
        }
    } 
    return new_str.concat();
}

// interview question 1.4
// Palindrome Permutation: Given a string, write a function to check if it is a permutation of a palin­
// drome. A palindrome is a word or  phrase that is the same forwards and backwards. A permutation 
// is a rearrangement of letters. The palindrome does not need to be limited to just dictionary words.
fn palindrome_permutation(input: String) -> bool {
    let mut hmap: HashMap<char, usize> = HashMap::new();

    // stupid that this took me so long to catch, but we have to make sure we 
    // remove all the spaces from the word first.
    let s = input.replace(" ", "");

    // go through all of the characters in the string and tally up how much we have of each.
    for c in s.chars().into_iter() {
        match hmap.contains_key(&c) {
            true => { hmap.entry(c).and_modify(|x| *x+= 1); }
            false => {hmap.insert(c, 1);}
        }
    }

    dbg!(&hmap);

    // this is our tracker to count how many odd values we have in the map
    let mut found_odd: usize = 0;

    // now that we have the count, verify that only (at most) 1 character is odd
    hmap.values().for_each(|values| { 
        if values % 2 != 0 { found_odd += 1; }
    }); 

    found_odd <= 1
}


// interview question 1.5
// One Away: There are three types of edits that can be performed on strings: insert a character,
// remove a character, or replace a character. Given two strings, write a function to check if they are
// one edit (or zero edits) away.
fn one_away(input1: String, input2: String) -> bool {
    if input1.len() - input2.len() > 1 { return false;}




    true
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
        assert_eq!(
            check_permutation(String::from("was"), String::from("")),
            false
        );
        assert_eq!(
            check_permutation(String::from("zen"), String::from("maps")),
            false
        );
        assert_eq!(
            check_permutation(String::from("red"), String::from("erd")),
            true
        );
    }
    #[test]
    fn check_url_ify() {
        assert_eq!(url_ify(String::from("Mr John Cena ")), "Mr%20John%20Cena"); 
        assert_eq!(url_ify(String::from("     exam ple      ")), "exam%20ple"); 
    }

    #[test]
    fn check_palindrome_permutation() {
        assert_eq!(palindrome_permutation(String::from("atco cta")), true);  
        assert_eq!(palindrome_permutation(String::from("atco www")), false);  
    }
}

