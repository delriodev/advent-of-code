# Sort la liste
with open('input') as input:
    nums = [int(val) for val in input] 
nums.sort()

def part1():
    jolt1 = 1
    jolt3 = 1
    for i in range(len(nums)-1):
        delta = nums[i+1] - nums[i]
        if(delta == 1):
            jolt1 += 1
        elif(delta == 3):
            jolt3 += 1

    print(jolt1 * jolt3)   

def part2():
    nums.insert(0, 0)
    nums.append(max(nums)+3)
    nums.reverse()
    
    cw = {}

    def execute(l):

        def sum_children(i, n):
            s = 0
            for j in range(i-3, i):
                if(j >= 0):         # To prevent out of bound exception
                     if(nums[j] in range(nums[i] + 1, nums[i] + 4)):        # Inside next 3 values
                         s += cw[j]
            return s

        for i, n in enumerate(l):
            if(i == 0):
                cw[i] = 1
            else:
                cw[i] = sum_children(i, n)

    execute(nums)
    print(cw)

# part1()
part2()





'''
Useless but never delete code

    print(nums)
    while(nums[i] != max(nums)):
        print('i: ' + str(i))
        print('nums: ' + str(nums[i]))
        for j in range(i + 1, i + 4):
            print(j)
            if (nums[j] in range(nums[i] + 1, nums[i] + 4)):
                print('possibilitees: ' + str(nums[j]))
        i += 1
   
        # Calculer le nombre de possibilites ahead p
        # pour chaque possibilite p
            # Creer un nouveau chemin 


    
        if(nums[i] == max(nums)):
            cw

        else:
            cw = []
            for j in range(i + 1, i + 4):
                if(j < len(nums)):
                    if (nums[j] in range(nums[i] + 1, nums[i] + 4)):
                        cw.append(execute(j))
            return sum(cw)
'''