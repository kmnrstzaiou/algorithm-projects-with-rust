use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 27;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [['.'; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    let success = place_queens_4(&mut board, 0);
    let duration = start.elapsed();

    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
        dump_board(&board);
    } else {
        println!("Could not find a tour.");
    }
}

fn place_queens_4(board: &mut [[char; NUM_COLS]; NUM_ROWS], c: i32) -> bool {
    if c == INUM_ROWS {
        return board_is_legal(board);
    }

    if !board_is_legal(board) {
        return false;
    }

    for r in 0..NUM_ROWS {
        board[r][c as usize] = 'Q';
        if place_queens_4(board, c + 1) {
            return true;
        }
        board[r][c as usize] = '.';
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
    let mut has_queen = false;
    let mut r = r0;
    let mut c = c0;
    while 0 <= r && r < INUM_ROWS && 0 <= c && c < INUM_COLS {
        if board[r as usize][c as usize] == 'Q' {
            if has_queen {
                return false;
            }
            has_queen = true;
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
                for (dr, dc) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1)] {
                    if !series_is_legal(board, r, c, dr, dc) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn dump_board(board: &[[char; NUM_COLS]; NUM_ROWS]) {
    for row in board {
        for col in row {
            print!("{col} ");
        }
        println!();
    }
}
