fn main() {

    const BOARD_SIZE: usize = 9; // rust constants want this fancy decleration where we need to type hint.... ok!
    const EMPTY: char = ' ';
    const CROSS: char = 'X';
    const CIRCLE: char = 'O';

    let mut board: [char; BOARD_SIZE] = [EMPTY; BOARD_SIZE];

    board[0] = CROSS;
    board[3] = CIRCLE;

    print_board(board);

    println!("{}", board.len());

}

fn print_board(board: [char; 9]) {

    for x in board.into_iter() {
        print!("{} ", x);
    }

}