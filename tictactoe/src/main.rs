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
    if board[*m] == " "{
        return &true;
    } else {
        return &false;
    }
}

fn check_winner<'a>(board: &[&'a str; 9]) -> &'a bool{
    // TODO
    return &false;
}

fn tictactoe(){
    printdoc!("
        Welcome to TicTacToe!
        Players can make moves by entering the number of the square they would like to play in:
        ");
    display(&["0","1","2","3","4","5","6","7","8"]);
    println!("X goes first. Have fun!");

    let mut board = [" ", " ", " ", " ", " ", " ", " ", " ", " "];
    let mut player = "X"; // start with X's turn

    loop {
        println!("Player {}, please make your move (0-8): ", player);
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let position: usize = line.trim().parse().expect("Input not an integer");

        match position{
            0..=8 => (),
            _ => continue,
        };

        if !check_valid_move(&board, &position){
            println!("That square is already taken. Try again!");
            continue;
        }

        board[position] = player;
        display(&board);
        
        // flip player
        player = if player == "O" {"X"} else {"O"};

        // check game end state
        let winner = check_winner(&board);
        match winner{
            true => break,
            _ => continue,
        };
    }
}


fn main(){
    loop{
        tictactoe();
    }
 }