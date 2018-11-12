use std::io::{self, BufRead};

type Board = [[u8; 8]; 8];

fn main() {
    let (board, player) = read_board();
    let new_board = solve(board, player);
    print_board(new_board);
    println!("{}", format_whatever(player));
}

fn read_board() -> (Board, u8) {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let mut board: Board = [[0u8; 8]; 8];
    for line in 0..board.len() {
        let input = iter.next().unwrap().unwrap();
        let mut chars = input.chars();
        for column in 0..board[line].len() {
            board[line][column] = parse_char(chars.next().unwrap());
        }
    }
    let player_vec: Vec<char> =
        iter.next().unwrap().unwrap().chars().collect();
    let player = parse_char(player_vec[0]);
    return (board, player)
}

fn parse_char(ch: char) -> u8 {
    return match ch {
        'B' => 1,
        'W' => 2,
        _   => 0,
    }
}

fn print_board(board: Board) {
    for line in 0..board.len() {
        for column in 0..board[line].len() {
            print!("{} ", format_whatever(board[line][column]))
        }
        println!("")
    }
}

fn format_whatever(x: u8) -> char {
    return match x {
        1 => 'B',
        2 => 'W',
        3 => '0',
        _ => '.',
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
            new_board = mark_move(new_board, find_move(board, line, column, ( 1,  1)));
            new_board = mark_move(new_board, find_move(board, line, column, (-1, -1)));
            new_board = mark_move(new_board, find_move(board, line, column, (-1,  1)));
            new_board = mark_move(new_board, find_move(board, line, column, ( 1, -1)));
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
     return match player {
        1 => 2,
        _ => 1
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