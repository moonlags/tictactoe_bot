use std::io::{self, Write};

#[derive(Clone)]
struct TicTacToeBoard {
    board: [char; 9],
    player_char: char,
    bot_char: char,
    moves: [fn(&mut TicTacToeBoard); 2],
}

impl TicTacToeBoard {
    fn print(&self) {
        println!("-----------------------------");
        for (i, tile) in self.board.iter().enumerate() {
            if i % 3 == 0 {
                println!();
            }
            print!("|{}|", *tile);
        }
        println!();
    }

    fn evaluate(&self) -> i32 {
        const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8], // Rows
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8], // Columns
            [0, 4, 8],
            [2, 4, 6], // Diagonals
        ];

        for combination in WINNING_COMBINATIONS.iter() {
            let [a, b, c] = *combination;
            if self.board[a] != ' '
                && self.board[a] == self.board[b]
                && self.board[a] == self.board[c]
            {
                if self.board[a] == self.bot_char {
                    return 10;
                } else {
                    return -10;
                };
            }
        }

        let mut eval = 0;

        for (i, tile) in self.board.iter().enumerate() {
            if *tile == ' ' {
                continue;
            }

            match i {
                4 => {
                    eval += if *tile == self.player_char { -3 } else { 3 };
                }
                0 | 2 | 6 | 8 => {
                    eval += if *tile == self.player_char { -2 } else { 2 };
                }
                _ => {
                    eval += if *tile == self.player_char { -1 } else { 1 };
                }
            }
        }

        eval
    }

    fn is_moves_left(&self) -> bool {
        self.board.iter().any(|&tile| tile == ' ')
    }
}

fn main() {
    let mut board = TicTacToeBoard {
        board: [' '; 9],
        player_char: 'X',
        bot_char: 'O',
        moves: [player_move, bot_move],
    };

    println!("Welcome to the game of Tic Tac Toe!");

    print!("Do you want to make a move first (Y/n): ");
    io::stdout().flush().expect("failed to flush");

    let mut is_player_first = String::new();
    io::stdin()
        .read_line(&mut is_player_first)
        .expect("failed to readline");

    if is_player_first.trim().to_lowercase() == "n" {
        board.moves[0] = bot_move;
        board.moves[1] = player_move;
        board.bot_char = 'X';
        board.player_char = 'O';
    }

    board.print();

    let mut i: usize = 0;
    loop {
        board.moves[i](&mut board);
        board.print();

        let eval = board.evaluate();
        if eval == 10 || eval == -10 {
            println!("We have a winner!");
            break;
        } else if !board.is_moves_left() {
            println!("We have a tie!");
            break;
        }

        i = 1 - i;
    }
}

fn player_move(board: &mut TicTacToeBoard) {
    loop {
        let mut player_move = String::new();

        print!("Time for your move (1-9): ");
        io::stdout().flush().expect("failed to flush");

        io::stdin()
            .read_line(&mut player_move)
            .expect("failed to readline");

        let player_move: usize = match player_move.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if (1..=9).contains(&player_move) {
            board.board[player_move - 1] = board.player_char;
            break;
        }
    }
}

fn bot_move(board: &mut TicTacToeBoard) {
    let mut best_move: usize = 0;
    let mut best_score = i32::MIN;

    for (i, tile) in board.board.clone().iter().enumerate() {
        if *tile == ' ' {
            board.board[i] = board.bot_char;
            let move_score = minimax(board, 5, i32::MIN, i32::MAX, false);
            board.board[i] = ' ';

            if move_score > best_score {
                best_score = move_score;
                best_move = i;
            }
        }
    }

    board.board[best_move] = board.bot_char;
}

fn minimax(
    board: &mut TicTacToeBoard,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
) -> i32 {
    let evaluation = board.evaluate();

    if depth == 0 || evaluation == 10 || evaluation == -10 {
        return evaluation;
    } else if !board.is_moves_left() {
        return 0;
    }

    if maximizing_player {
        let mut max_eval = i32::MIN;
        for (i, tile) in board.board.clone().iter().enumerate() {
            if *tile == ' ' {
                board.board[i] = board.bot_char;
                let eval = minimax(board, depth - 1, alpha, beta, false);
                board.board[i] = ' ';

                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for (i, tile) in board.board.clone().iter().enumerate() {
            if *tile == ' ' {
                board.board[i] = board.player_char;
                let eval = minimax(board, depth - 1, alpha, beta, true);
                board.board[i] = ' ';

                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
        }
        min_eval
    }
}
