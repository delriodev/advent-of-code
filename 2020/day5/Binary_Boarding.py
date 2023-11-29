# Working with binary and sets
# F = L = 0, B = R = 1

all_seats = set()
seats = set()
seats_ids = []

def getSeat(code):
    row_bits = ''
    col_bits = ''
    for l in code:
        if  (l == 'F'):
            row_bits += '0'
        elif(l == 'B'):
            row_bits += '1'
        elif(l == 'L'):
            col_bits += '0'
        elif(l == 'R'):
            col_bits += '1'

    return int(row_bits, 2), int(col_bits, 2)

def getSeatId(seat):
    return ((seat[0] * 8) + seat[1])

def getMax(seats):
    _max = 0
    for seat in seats:
        if( getSeatId(seat) > _max ):
            _max = getSeatId(seat)
    return(_max)

# Initalise all possible sets
for r in range(0, 128):
    for c in range(0, 8):
        t = r, c
        all_seats.add(t)

# Read the input file
with open('input') as input:
    lines = input.read().splitlines()

# Parse input seats
for line in lines:
    _seat = getSeat(line)
    seats.add(_seat)
    seats_ids.append(getSeatId(_seat))

# Answear to part 1
print(getMax(seats))

for seat in all_seats.difference(seats):
    if(((getSeatId(seat) - 1) in seats_ids) and ((getSeatId(seat) + 1) in seats_ids)):
        print(getSeatId(seat))

