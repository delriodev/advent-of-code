import re
# PART ONE:
# Extract and parse integers and characters from a file and make validations

data = []
correct_passwords_one = 0
correct_passwords_two = 0

def nb_chars_txt(c, string):
    count = 0
    for char in string:
        if (char == c):
            count += 1
    return count

def isValidOne(min, max, value):
    return value >= min and value <= max

def isValidTwo(min, max, char, string):
    return ((string[min-1] == char) and not (string[max-1] == char)) or (not (string[min-1] == char) and (string[max-1] == char))
        
# 1. Parse the data into dictionaries into an array ( Practicing )
with open('input') as input:
    for index, line in enumerate(input, start=1):
        min_val = int(line.split('-')[0])
        max_val = int(line.split('-')[1].split(' ')[0])
        char = line.split(' ')[1][0]
        password = re.sub('\n', '', line.split(' ')[2])
        count = nb_chars_txt(char, password)
        is_valid_one = isValidOne(min_val, max_val, count)
        is_valid_two = isValidTwo(min_val, max_val, char, password)
        data.append({'index': index, 'min': min_val, 'max': max_val, 'char': char, 'password': password, 'count': count, 'valid_one': is_valid_one, 'valid_two': is_valid_two})

for obj in data:
    if(obj['valid_one']):
        correct_passwords_one += 1

for obj in data:
    if(obj['valid_two']):
        correct_passwords_two += 1

print(correct_passwords_one)
print(correct_passwords_two)
