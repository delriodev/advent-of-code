# Graph traversal

import re

TARGET = 'shiny gold'

def parseDocument(lines, rules):
    for index, line in enumerate(lines):
        # Remove useless information
        line = line.replace('bags contain no other bags.', '')
        line = line.replace(' contain', ',')
        line = line.replace(' bags', '')
        line = line.replace(' bag', '')
        line = line.replace(', ', ',')
        line = line[:-1]
        # Split information
        number_bag_list = line.split(',')
        key = number_bag_list[0]
        values = {}
        for i in range(1, len(number_bag_list)):
            # Remove letters # Another way to do this: amount = [int(s) for s in number_bag_list[i].split() if s.isdigit()][0]
            amount = int(re.sub('\D', '', number_bag_list[i]))
            # Remove digits
            color = re.sub('\d ', '', number_bag_list[i])

            values[color] = amount
        rules[key] = values

# Depth-first search
def checkBagsRecurs(key, rules, valid_bags, checked_bags):
    if(key not in checked_bags):
        checked_bags.add(key)

        for adjecentKey in rules[key].keys():
            if(adjecentKey == TARGET or checkBagsRecurs(adjecentKey, rules, valid_bags, checked_bags)):
                valid_bags.add(key)
                return True

    return key in valid_bags

# Explore the graph once for each vertex. Maybe a bit overkill, it doesn't matter.
def findValidBags(rules, valid_bags, checked_bags):

    for key in rules.keys():
        checkBagsRecurs(key, rules, valid_bags, checked_bags)

def isLeaf(node):
    return not rules[node] 

def countTotalBags(dictionary):

    # If vertex has no children 
    _sum = 0
    for item in dictionary.items():
        print(item[1])
        print(rules[item[0]])
        countTotalBags(rules[item[0]])
        
        

# The actual executed part of the script

rules = {} # Graph
checked_bags = set() # Label as visited
valid_bags = set() # Indicates that this edge gets to the targe edge
total_bags = 0

with open('input') as input:
    lines = input.read().splitlines()

parseDocument(lines, rules)
findValidBags(rules, valid_bags, checked_bags)
print(len(valid_bags))

total_bags = countTotalBags(rules[TARGET])
print(total_bags)








'''
6339
def countTotalBags(dictionary, p):

    # If vertex has no children 
    if(not dictionary):
        return p
    
    else:
        _sum = 0
        for item in dictionary.items():
            _sum += countTotalBags(rules[item[0]], item[1])
        return item[1] * _sum + item[1]

888
def countTotalBags(dictionary):

    # If vertex has no children 
    if(not dictionary):
        return 1
    else:
        _sum = 0
        for item in dictionary.items():
            _inner_sum = countTotalBags(rules[item[0]])
            _sum += item[1] * _inner_sum
        return _sum


'''


