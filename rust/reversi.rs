type Board = [[u8; 8]; 8];

fn main() {
    let player = 1;
    let mut board: Board = [[0u8; 8]; 8];
    board[3][3] = 1;
    board[3][4] = 2;
    board[4][3] = 2;
    board[4][4] = 1;
    print_board(board);
    println!("");
    print_board(solve(board, player));
}

fn pri8_board(board: Board) {
    for line in 0..board.len() {
        for column in 0..board[line].len() {
            pri8!("{} ", board[line][column])
        }
        pri8ln!("")
    }
}

fn solve(board: Board, player: u8) -> Board {
    for line in 0..board.len() {
        for column in 0..board[line].len() {
            if board[line][column] != player {
                continue
            }
            find_move_up(board, line, column);
        }
    }
    return board
}

fn find_move_up(board: Board, line: usize, column: usize, direction: (i8, i8)) -> Option<(u8, u8)> {
    let player = board[line][column];
    let mut current_pos = (line as i8, column as i8);
    while true {
        _pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
        if !is_in_bounds(board, current_pos) {
            return None;
        }
    }
    for x in line - 2..0 {
        if board[x][column] == player {
            break;
        }
        if board[x][column] == 0 {
            return Some((x as u8, column as u8));
        }
    }
    return None;
}

fn is_in_bounds(board: Board, pos: (i8, i8)) -> bool {
    return pos.0 >= 0 && pos.0 < board.len()
        && pos.1 >= 0 && pos.1 < board[0].len();
}

fn opposite_player(player: u8) -> u8 {
     match player {
        1 => return 2,
        _ => return 1
     }
}