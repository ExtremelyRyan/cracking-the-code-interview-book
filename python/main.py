 

def test_is_unique():
    # print("is_unique result (avbbc): ",is_unique("avbbc"))
    # print("is_unique result (abcde): ",is_unique("abc"))
    assert is_unique("avbbc") == False
    assert is_unique("abc") == True

# Implement an algorithm to determine if a string has all unique characters
def is_unique(input: str) -> bool:
    char_dict = {}

    for c in input:
        if char_dict.__contains__(c):
            return False
        else:
            char_dict[c] = 1
            # current_value = char_dict.get(c)
            # char_dict.update(c, current_value+1)
    return True

if __name__ == "__main__":
    test_is_unique()