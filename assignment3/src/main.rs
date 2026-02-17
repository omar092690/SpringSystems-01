fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let mut secret: i32 = 42;   
    let mut guess: i32;         
    let mut guesses_taken: i32 = 0;

    let guesses: [i32; 6] = [10,50,40,65,67,42];  

    let mut i: usize = 0;

    loop {
        guess = guesses[i];     
        guesses_taken += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {guess} is correct!");
            break;
        } else if result == 1 {
            println!("Guess {guess} is too high.");
        } else {
            println!("Guess {guess} is too low.");
        }

        i += 1;
    }

    println!("It took {guesses_taken} guesses.");
}
