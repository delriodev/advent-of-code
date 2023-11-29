# Working with 2D Arrays

def getCoord(_row, _col):
    return terrain[_row][0][_col]

def moveRight(_col, _right):
    _col += _right
    if(_col >= nb_cols):
        _col -= nb_cols
    return _col

def moveDown(_row, _down):
    _row += _down
    return _row

def countSlopeTrees(right, down, col = 0, row = 0, nb_trees = 0):
    while(row < nb_rows-1):
        col = moveRight(col, right)
        row = moveDown(row, down)
        if(getCoord(row, col) == '#'):
            nb_trees += 1
    return nb_trees

# Program to execute

terrain = [[]]

with open('input') as input:
    terrain = [_row.split() for _row in input]

nb_rows = len(terrain)
nb_cols = len(terrain[0][0])

print(countSlopeTrees(1, 1) * countSlopeTrees(3, 1) * countSlopeTrees(5, 1) * countSlopeTrees(7, 1) * countSlopeTrees(1, 2))