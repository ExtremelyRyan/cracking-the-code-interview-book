#![allow(dead_code)]
// needed help from https://github.com/dan-fritchman/CtCI-6th-Edition-Rust/blob/main/src/chapter_01/
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
    true
}

// ------------------------------------

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

// ------------------------------------

// interview question 1.3
// URLify: Write a method to replace all spaces in a string with '%20'. You may assume that the string
// has sufficient space at the end to hold the additional characters, and that you are given the "true"
// length of the string. (Note: If implementing in Java, please use a character array so that you can
// perform this operation in place.)
fn url_ify(input: String) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    // input.trim().replace(" ", "%20")

    let mut new_str: Vec<String> = Vec::new();

    for c in input.trim().clone().chars() {
        match c {
            ' ' => new_str.push("%20".to_string()),
            _ => new_str.push(c.to_string()),
        }
    }
    new_str.concat()
}

// ------------------------------------

// interview question 1.4
// Palindrome Permutation: Given a string, write a function to check if it is a permutation of a palin­
// drome. A palindrome is a word or phrase that is the same forwards and backwards. A permutation
// is a rearrangement of letters. The palindrome does not need to be limited to just dictionary words.
fn palindrome_permutation(input: String) -> bool {
    let mut hmap: HashMap<char, u32> = HashMap::new();

    // stupid that this took me so long to catch, but we have to make sure we
    // remove all the spaces from the word first.
    let s = input.replace(' ', "");

    // go through all of the characters in the string and tally up how much we have of each.
    for c in s.chars() {
        match hmap.contains_key(&c) {
            true => { 
                hmap.entry(c).and_modify(|x| *x += 1);
            }
            false => {
                hmap.insert(c, 1);
            }
        }
    } 

    // this is our tracker to count how many odd values we have in the map
    let mut found_odd: usize = 0;

    // now that we have the count, verify that only (at most) 1 character is odd
    hmap.values().for_each(|values| {
        if values % 2 != 0 {
            found_odd += 1;
        }
    });

    found_odd <= 1
}

// ------------------------------------

// interview question 1.5
// One Away: There are three types of edits that can be performed on strings: insert a character,
// remove a character, or replace a character. Given two strings, write a function to check if they are
// one edit (or zero edits) away.

fn one_away(input1: String, input2: String) -> bool {
    let diff: i16 = input1.len() as i16 - input2.len() as i16;

    match diff {
        0 => same_len(input1, input2),
        1 => one_edit(input1, input2),
        -1 => one_edit(input2, input1),
        _ => false,
    }
}

fn same_len(s1: String, s2: String) -> bool {
    // our bool check to see if / when we spot the difference
    let mut found_differnece = false;

    let mut s2_chars = s2.chars();

    for ch1 in s1.chars() {
        if ch1 != s2_chars.next().unwrap() {
            // found our mis-match
            if found_differnece {
                // bail out if we find more than one.
                return false;
            } 
            found_differnece = true;
        }
    }
    true
}
fn one_edit(longer: String, shorter: String) -> bool {
    let mut found_differnece = false;
    let mut longer_iter = longer.chars();
    for short in shorter.chars() {
        if longer_iter.next().unwrap() != short {
            if found_differnece {
                return false;
            }
            // Set the "seen a difference" flag, skip over and check the next longer-iter character
            found_differnece = true;
            if longer_iter.next().unwrap() != short {
                return false;
            }
        }
    }
    true
}

// ------------------------------------

// Interview Question 1.6
// String Compression: Implement a method to perform basic string compression using the counts
// of repeated characters. For example, the string aabcccccaaa would become a2blc5a3. If the
// "compressed" string would not become smaller than the original string, your method should return
// the original string. You can assume the string has only uppercase and lowercase letters (a - z).
fn string_compression(s: String) -> String {
    let mut compressed = String::new();
    if s.is_empty() || s.clone().chars().any(|x| x.is_numeric()) { 
        return compressed;
    }
    let mut chars = s.chars();

    // Pop off the first character as our loop prelude
    let mut prev = chars.next().unwrap();
    println!("{prev}");
    prev = prev.to_lowercase().next().unwrap();
    println!("{prev}");
    // Initialize a count of consecutive identical characters
    let mut count = 1;

    // And iterate over the rest of the string
    for c in chars {
        if c.to_lowercase().next().unwrap() == prev {
            count += 1;
        } else {
            compressed.push(prev);
            compressed.push_str(&count.to_string());
            prev = c;
            count = 1;
        }
        if compressed.len() >= s.len() {
            return s.to_string();
        }
    }
    // Now we're at the end of the string, so we need to push the last character(s)
    compressed.push(prev);
    compressed.push_str(&count.to_string());
    if compressed.len() >= s.len() {
        return s.to_string();
    }
    // Finished, return the compressed string
    compressed
}

// ------------------------------------
 
// Rotate Matrix: 
// Given an image represented by an NxN matrix, where each pixel in the image is 4 bytes,
// write a method to rotate the image by 90 degrees.
// Can you do this in place? 

/// Primary Implementation Method
///
/// Rotate 90 degrees clockwise, inline, running a "layer" at a time from the matrix exterior to center.
/// Note the ultra-cool const-generic usage.
///
pub fn rotate_matrix<const N: usize>(mat: &mut [[isize; N]; N]) {
    for layer in 0..N / 2 {
        let last = N - 1 - layer;
        for idx in layer..last {
            // The reverse-index, used for rows & cols moving "backward"
            let rev = N - 1 - idx;
            // Use one temporary to store the initial top-left
            let temp = mat[layer][idx];
            // Move left to top
            mat[layer][idx] = mat[rev][layer];
            // Move bottom to left
            mat[rev][layer] = mat[last][rev];
            // Move right to bottom
            mat[last][rev] = mat[idx][last];
            // Move top to right, via the temporary
            mat[idx][last] = temp;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_unique() {
        let test_cases = [("", false), ("maps", true), ("shall", false)];

        for case in test_cases {
            assert_eq!(is_unique(case.0.to_string()), case.1);
        }
    }

    #[test]
    fn check_check_permutation() {
        let test_cases = [
            ("was", "", false),
            ("zen", "maps", false),
            ("red", "erd", true),
        ];

        for case in test_cases {
            assert_eq!(
                check_permutation(case.0.to_string(), case.1.to_string()),
                case.2
            );
        }
    }
    #[test]
    fn check_url_ify() {
        assert_eq!(url_ify(String::from("Mr John Cena ")), "Mr%20John%20Cena");
        assert_eq!(url_ify(String::from("   exam ple   ")), "exam%20ple");
    }

    #[test]
    fn check_palindrome_permutation() {
        assert_eq!(palindrome_permutation(String::from("atco cta")), true);
        assert_eq!(palindrome_permutation(String::from("racecar")), true);
        assert_eq!(palindrome_permutation(String::from("atco www")), false);
    }

    #[test]
    fn check_one_away() {
        let test_cases = [
            // no changes
            ("pale", "pale", true),
            ("", "", true),
            // one insert
            ("pale", "ple", true),
            ("ple", "pale", true),
            ("pales", "pale", true),
            ("ples", "pales", true),
            ("pale", "pkle", true),
        ];

        for case in test_cases {
            assert_eq!(one_away(case.0.to_string(), case.1.to_string()), case.2);
        }
    }

    #[test]
    fn check_string_compression() {
        let test_cases = [
            ("1111111545555", ""),
            ("aabcccccaaa", "a2b1c5a3"),
            ("abcdef", "abcdef"),
            ("aabb", "aabb"),
            ("aaa", "a3"),
            ("a", "a"),
            ("", ""),
        ];

        for case in test_cases { 
            assert_eq!(string_compression(case.0.to_string()), case.1.to_string());
        }
    }

    #[test]
fn test_rotate_matrix() {
    let mut test_case = (
        [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        [[7, 4, 1], [8, 5, 2], [9, 6, 3]],
    );
    rotate_matrix(&mut test_case.0);
    assert_eq!(test_case.0, test_case.1);

    let mut test_case = (
        [
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25],
        ],
        [
            [21, 16, 11, 6, 1],
            [22, 17, 12, 7, 2],
            [23, 18, 13, 8, 3],
            [24, 19, 14, 9, 4],
            [25, 20, 15, 10, 5],
        ],
    );
    rotate_matrix(&mut test_case.0);
    assert_eq!(test_case.0, test_case.1);
}
}
