# Working with dictionaries
# Count the number of valid passports - those that have all required fields. Treat cid as optional.

import re

improved_required_fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
required_fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid', 'cid']
eye_colors = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']

def parseLineIntoPassport(_line, _passport):
    # print(_line.split(' '))
    for code in _line.split(' '):
        # print(code)
        key = code.split(':')[0]
        # print(key)        
        value = code.split(':')[1]
        # print(value)
        _passport[key] = value

def heightIsValid(height):
    # A 2 or 3 digit number followed by cm or in
    if(re.match("^([0-9]{2,3}(in|cm))$", height) ):
        value = int(re.findall(r'\d+', height)[0])
        # if cm, number in [150, 193]
        if('cm' in height):
            return value >= 150 and value <= 193
        # if in, number in [59, 76]
        else:
            return value >= 59 and value <= 76
            
with open('input') as input:
    lines = input.read().splitlines()
    lines.append('')
# print(lines)

passports = []
passport = {}

for line in lines:
    if(line != ''):
        parseLineIntoPassport(line, passport)
    else:
        # print(passport)
        passports.append(passport)
        passport = {}
# print(passports)

valid_passports = 0

for dictionary in passports:
    # print(list(dictionary.keys()))
    if(all(field in list(dictionary.keys()) for field in improved_required_fields)):
        if(int(dictionary['byr']) >= 1920 and int(dictionary['byr']) <= 2002):
            if(int(dictionary['iyr']) >= 2010 and int(dictionary['iyr']) <= 2020):
                if(int(dictionary['eyr']) >= 2020 and int(dictionary['eyr']) <= 2030):
                    if(heightIsValid(dictionary['hgt'])):
                        if(re.match("^#[a-f0-9]{6}$", dictionary['hcl'])):
                            if(dictionary['ecl'] in eye_colors):
                                if(re.match("^[0-9]{9}$", dictionary['pid'])):
                                    valid_passports += 1
                        
print(valid_passports)