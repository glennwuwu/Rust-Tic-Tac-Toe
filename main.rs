use regex::Regex;
use std::io;

const BOARD_SIZE: usize = 3;
const PLAYERS: [char; 2] = ['X', 'O'];

fn main() {
    let mut gs: GameState = init_game();
    loop {
        gs = play_turn(gs);
        if has_won(&gs) {
            println!("{} has won", PLAYERS[gs.turn % PLAYERS.len()]);
            break;
        }
    }
}

struct GameState {
    board: [[TileState; BOARD_SIZE]; BOARD_SIZE],
    turn: usize,
}

#[derive(Copy, Clone)]
struct TileState {
    id: i32,
    state: char,
}

fn init_game() -> GameState {
    let mut gs: GameState = GameState {
        board: [[TileState { id: 0, state: '_' }; BOARD_SIZE]; BOARD_SIZE],
        turn: 0,
    };
    let mut tile_no: i32 = 0;
    for row in 0..BOARD_SIZE {
        for tile in 0..BOARD_SIZE {
            tile_no += 1;
            gs.board[row][tile].id = tile_no;
        }
    }
    gs
}

fn formatted_board(gs: &GameState) -> String {
    let mut board: String = String::new();
    for i in 0..BOARD_SIZE {
        let row_str: String = gs.board[i]
            .iter()
            .map(|num| format!("{} ", num.state))
            .collect::<String>();
        board = format!("{}\n{}", board, row_str);
    }
    board
}

fn update_board(gs: GameState, tile_id: i32) -> GameState {
    let mut gs: GameState = gs;
    let mut tile: i32 = 0;
    'outer: for row in 1..BOARD_SIZE + 1 {
        for col in 1..BOARD_SIZE + 1 {
            tile += 1;
            if tile == tile_id {
                if gs.board[row - 1][col - 1].state == '_' {
                    gs.board[row - 1][col - 1].state = PLAYERS[gs.turn % PLAYERS.len()];
                    gs.turn += 1;
                    break 'outer;
                }
            }
        }
    }
    gs
}

fn play_turn(gs: GameState) -> GameState {
    let mut gs = gs;
    let stdin = io::stdin();
    println!("Current Board State: {}", formatted_board(&gs));
    println!(
        "Player {} please make your move",
        PLAYERS[gs.turn % PLAYERS.len()]
    );
    loop {
        let user_move = loop {
            let mut user_input = String::new();
            stdin.read_line(&mut user_input).unwrap();
            user_input = user_input.trim().to_string();
            let re = Regex::new(r"\D+").unwrap();
            let user_input_num_only = re.replace_all(user_input.as_str(), "");
            let number: Result<i32, _> = user_input_num_only.parse();
            match number {
                Ok(parsed_number) => {
                    if parsed_number <= BOARD_SIZE as i32 * BOARD_SIZE as i32 && parsed_number >= 1
                    {
                        // println!("input: {} board: {}", parsed_number, BOARD_SIZE * BOARD_SIZE);
                        break parsed_number;
                    } else {
                        println!("Number must be in the valid range.");
                    }
                    // println!("Parsed number: {}", parsed_number);
                }
                Err(_) => {
                    println!("Failed to parse the number.");
                }
            }
            // println!("{}", user_input.parse());
        };
        // println!("{}", user_move);
        let old_turn = gs.turn;
        gs = update_board(gs, user_move);
        if old_turn == gs.turn {
            println!("You cannot input a tile which has already been taken.")
        } else {
            break;
        }
    }
    gs
}

fn has_won(gs: &GameState) -> bool {
    let board = gs.board;
    let player = PLAYERS[gs.turn % PLAYERS.len()];
    let size = board.len(); // Assuming the board is a square grid

    // Check rows
    for row in 0..size {
        let mut row_win = true;
        for col in 0..size {
            if board[row][col].state != player {
                row_win = false;
                break;
            }
        }
        if row_win {
            return true;
        }
    }

    // Check columns
    for col in 0..size {
        let mut col_win = true;
        for row in 0..size {
            if board[row][col].state != player {
                col_win = false;
                break;
            }
        }
        if col_win {
            return true;
        }
    }

    // Check diagonals
    let mut diag1_win = true;
    let mut diag2_win = true;
    for i in 0..size {
        if board[i][i].state != player {
            diag1_win = false;
        }
        if board[i][size - 1 - i].state != player {
            diag2_win = false;
        }
    }
    if diag1_win || diag2_win {
        return true;
    }

    false
}
