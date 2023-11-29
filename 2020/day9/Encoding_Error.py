with open('input') as input:
    nums = [int(val) for val in input] 

def findPair(c, t):
    for i in range(len(c)):
        for j in range(i, len(c)):
            if c[i] != c[j]:
                if c[i] + c[j] == t:
                    return True
    return False

tc1 = [1, 2, 3, 4, 5]
tc2 = [7, 2, 13, 4, 5]
def test_findPair():
    assert findPair(tc1, 7) == True
    assert findPair(tc2, 100) == False
test_findPair()

def part1():
    for i in range(25, len(nums)):
        pre = []
        for p in range(i - 25, i):
            if nums[p] not in pre:
                pre.append(nums[p])
        if(not findPair(pre, nums[i])):
            print(nums[i])
            return i
            # return nums[i]

def part2():
    piv = part1()
    target = nums[piv]
    nums2 = nums[:piv]
    for i in range(len(nums2)-1):
        cont = []
        cont.append(nums2[i])
        j = i + 1
        while(j < len(nums2) and sum(cont) < target):
            cont.append(nums[j])
            j += 1

            if(sum(cont) == target):
                res = min(cont) + max(cont)
                print(res)
                return res

part2()






