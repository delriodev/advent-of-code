# Working with classes, default dictionaries

import re, operator

class Command:
    def __init__(self, code, oper, val):
        self.code = code
        self.oper = oper
        self.val = val

with open('input') as input:
    lines = input.read().splitlines()

comms = {}
for i, l in enumerate(lines):
    code = re.match(r'[a-z]+', l)[0]
    val = int(re.search(r'\d+', l)[0])
    if(re.search(r'(-|\+)', l)[0] == '+'):
        oper = operator.add
    else:
        oper = operator.sub
    comms[i] = Command(code, oper, val)

def execute(i, acc, path):
    if(i == 647):
        print(acc)
        return acc, True
    elif(i not in path):
        path.append(i)
        c = comms[i]
        if(c.code == 'acc'):
            acc = c.oper(acc, c.val)
            i = operator.add(i, 1)
        elif(c.code == 'nop'):
            i = operator.add(i, 1)
        else: # code is jmp
            i = c.oper(i, c.val)
        execute(i, acc, path)
    else:
        # print(acc)
        return acc, False

def change(comm):
    c = comms[comm]
    if(c.code == 'jmp'):
        c.code = 'nop'
    elif(c.code == 'nop'):
        c.code = 'jmp'

def part1():
    execute(0, 0, [])

def part2():
    for comm in comms:
        change(comm)
        execute(0, 0, [])
        change(comm)

part1()
part2()

'''
Solution to another problem: Find the fastest way to the end if you can interchange jmp and nop commands.
def part2():
    _path = []
    _acc = 0
    _i = 0
    def execute(i, acc, path):
        print(i)
        if(i == 647):
            print('True')
            print(acc)
            return True
        elif(i not in path):
            path.append(i)
            print(path)
            c = comms[i]
            if(c.code == 'acc'):
                i = operator.add(i, 1)
                acc = c.oper(acc, c.val)
                return execute(i, acc, path)
            else:
                path1 = list(path)
                path2 = list(path)
                acc1 = int(acc)
                acc2 = int(acc)
                if(c.code == 'nop'):
                    # Execute as jmp
                    i1 = c.oper(i, c.val)
                    if(execute(i1, acc1, path1)):
                        return True
                    # Execute as nop
                    i2 = operator.add(i, 1)
                    return execute(i2, acc2, path2)
                else: 
                    # Execute as nop
                    i1 = operator.add(i, 1)
                    if(execute(i1, acc1, path1)):
                        return True
                    # Execute as jmp
                    i2 = c.oper(i, c.val)
                    return execute(i2, acc2, path2)
        else:
            return False
    execute(_i, _acc, _path)

'''