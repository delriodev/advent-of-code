# PART ONE
# Find a pair that sum to a given value
# Divide the array into two sections
# O(n2 + n)

def find2numbers(arr, target):

    pivot = get_first_index_bigger_than(arr, target/2)

    for x in arr[:pivot]:
        for y in arr[pivot:]:
            if(x + y == 2020):
                return x*y

def get_first_index_bigger_than(arr, target):
    for index, val in enumerate(arr):
        if(val > target):
            return index
    return -1

# Find a triplet that sum to a given value
# Two pointer technique
# O(n2)

def find3Numbers(arr, target):
    for i in range(0, len(arr) - 2):
        l = i + 1
        r = len(arr)-1
        while(l < r):

            summ = arr[i] + arr[l] + arr[r]

            if( summ == TARGET ):
                return arr[i] * arr[l] * arr[r]
            elif( summ < TARGET ):
                l += 1
            else: # summ > TARGET
                r -= 1
    return -1

numbers = []
TARGET = 2020

# 1. Parse the data into an int array
with open('input') as input:
    numbers = [int(val) for val in input] 
numbers.sort()

# PART ONE
print(find2numbers(numbers, TARGET))

# PART TWO
print(find3Numbers(numbers, TARGET))