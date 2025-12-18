use std::io;

fn main() {

    const BOARD_SIZE: usize = 3; // rust constants want this fancy decleration where we need to type hint.... ok!
    const EMPTY: char = 'E';
    const CROSS: char = 'X';
    const CIRCLE: char = 'O';

    let mut board: [[char; BOARD_SIZE]; BOARD_SIZE] = [[EMPTY; BOARD_SIZE]; BOARD_SIZE];

    board[0][0] = CROSS;

    print_board(board);

    println!("{}", board.len());
}

fn print_board(board: [[char; 3]; 3]) {

    for x in board.into_iter() {
        print!("{} ", x[0]);
    }

}