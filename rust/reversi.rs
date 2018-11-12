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

fn find_move_up(board: Board, line: usize, column: usize) -> (u8, u8) {
    let player = board[line][column];
    return (0, 0)
}
