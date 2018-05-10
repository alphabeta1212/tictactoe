fn greeting() {
    println!("\nRust TicTacToe\n\
             --------------\n\
             A simple game written in the rust programming language.\n\
             Code is available at: https://github.com/flofriday/tictactoe")
}

fn draw(state: &[char]) {
    println!("\n");

    for i in 0..3 {
        let offset = i * 3;

        println!(
            "-------------\n\
            | {} | {} | {} |",
            state[offset],
            state[offset + 1],
            state[offset + 2]
        );
    }

    println!("-------------");
}

fn ask_user(state: &mut [char], player: char) {
    loop {
        println!("Player '{}', enter a number: ", player);

        let mut input = String::new();

        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Couldn't read line! Try again.");
            continue;
        }

        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                println!("The field number must be between 1 and 9.");
                continue;
            }

            let number = number - 1;

            if state[number] == 'X' || state[number] == 'O' {
                println!("This field is already taken by '{}'.", state[number]);
                continue;
            }

            state[number] = player;

            break;
        } else {
            println!("Only numbers are allowed.");
            continue;
        }
    }
}

fn has_won(state: &[char]) -> bool {
    for tmp in 0..3 {
        if state[tmp] == state[tmp + 3] && state[tmp] == state[tmp + 6] {
            return true;
        }

        let tmp = tmp * 3;

        if state[tmp] == state[tmp + 1] && state[tmp] == state[tmp + 2] {
            return true;
        }
    }

    if (state[0] == state[4] && state[0] == state[8])
        || (state[2] == state[4] && state[2] == state[6])
    {
        return true;
    }

    false
}

#[inline(always)]
fn is_over(state: &[char]) -> bool {
    state.iter().all(|&v| v == 'X' || v == 'O')
}

fn main() {
    let mut state = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player = 'X';

    greeting();

    loop {
        // Draw the field
        draw(&state);

        // Ask for user input
        ask_user(&mut state, player);

        // Check if a player won
        if has_won(&state) {
            draw(&state);
            println!("Player '{}' won! \\(^.^)/", player);
            break;
        }

        // Check if all fields are used
        if is_over(&state) {
            draw(&state);
            println!("All fields are used. No one won.");
            break;
        }

        // Switch player
        player = if player == 'X' { 'O' } else { 'X' }
    }
}
