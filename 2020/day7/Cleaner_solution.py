# Got this from the internet and laern a couple of things.

import re
from collections import defaultdict

with open('input') as input:
    lines = input.read().splitlines()

bags = defaultdict(dict)
for l in lines:
    bag = re.match(r'(.*) bags contain', l).groups()[0]
    for count, b in re.findall(r'(\d+) (\w+ \w+) bag', l):
        bags[bag][b] = int(count)

def part1():
    answer = set()
    def search(color):
        for b in bags:
            if color in bags[b]:
                answer.add(b)
                search(b)
    search('shiny gold')
    return len(answer)

def part2():
    def search(bag):
        count = 1
        for s in bags[bag]:
            multiplier = bags[bag][s]
            count += multiplier * search(s)
        return count
    return search('shiny gold' ) - 1  # Rm one for shiny gold itself

print(part1())
print(part2())