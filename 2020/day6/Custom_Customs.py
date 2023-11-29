# Working with sets

from functools import reduce

# Part 1
def count_any_answear(_lines):

    _count = 0
    _answers = []
    _answer = set()

    for line in _lines:
        if(line != ''):
            for char in line:
                _answer.add(char)
        else:
            _count += len(_answer)
            _answers.append(_answer)
            _answer = set()
    return _count

# Part 2:
def count_all_answers(_lines):
    
    count = 0
    flight_answers = []
    group_answers = []
    person_answers = set()

    for _line in _lines:
        if(_line != ''):
            # Create person answer
            for _char in _line:
                person_answers.add(_char)
            # Add answer to group and clear answer
            group_answers.append(person_answers)
            person_answers = set()
        else:
            # get intersection :
            group_common_answers = reduce(set.intersection, group_answers)
            count += len(group_common_answers)
            # Add intersection
            flight_answers.append(group_common_answers)
            # Add group answer to flight answers and clear group_answers
            group_answers = []
            
    return count

    
with open('input') as input:
    lines = input.read().splitlines()
    lines.append('')

# Part 1 result
print(count_any_answear(lines))

# Part 2 result
print(count_all_answers(lines))