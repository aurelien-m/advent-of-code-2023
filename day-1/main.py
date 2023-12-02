input_list = open('input', 'r').read().split('\n') # read input file and split by new line
print(input_list) # print input list

sum = 0 # create our sum integer
# create list of integers spelled-out in english
integer_names = ['zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
for inp in input_list:
    nums = [] # craate empty list that we will use to track all integers in list
    for i, letter in enumerate(inp): # loop through all letters in input line
        for val, name in enumerate(integer_names): # loop through all spelled-out number options
            if name in inp[i:i+len(name)]: # if the current spelled-out number we are looking for is found starting at index i, add it to our nums list
                nums.append(str(val)) # append to our list of integer characters
        if ord(letter) <= 57: # if unicode value of character is <=57, we know it's an integer
            nums.append(letter) # append to our list of integer characters

    # concat the first and last characters of our list and add their integer representation to our sum
    sum += int(nums[0] + nums[-1])

    print(inp, "->", nums[0], nums[-1])

print("Total:", sum) # print solution to part 2