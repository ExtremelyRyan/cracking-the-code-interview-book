 

# Interview Question 1.1
# Implement an algorithm to determine if a string has all unique characters
def is_unique(input: str) -> bool:
    char_dict = {} 
    for c in input:
        if c in char_dict:
            return False
        char_dict[c] = 1
    # print(char_dict)
    return True

def test_is_unique():
    print("is_unique(avbbc): " , is_unique("avbbc")) # True
    print("is_unique(abc): " , is_unique("abc")) # False
    
# ------------------------

# Interview Question 1.2
# Check Permutation: Given two strings, write a method to decide if one is a permutation of the other.
def check_permutation(str1: str, str2: str) -> bool:
    if len(str1) != len(str2):
        return False
    
    sorted1 = ''.join(sorted(str1))
    sorted2 = ''.join(sorted(str2))
    
    return sorted1 == sorted2

def test_check_permutation():
    print("check_permutation (was, ): " ,     check_permutation("was", "")) # False
    print("check_permutation (zen, maps): " , check_permutation("zen", "maps")) # False
    print("check_permutation (red, erd): " ,  check_permutation("red", "erd")) # True

# ------------------------

#Interview Question 1.3
# URLify: Write a method to replace all spaces in a string with '%20'. You may assume that the string
# has sufficient space at the end to hold the additional characters, and that you are given the "true"
# length of the string. (Note: If implementing in Java, please use a character array so that you can
# perform this operation in place.)
def URLify(input: str) -> str:
    print(input)
    
    if len(input) < 1:
        print("input is an empty string.")
        return input
    
    return input.strip().replace(" ", "%20")
    
def test_URLify():
    print("URLify (' Mr John Smith    '):" , URLify(" Mr John Smith    ")) # Mr%20John%20Smith 

# ------------------------

#Interview Question 1.4
# Palindrome Permutation: Given a string, write a function to check if it is a permutation of a palindrome.
# A palindrome is a word or phrase that is the same forwards and backwards. A permutation
# is a rearrangement of letters. The palindrome does not need to be limited to just dictionary words.
def palindrome_permutation(input: str) -> bool: 
    print(input)
    letters = {}
    
    for c in input.lower():
        if c in letters:
            num = letters.get(c)
            letters[c] = num+1
        else:
            letters[c] = 1
    
    return False
    
def test_palindrome_permutation():
    print("palindrome_permutation ('Tact Coa'):" , palindrome_permutation("Tact Coa")) # True "Taco Cat"


if __name__ == "__main__":
    test_is_unique()
    test_check_permutation()
    test_URLify()
    test_palindrome_permutation()