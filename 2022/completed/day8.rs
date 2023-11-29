use bitmaps::Bitmap;

pub fn sol1(content: &String) -> usize {
    let mut lines = content.as_str().lines();
    let n_cols = &lines.next().unwrap().chars().count() - 1;
    let n_rows = &lines.count();
    let mut total = 2 * n_cols + 2 * n_rows;

    println!("the initial total is : {}", total);

    let grid: Vec<Vec<u8>> = content
        .as_str()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| u8::try_from(char.to_digit(10).unwrap()).unwrap())
                .collect()
        })
        .collect();

    for x in 1..*n_rows {
        for y in 1..n_cols {
            print!("({},{}): {} - ", x + 1, y + 1, &grid[x][y]);

            let bitmap: Bitmap<4> = is_visible(&grid, x, y, *n_rows, n_cols);
            if bitmap.is_empty() {
                println!("is hidden!");
            } else {
                total += 1;
                print!(" is visible from");
                for x in bitmap.into_iter() {
                    match x {
                        0 => print!(" the left,"),
                        1 => print!(" above,"),
                        2 => print!(" the right,"),
                        3 => print!(" below,"),
                        _ => print!(" impossible"),
                    }
                }
                println!();
            }
        }
    }

    println!("The total is : {}", total);
    total
}

#[allow(unused_variables)]
pub fn sol2(content: &String) -> usize {
    let mut lines = content.as_str().lines();
    let n_cols = &lines.next().unwrap().chars().count() - 1;
    let n_rows = &lines.count();

    let grid: Vec<Vec<u8>> = content
        .as_str()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| u8::try_from(char.to_digit(10).unwrap()).unwrap())
                .collect()
        })
        .collect();

    let mut best_total = 0;
    for x in 1..*n_rows {
        for y in 1..n_cols {
            print!("({},{}):{} :: ", x, y, &grid[x][y]);

            let mut left_total = 1;
            let mut left = y.clone() - 1;
            // print!("left_total : {}, left : {} ", left_total, left);
            while left > 0 && &grid[x][left] < &grid[x][y] {
                left_total += 1;
                left -= 1;
                // print!(" ... Added 1, left_total : {}", left_total);
            }
            print!(" left : {} ", left_total);

            let mut up_total = 1;
            let mut up = x.clone() - 1;
            while up > 0 && &grid[up][y] < &grid[x][y] {
                up_total += 1;
                up -= 1;
            }
            print!(" up : {} ", up_total);

            let mut right_total = 1;
            let mut right = y.clone() + 1;
            while right < n_cols && &grid[x][right] < &grid[x][y] {
                right_total += 1;
                right += 1;
            }
            print!(" right : {} ", right_total);

            let mut down_total = 1;
            let mut down = x.clone() + 1;
            while down < n_cols && &grid[down][y] < &grid[x][y] {
                down_total += 1;
                down += 1;
            }
            print!("down : {} ", down_total);

            if best_total < left_total * up_total * right_total * down_total {
                best_total = left_total * up_total * right_total * down_total;
            }
            println!("best : {}", best_total);
        }
    }

    println!("Solution 2 total : {}", best_total);
    best_total
}

fn is_visible(
    grid: &Vec<Vec<u8>>,
    row: usize,
    col: usize,
    row_limit: usize,
    col_limit: usize,
) -> Bitmap<4> {
    let mut bitmap: Bitmap<4> = Bitmap::new();

    let left = &grid[row][0..col];
    let up: Vec<u8> = grid[0..row].iter().map(|row| row[col]).collect();
    let right = &grid[row][col + 1..=col_limit];
    let down: Vec<u8> = grid[row + 1..=row_limit]
        .iter()
        .map(|row| row[col])
        .collect();

    if left.into_iter().all(|tree| grid[row][col] > *tree) {
        bitmap.set(0, true);
    }

    if up.into_iter().all(|tree| grid[row][col] > tree) {
        bitmap.set(1, true);
    }

    if right.into_iter().all(|tree| grid[row][col] > *tree) {
        bitmap.set(2, true);
    }

    if down.into_iter().all(|tree| grid[row][col] > tree) {
        bitmap.set(3, true);
    }

    bitmap
}
