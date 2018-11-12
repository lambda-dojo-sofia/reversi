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

fn print_board(board: Board) {
    for line in 0..board.len() {
        for column in 0..board[line].len() {
            print!("{} ", board[line][column])
        }
        println!("")
    }
}

fn solve(board: Board, player: u8) -> Board {
    let mut new_board = board;
    for line in 0..board.len() {
        for column in 0..board[line].len() {
            if board[line][column] != player {
                continue
            }
            new_board = mark_move(new_board, find_move(board, line, column, ( 1, 0)));
            new_board = mark_move(new_board, find_move(board, line, column, (-1, 0)));
            new_board = mark_move(new_board, find_move(board, line, column, (0,  1)));
            new_board = mark_move(new_board, find_move(board, line, column, (0, -1)));
        }
    }
    return new_board
}

fn find_move(board: Board, line: usize, column: usize,
             dir: (i8, i8)) -> Option<(u8, u8)>
{
    let player = board[line][column];
    let mut pos = (line as i8, column as i8);
    let mut moves = 0;
    loop {
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        if !is_in_bounds(board, pos) {
            return None;
        }
        moves = moves + 1;
        let current = board[pos.0 as usize][pos.1 as usize];
        if moves == 1 && current != opposite_player(player) {
            return None;
        }
        if current == 0 {
            return Some((pos.0 as u8, pos.1 as u8))
        }
        if current != opposite_player(player) {
            return None;
        }
    }
}

fn is_in_bounds(board: Board, pos: (i8, i8)) -> bool {
    return pos.0 >= 0 && pos.0 < board.len() as i8
        && pos.1 >= 0 && pos.1 < board[0].len() as i8;
}

fn opposite_player(player: u8) -> u8 {
     match player {
        1 => return 2,
        _ => return 1
     }
}

fn mark_move(board: Board, mv: Option<(u8, u8)>) -> Board {
    match mv {
        None => return board,
        Some((y, x)) => {
            let mut b = board;
            b[y as usize][x as usize] = 3;
            return b;
        }
    }
}