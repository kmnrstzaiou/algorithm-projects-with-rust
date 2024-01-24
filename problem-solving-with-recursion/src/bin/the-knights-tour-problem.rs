use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = true;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

fn main() {
    // Initialize the vector of move offsets.
    let mut offsets = [
        [-2, -1],
        [-1, -2],
        [2, -1],
        [1, -2],
        [-2, 1],
        [-1, 2],
        [2, 1],
        [1, 2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, &mut offsets, 0, 0, 1);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&board);
}

fn find_tour(
    board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &mut [[i32; 2]; 8],
    cur_row: i32,
    cur_col: i32,
    num_visited: i32,
) -> bool {
    if num_visited == INUM_ROWS * INUM_COLS {
        return if REQUIRE_CLOSED_TOUR {
            for offset in offsets {
                let next_row = cur_row + offset[0];
                let next_col = cur_col + offset[1];
                if (0..INUM_ROWS).contains(&next_row)
                    && (0..INUM_COLS).contains(&next_col)
                    && board[next_row as usize][next_col as usize] == 0
                {
                    return true;
                }
            }
            false
        } else {
            true
        };
    }

    for offset in *offsets {
        let next_row = cur_row + offset[0];
        let next_col = cur_col + offset[1];
        if (0..INUM_ROWS).contains(&next_row)
            && (0..INUM_COLS).contains(&next_col)
            && board[next_row as usize][next_col as usize] == UNVISITED
        {
            board[next_row as usize][next_col as usize] = num_visited;
            if find_tour(board, offsets, next_row, next_col, num_visited + 1) {
                return true;
            }
            board[next_row as usize][next_col as usize] = UNVISITED;
        }
    }
    false
}

fn dump_board(board: &[[i32; NUM_COLS]; NUM_ROWS]) {
    for row in board {
        for col in row {
            print!("{col:02} ");
        }
        println!();
    }
}
