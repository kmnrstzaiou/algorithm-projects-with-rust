use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 20;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [['.'; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    let success = place_queens_1(&mut board, 0);
    let duration = start.elapsed();

    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
        dump_board(&board);
    } else {
        println!("Could not find a tour.");
    }

    let start = Instant::now();
    let success = place_queens_2(&mut board);
    let duration = start.elapsed();

    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
        dump_board(&board);
    } else {
        println!("Could not find a tour.");
    }
}

fn place_queens_1(board: &mut [[char; NUM_COLS]; NUM_ROWS], row: usize) -> bool {
    if row >= NUM_ROWS {
        return board_is_a_solution(board, row);
    }

    for c in 0..NUM_COLS {
        if board[row][c] == '.' {
            board[row][c] = 'Q';
            if board_is_legal(board) && place_queens_1(board, row + 1) {
                return true;
            }
            board[row][c] = '.';
        }
    }
    false
}

// Return true if this series of squares contains at most one queen.
fn series_is_legal(
    board: &[[char; NUM_COLS]; NUM_ROWS],
    r0: i32,
    c0: i32,
    dr: i32,
    dc: i32,
) -> bool {
    let mut num_queens = 0;
    let mut r = r0;
    let mut c = c0;
    while 0 <= r && r < INUM_ROWS && 0 <= c && c < INUM_COLS {
        if board[r as usize][c as usize] == 'Q' {
            num_queens += 1;
            if num_queens > 1 {
                return false;
            }
        }
        r += dr;
        c += dc;
    }
    true
}

// Return true if the board is legal.
fn board_is_legal(board: &[[char; NUM_COLS]; NUM_ROWS]) -> bool {
    for r in 0..INUM_ROWS {
        for c in 0..INUM_COLS {
            if board[r as usize][c as usize] == 'Q' {
                for (dr, dc) in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    if !series_is_legal(board, r, c, dr, dc) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

// Return true if the board is legal and a solution.
fn board_is_a_solution(board: &[[char; NUM_COLS]; NUM_ROWS], num_rows: usize) -> bool {
    if num_rows != NUM_ROWS {
        return false;
    }
    board_is_legal(board)
}

fn place_queens_2(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    let mut num_attacking = [[0; NUM_COLS]; NUM_ROWS];
    do_place_queens(board, 0, 0, 0, &mut num_attacking)
}

fn do_place_queens(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    num_placed: usize,
    row: usize,
    col: usize,
    num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS],
) -> bool {
    if num_placed == NUM_ROWS {
        return board_is_a_solution(board, num_placed);
    }

    if row >= NUM_ROWS {
        return false;
    }

    let next_r = if col + 1 == NUM_COLS { row + 1 } else { row };
    let next_c = if col + 1 == NUM_COLS { 0 } else { col + 1 };

    if do_place_queens(board, num_placed, next_r, next_c, num_attacking) {
        return true;
    }

    if num_attacking[row][col] == 0 {
        board[row][col] = 'Q';

        adjust_attack_counts(num_attacking, row, col, 1);
        if do_place_queens(board, num_placed + 1, next_r, next_c, num_attacking) {
            return true;
        }
        board[row][col] = '.';
        adjust_attack_counts(num_attacking, row, col, -1);
    }
    false
}

fn adjust_attack_counts(
    num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS],
    row: usize,
    col: usize,
    amount: i32,
) {
    (0..NUM_COLS).for_each(|c| num_attacking[row][c] += amount);

    (0..NUM_ROWS).for_each(|r| num_attacking[r][col] += amount);

    (-INUM_ROWS..INUM_ROWS).for_each(|i| {
        let r = row as i32 + i;
        if r < 0 || INUM_ROWS <= r {
            return;
        }
        let c_right = col as i32 + i;
        let c_left = col as i32 - i;
        if 0 <= c_right && c_right < INUM_COLS {
            num_attacking[r as usize][c_right as usize] += amount;
        }
        if 0 <= c_left && c_left < INUM_COLS {
            num_attacking[r as usize][c_left as usize] += amount;
        }
    });
}

fn dump_board(board: &[[char; NUM_COLS]; NUM_ROWS]) {
    for row in board {
        for col in row {
            print!("{col} ");
        }
        println!();
    }
}
