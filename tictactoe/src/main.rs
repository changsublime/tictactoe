use std::io;
use indoc::printdoc;

fn display(&[a, b, c ,d, e, f, g, h, i]: &[&str; 9]){
    printdoc! {"
        _________________
          {}  |  {}  |  {} 
        _____|_____|_____
          {}  |  {}  |  {} 
        _____|_____|_____
          {}  |  {}  |  {} 
        _____|_____|_____
    ", a, b, c, d, e, f, g, h, i};
}

fn check_valid_move<'a>(board: &[&'a str; 9], m: &'a usize) -> &'a bool{
    if board[*m] == " " {
        return &true;
    } else {
        return &false;
    }
}

fn check_winner<'a>(board: &[&'a str; 9]) -> &'a bool{
    let win_states: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6]
    ];
    for [x, y, z] in &win_states {
        if board[*x] != " " && board[*x] == board[*y] && board[*y] == board[*z] {
            return &true;
        };
    };
    return &false;
}

fn check_tie<'a>(board: &[&'a str; 9]) -> &'a bool{
    for c in board {
        if *c == " " {
            return &false;
        }
    };
    return &true;
}

fn tictactoe(){
    println!("X goes first. Have fun!");

    let mut board = [" ", " ", " ", " ", " ", " ", " ", " ", " "];
    let mut player = "X"; // start with X's turn

    loop {
        println!("\nPlayer {}, please make your move (0-8): ", player);
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let position: usize = line.trim().parse().expect("Input not an integer");

        match position{
            0..=8 => (),
            _ => {
                println!("Invalid square. Try again!");
                continue;
            }
        };

        if !check_valid_move(&board, &position){
            println!("That square is already taken. Try again!");
            continue;
        }

        board[position] = player;
        display(&board);

        // check win state
        if *check_winner(&board){
            println!("Congrats player {}, you won!\n", player);
            break
        };

        // check tie state
        if *check_tie(&board){
            println!("It's a tied game!\n");
            break
        };

        // flip player
        player = if player == "O" {"X"} else {"O"};
    }
}


fn main(){
    loop{
        printdoc!("
            Welcome to TicTacToe!
            Players can make moves by entering the number of the square they would like to play in:
            ");
        display(&["0","1","2","3","4","5","6","7","8"]);
        tictactoe();

        loop {
            println!("Would you like to play a new game? (y/n)");
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            
            match line.trim() {
                "y" => {
                    println!("Starting a new game");
                    break;
                },
                "n" => {
                    println!("Thanks for playing!");
                    return;
                },
                x => {
                    println!("{}", x);
                    continue;
                }
            };
        }
    }
 }